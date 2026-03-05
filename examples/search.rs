use std::env;

use dotenvy::dotenv;
use teraflopai::{TeraflopAI, TeraflopError};

#[tokio::main] 
async fn main() -> Result<(), TeraflopError> {
    dotenv().ok();

    let api_key = env::var("TERAFLOPAI_API_KEY").expect("TERAFLOP_API_KEY missing");
    let url = "https://api.caselaw.teraflopai.com/v1/search/free";

    let api = TeraflopAI::new(api_key, url)?;
    let resp = api.search("City of Houma").await?;
    println!("{resp}");

    Ok(())
}