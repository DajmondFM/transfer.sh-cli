use std::env;
use reqwest::Client;
use std::io::Read;

#[tokio::main]
async fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() > 1 {
    let var = &args[1];

      if var == "-h" || var == "--help" {
          help();
      } else if var == "-del" || var == "--delete" {
          if args.len() > 2 {
              if let Err(err) = delete(&args[2]).await {
                  eprintln!("Error: {}", err);
              }
          } else {
              println!("Missing delete link argument.");
          }
      } else {
          if let Err(err) = upload(&var).await {
              eprintln!("Error: {}", err);
          }
      }
  } else {
      println!("Missing argument.");
      println!("Use -h for help.");
  }
}

fn help() {
    println!("Unofficial Transfer.sh CLI\n");
    println!("Usage: transfer.exe [OPTIONS]\n");
    print!("Options:
  -h, --help                             Print help
  -del, --delete <delete-link>           Delete file");
}

async fn upload(file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("https://transfer.sh/{file}");
    let file_path = file;
    let client = Client::new();
    let mut file = std::fs::File::open(file_path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    let response = client.put(&url)
        .body(contents)
        .send()
        .await?;

    let delete_link = response.headers()
        .get("x-url-delete")
        .and_then(|value| value.to_str().ok())
        .unwrap_or("Unknown");
    println!("Delete-link: {}", delete_link);

    println!("{}", response.text().await?);

    Ok(())
}

async fn delete(link: &str) -> Result<(), Box<dyn std::error::Error>> {
  let client = Client::new();
  let response = client.delete(link).send().await?;

  println!("Status: {}", response.status());

  Ok(())
}