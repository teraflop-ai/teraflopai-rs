use std::env;

use dotenvy::dotenv;
use teraflopai::{TeraflopAI, TeraflopError};

#[tokio::main] 
async fn main() -> Result<(), TeraflopError> {
    dotenv().ok();

    let api_key = env::var("TERAFLOPAI_API_KEY").expect("TERAFLOPAI_API_KEY missing");
    let url = "https://api.teraflopai.com/v1/embeddings/free";
    let model="concept-embedding-legal-nano";

    let api = TeraflopAI::new(api_key, url)?;
    let resp = api.embeddings("City of Houma", model).await?;
    println!("{resp}");

    Ok(())
}