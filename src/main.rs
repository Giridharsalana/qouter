use std::error::Error;
use reqwest::StatusCode;
use serde_derive::{Serialize, Deserialize};
use clap::{Command};

#[derive(Debug, Deserialize, Serialize)]
struct Quote {
    content: String,
    author: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _matches = Command::new("qoute")
        .version("1.0")
        .author("Giridhar Salana <giridharsalana@gmail.com>")
        .about("Fetches a random quote from the qoutable.io")
        .get_matches();

    let base_url = "https://api.quotable.io/random";

    let response = reqwest::get(base_url).await?;

    match response.status() {
        StatusCode::OK => {
            // Deserialize the JSON response into the Quote struct
            let quote: Quote = response.json().await?;
            println!("{} -- {}", quote.content, quote.author);
            // println!("Author: {}", quote.author);
        }
        _ => {
            println!("Failed to fetch a random quote. HTTP Status: {:?}", response.status());
        }
    }

    Ok(())
}