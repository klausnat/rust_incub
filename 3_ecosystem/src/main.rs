use clap::Parser;
use image::codecs::jpeg::JpegEncoder; // For direct quality control
use image::ImageReader;
use num_cpus;
//use core::num;
//use reqwest;
//use std::fs;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use tokio;

/// Run the application (examples):
///
/// cargo run links.txt
/// cargo run links.txt --max-images=3
///
/// max_images = how much images are processed at the same time
/// output_directory = output directory to store processed images in
#[derive(Parser)]
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
    let mut filename = url.clone();
    // if url is web link - create appropriate filename
    if url.starts_with("http") || url.starts_with("www.") {
        if let Some(res) = url.split('/').last() {
            filename = res.to_string();
        } else {
            filename = backup_name.to_string();
        }
    } else {
        filename = url
    }

    filename
}

async fn compress_imgs(
    urls: Vec<String>,
    options: &Args,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut backup_name = 0;
    let mut handles = vec![];
    for url in urls {
        backup_name += 1;
        let options = options.clone();
        let output_dir_clone = options.output_directory.clone();
        let quality = options.quality;
        
        let handle = if url.starts_with("http") || url.starts_with("www.") {
            tokio::spawn(async move {
                let filename = create_filename(url, backup_name);
            })
            // @TODO =====================================================
            // @TODO do via reqwest::get() only works with HTTP/HTTPS URLs
            //       example: https://kusoksala.ru/images/logo.jpg
            // @TODO =====================================================
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

                // Create the output file

                let out_fl = File::create(output_dir_clone.to_owned() + &url);
                let output_file = match out_fl {
                    Ok(res) => res,
                    Err(e) => {
                        println!("couldn't create output file, due to error: {}", e);
                        return;
                    }
                };
                let mut writer = BufWriter::new(output_file);

                // Create an encoder with a specific quality (1-100, lower means more compression)
                let encoder = JpegEncoder::new_with_quality(&mut writer, quality);
                img.write_with_encoder(encoder);
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
// 1. make it work assyncronously with Tokio (with simple links, not urls)
// 2. make it work with urls too, using reqwest
/*
// Async version (if you prefer)
#[tokio::main]
async fn download_and_decode_image_async(url: &str) -> Result<image::DynamicImage, Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;
    let bytes = response.bytes().await?;

    let img = ImageReader::new(Cursor::new(bytes))
        .with_guessed_format()?
        .decode()?;

    Ok(img)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://kusoksala.ru/images/logo.jpg";
    let img = download_and_decode_image(url)?;

    println!("Image dimensions: {}x{}", img.width(), img.height());

    // Save the image locally if needed
    img.save("downloaded_image.jpg")?;

    Ok(())
}
     */

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    println!("Processing {} with {} threads", args.file, args.max_images);

    let list_links = get_links(&args.file.as_str())?;

    for i in &list_links {
        println!("link to image: {}", i)
    }

    compress_imgs(list_links, &args).await?;

    Ok(())
}
