use clap::Parser;
use image::codecs::jpeg::JpegEncoder; // For direct quality control
use image::ImageReader;
use num_cpus;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Cursor;
use tokio;

/// Run the application (examples):
///
/// cargo run links.txt
/// cargo run links.txt --max-images=3
///
/// max_images = how much images are processed at the same time
/// output_directory = output directory to store processed images in
#[derive(Parser, Clone)]
struct Args {
    #[arg(long, default_value_t = num_cpus::get())]
    max_images: usize,
    file: String,
    #[arg(long, default_value_t = String::from("compressed/"))]
    output_directory: String,
    #[arg(long, default_value_t = 20)]
    quality: u8,
}

//@TODO We will need it if not reading from file via Args parsing...
// impl Default for Args {
//     fn default() -> Self {
//         Self {
//             max_images: num_cpus::get(),
//             file: String::from(""),
//             output_directory: String::from("compressed/"),
//             quality: 80,
//         }
//     }
// }

fn get_links(file: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let f = File::open(file)?;
    let buf = BufReader::new(f);
    let lines: Vec<String> = buf
        .lines()
        .map(|x| x.unwrap().trim().to_string())
        .filter(|x| !x.is_empty() && (x.ends_with(".jpg") || x.ends_with(".jpeg")))
        .collect();
    Ok(lines)
}

fn create_filename(url: String, backup_name: i32) -> String {
    let filename;
    // if url is web link - create appropriate filename
    if url.starts_with("http") || url.starts_with("www.") {
        if let Some(res) = url.split('/').last() {
            filename = res.to_string();
        } else {
            filename = backup_name.to_string();
        }
    } else {
        filename = url.to_string()
    }

    filename
}

async fn download_and_decode_image_from_url(
    url: &str,
) -> Result<image::DynamicImage, Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;
    let bytes = response.bytes().await?;

    let img = ImageReader::new(Cursor::new(bytes))
        .with_guessed_format()?
        .decode()?;

    Ok(img)
}

async fn create_output_file_and_encoder(
    filename: String,
    options: Args,
) -> Result<(JpegEncoder<BufWriter<File>>), Box<dyn std::error::Error>> {
    let out_fl = File::create(options.output_directory.to_owned() + filename.as_str())?;
    let writer = BufWriter::new(out_fl);
    let encoder = JpegEncoder::new_with_quality(writer, options.quality);
    Ok(encoder)
}

async fn compress_imgs(urls: Vec<String>, options: Args) -> Result<(), Box<dyn std::error::Error>> {
    let mut backup_name = 0;
    let mut handles = vec![];
    for url in urls {
        backup_name += 1;
        let options = options.clone();
        let filename = create_filename(url.clone(), backup_name);

        let handle = if url.starts_with("http") || url.starts_with("www.") {
            // we are working with URL, will use reqwest crate
            tokio::spawn(async move {
                let img = match download_and_decode_image_from_url(url.as_str()).await {
                    Ok(img) => img,
                    Err(e) => {
                        println!("couldn't download img: {}", e);
                        return;
                    }
                };

                if let Ok(encoder) = create_output_file_and_encoder(filename, options).await {
                    img.write_with_encoder(encoder).unwrap()
                } else {
                    println!("couldn't create output file and encoder for URL: {}", &url)
                }
            })
        } else {
            // do via ImageReader::open() - works with local file paths
            tokio::spawn(async move {
                // Open and decode the image
                let img = if let Ok(image_reader) = ImageReader::open(&url) {
                    match image_reader.decode() {
                        Ok(decoded_img) => decoded_img,
                        Err(e) => {
                            println!("Couldn't decode image from {}: {}", url, e);
                            return; // @TODO or continue?
                        }
                    }
                } else {
                    println!("Couldn't open image from {}", url);
                    return; // or continue/break depending on your context
                };

                if let Ok(encoder) = create_output_file_and_encoder(filename, options).await {
                    img.write_with_encoder(encoder).unwrap()
                } else {
                    println!(
                        "couldn't create output file and encoder for LOCAL PATH: {}",
                        &url
                    )
                }
            })
        };
        handles.push(handle);
    }

    for handle in handles {
        handle.await?;
    }
    Ok(())
}

// @TODO
// 1. сделать так, чтобы max_images использовался
// 2.

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    println!("Processing {} with {} threads", args.file, args.max_images);

    let list_links = get_links(&args.file.as_str())?;

    for i in &list_links {
        println!("link to image: {}", i)
    }

    compress_imgs(list_links, args).await?;

    Ok(())
}
