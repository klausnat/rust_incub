// Implement an async-driven CLI tool, which downloads specified web pages:
// cargo run -p step_3_11 -- [--max-threads=<number>] <file>
// It must read a list of links from the <file>, and then concurrently download
// a content of each link into a separate .html file (named by a link).
// --max-threads argument must control the maximum number of simultaneously
// running threads in the program (should default to CPUs number).
use clap::Parser;
use num_cpus;
use reqwest;
use std::fs;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use tokio;

/// Run the application (examples):
///
/// cargo run links.txt
/// cargo run links.txt --max-threads=3

#[derive(Parser)]
struct Args {
    #[arg(long, default_value_t = num_cpus::get())]
    max_threads: usize,
    file: String,
}

fn get_links(file: String) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let f = File::open(file)?;
    let buf = BufReader::new(f);
    let lines: Vec<String> = buf
        .lines()
        .map(|x| x.unwrap().trim().to_string())
        .filter(|x| !x.is_empty() && (x.starts_with("https://") || x.starts_with("http://")))
        .collect();
    Ok(lines)
}

fn create_filename(name: &str) -> Option<String> {
    let cur = name.replace("http://", "").replace("https://", "").replace("www", "");
    if let Some((before_dot, _after_dot)) = cur.split_once('.') {
        Some(before_dot.to_string() + ".html")
    } else {
        None
    }
}

async fn create_html(urls: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let mut handles = vec![];
    let client = reqwest::Client::new();
    for url in urls {
        let source = url.clone();
        let client = client.clone();
        let handle = tokio::spawn(async move {
            match client.get(url).send().await {
                Ok(response) => match response.text().await {
                    Ok(content) => {
                        if let Some(filename) = create_filename(&source) {
                            if let Err(e) = fs::write(create_filename(&source).unwrap(), content) {
                                eprintln!("failed to write {}, {}", &source, e)
                            } else {
                                println!("Downloaded: {} -> {}", source, filename)
                            }
                        } else {
                            eprintln!("failed to create filename from {}", &source)
                        }
                    }
                    Err(e) => {
                        eprintln!("failed to read text from {}: {}", source, e)
                    }
                },
                Err(e) => {
                    eprintln!("failed to download {}: {}", source, e)
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await?;
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    println!("Processing {} with {} threads", args.file, args.max_threads);

    let list_links = get_links(args.file)?;
    create_html(list_links).await?;

    Ok(())
}
