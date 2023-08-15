// use std::fmt::format;
// use std::fs::File;
use std::io::Read;

// use reqwest::Client;

// async fn upload_file() -> Result<(), Box<dyn std::error::Error>> {
//   let url = "https://transfer.sh/okok.txt";
//   let mut file = File::open("./plik.txt")?;
//   let mut contents = Vec::new();
//   file.read_to_end(&mut contents)?;

//   let client = Client::new();
//   let response = client.put(url).body(contents).send().await?;
//   println!("{}", response.text().await?);

//   Ok(())
// }

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//   tokio::runtime::Runtime::new()?.block_on(upload_file())?;

//   Ok(())
// }


use std::env;
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let var = &args[1];
    let url = &format!("https://transfer.sh/{var}");
    let file_path = var;

    let client = Client::new();
    let mut file = std::fs::File::open(file_path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    let response = client.put(url)
        .body(contents)
        .send()
        .await?;

        
    let content_type = response.headers()
        .get("x-url-delete")
        .and_then(|value| value.to_str().ok())
        .unwrap_or("Unknown");
    println!("Delete-link: {}", content_type);

    // Access other headers as needed

    println!("{}", response.text().await?);

    Ok(())
}
