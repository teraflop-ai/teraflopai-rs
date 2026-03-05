# Installation

```bash
cargo add teraflopai
```

## Usage

```bash
export TERAFLOPAI_API_KEY="your_key"
```

### Search API
```rust
use std::env;
use teraflopai::{TeraflopAI, TeraflopError};

#[tokio::main] 
async fn main() -> Result<(), TeraflopError> {
    let api_key = env::var("TERAFLOPAI_API_KEY").expect("TERAFLOPAI_API_KEY missing");
    let url = "https://api.caselaw.teraflopai.com/v1/search/free";

    let api = TeraflopAI::new(api_key, url)?;
    let resp = api.search("City of Houma").await?;
    println!("{resp}");

    Ok(())
}
```