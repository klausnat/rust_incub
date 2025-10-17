use clap::Parser;
use image::codecs::jpeg::JpegEncoder; // For direct quality control
use image::ImageReader;
use log::{debug, error, info, trace, warn};
use num_cpus;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Cursor;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Semaphore;

/// Run the application (examples):
///
/// cargo run links.txt
/// cargo run links.txt --max-images=3
///
/// max_images = how much images are processed at the same time
/// output_directory = output directory to store processed images in
#[derive(Parser, Clone)]
#[command(version, about)]
struct Args {
    #[arg(long, short, env = "IMAGES_PROCESSED_ASYNC", default_value_t = num_cpus::get())]
    max_images: usize,

    #[arg(required = false, env = "FILE_WITH_LINKS")]
    // If file was not provided - user has to use STDIN
    file: Option<String>, // Option<String>

    #[arg(long, short, env = "OUTPUT_DIRECTORY", default_value_t = String::from("compressed"))]
    output_directory: String,

    #[arg(
        short,
        long,
        env = "IMAGE_COMPRESSOR_QUALITY",  // Read from environment variable
        value_parser = clap::value_parser!(u8).range(1..=100),  // Validate range 
        default_value_t = 20,
        help = "JPEG quality (1-100) [env: IMAGE_COMPRESSOR_QUALITY]")]
    quality: u8,
}

fn get_links(file: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let start = Instant::now();
    let f = File::open(file)?;
    let buf = BufReader::new(f);
    let lines: Vec<String> = buf
        .lines()
        .map(|x| x.unwrap().trim().to_string())
        .filter(|x| !x.is_empty() && (x.ends_with(".jpg") || x.ends_with(".jpeg")))
        .collect();
    debug!(
        "Loaded {} links from {} in {:?}",
        lines.len(),
        file,
        start.elapsed()
    );
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
    trace!("Created filename '{}' for URL: {}", filename, url);
    filename
}

async fn download_and_decode_image_from_url(
    url: &str,
) -> Result<image::DynamicImage, Box<dyn std::error::Error>> {
    let start = Instant::now();
    debug!("Starting download from: {}", url);

    let response = reqwest::get(url).await?;
    let bytes = response.bytes().await?;

    debug!(
        "Downloaded {} bytes from {} in {:?}",
        bytes.len(),
        url,
        start.elapsed()
    );

    let decode_start = Instant::now();
    let img = ImageReader::new(Cursor::new(bytes))
        .with_guessed_format()?
        .decode()?;

    debug!("Decoded image from {} in {:?}", url, decode_start.elapsed());
    info!("Successfully downloaded and decoded image from: {}", url);

    Ok(img)
}

async fn create_output_file_and_encoder(
    filename: String,
    options: Args,
) -> Result<JpegEncoder<BufWriter<File>>, Box<dyn std::error::Error>> {
    let start = Instant::now();
    let out_fl = File::create(options.output_directory.to_owned() + "/" + filename.as_str())?;
    let writer = BufWriter::new(out_fl);
    let encoder = JpegEncoder::new_with_quality(writer, options.quality);
    debug!(
        "Created output file and encoder for '{}' in {:?}",
        &options.output_directory,
        start.elapsed()
    );
    Ok(encoder)
}

async fn compress_imgs(urls: Vec<String>, options: Args) -> Result<(), Box<dyn std::error::Error>> {
    let total_start = Instant::now();
    info!(
        "Starting compression of {} images with max_concurrency: {}",
        urls.len(),
        options.max_images
    );

    let mut backup_name = 0;
    let mut handles = vec![];

    for url in urls {
        backup_name += 1;
        let options = options.clone();
        let filename = create_filename(url.clone(), backup_name);

        // Create a semaphore with max_images permits
        let semaphore = Arc::new(Semaphore::new(options.max_images));

        let handle = tokio::spawn(async move {
            let task_start = Instant::now();
            // Acquire a permit - this will wait if max_images tasks are already running
            let _permit = semaphore.acquire().await.unwrap();

            debug!(
                "Started processing: {} (wait time: {:?})",
                url,
                task_start.elapsed()
            );

            if url.starts_with("http") || url.starts_with("www.") {
                debug!("Processing as URL: {}", url);
                let img = match download_and_decode_image_from_url(url.as_str()).await {
                    Ok(img) => img,
                    Err(e) => {
                        println!("couldn't download img: {}", e);
                        return;
                    }
                };
                println!("compressing image: {}", &filename);
                if let Ok(encoder) = create_output_file_and_encoder(filename, options).await {
                    let encode_start = Instant::now();
                    img.write_with_encoder(encoder).unwrap();
                    info!(
                        "Successfully compressed URL image: {} in {:?}",
                        url,
                        encode_start.elapsed()
                    );
                } else {
                    error!("Failed to encode image from URL: {}", url)
                }
            } else {
                debug!("Processing as local file: {}", url);
                let open_start = Instant::now();
                let img = if let Ok(image_reader) = ImageReader::open(&url) {
                    match image_reader.decode() {
                        Ok(decoded_img) => {
                            debug!(
                                "Opened and decoded local file {} in {:?}",
                                url,
                                open_start.elapsed()
                            );
                            decoded_img
                        }
                        Err(e) => {
                            error!("Failed to decode URL {}: {}", url, e);
                            return;
                        }
                    }
                } else {
                    println!("Couldn't open image from {}", url);
                    return; // or continue/break depending on your context
                };
                println!("compressing image: {}", &filename);
                let encode_start = Instant::now();
                if let Ok(encoder) = create_output_file_and_encoder(filename, options).await {
                    info!(
                        "Successfully compressed local image: {} in {:?}",
                        url,
                        encode_start.elapsed()
                    );
                    img.write_with_encoder(encoder).unwrap()
                } else {
                    error!(
                        "couldn't create output file and encoder for LOCAL PATH: {}",
                        &url
                    )
                }
            }
        });
        handles.push(handle);
    }

    let mut successful = 0;
    let mut failed = 0;

    for handle in handles {
        match handle.await {
            Ok(_) => successful += 1,
            Err(e) => {
                error!("Task failed: {}", e);
                failed += 1;
            }
        }
    }

    let total_time = total_start.elapsed();
    info!(
        "Completed all tasks. Successful: {}, Failed: {}, Total time: {:?}",
        successful, failed, total_time
    );

    if failed > 0 {
        warn!("Some tasks failed. Check logs for details.");
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger with RUST_LOG support
    env_logger::init();
    let args_start = Instant::now();
    let args = Args::parse();
    debug!("Parsed arguments in {:?}", args_start.elapsed());

    // Create output directory
    let dir_start = Instant::now();
    std::fs::create_dir_all(&args.output_directory)?;
    debug!("Created output directory in {:?}", dir_start.elapsed());

    info!("Processing with {} threads", args.max_images);
    info!("Processing with quality = {}", args.quality);
    info!("Saving compressed files to: {}", args.output_directory);

    let mut sources = Vec::new();

    if let Some(file) = &args.file {
        sources = get_links(&file.as_str())?;
    } else {
        // Read from STDIN
        info!("Reading image sources from STDIN (one per line, Ctrl+D to end):");
        let stdin = std::io::stdin();
        for line in stdin.lock().lines() {
            let line = line?.trim().to_string();
            if !line.is_empty() && !line.starts_with('#') {
                sources.push(line);
            }
        }
    }

    compress_imgs(sources, args).await?;

    Ok(())
}
