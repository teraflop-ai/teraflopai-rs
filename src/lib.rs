use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use serde_json::Value;

#[derive(Debug)]
pub enum TeraflopError {
    Reqwest(reqwest::Error),
    BadStatus { status: reqwest::StatusCode, body: String },
}

impl From<reqwest::Error> for TeraflopError {
    fn from(e: reqwest::Error) -> Self {
        TeraflopError::Reqwest(e)
    }
}

pub struct TeraflopAI {
    url: String,
    client: reqwest::Client,
}

impl TeraflopAI {
    pub fn new(api_key: impl Into<String>, url: impl Into<String>) -> Result<Self, TeraflopError> {
        let api_key = api_key.into();
        let url = url.into();

        let mut headers = HeaderMap::new();
        let auth = format!("Bearer {}", api_key);
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&auth).expect("valid header value"));

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .http2_prior_knowledge()
            .default_headers(headers)
            .build()?;

        Ok(Self { url, client })
    }

    pub async fn search(&self, query: &str) -> Result<Value, TeraflopError> {
        self.post(query).await
    }

    pub async fn segment(&self, query: &str) -> Result<Value, TeraflopError> {
        self.post(query).await
    }

    pub async fn embeddings(&self, query: &str, model: &str) -> Result<Value, TeraflopError> {
        let payload = serde_json::json!({ "input": query, "model": model });

        let response = self.client.post(&self.url).json(&payload).send().await?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(TeraflopError::BadStatus { status, body });
        }

        Ok(response.json::<Value>().await?)
    }

    pub async fn post(&self, query: &str) -> Result<Value, TeraflopError> {
        let payload = serde_json::json!({ "query": query });

        let res = self.client.post(&self.url).json(&payload).send().await?;

        let status = res.status();
        if !status.is_success() {
            let body = res.text().await.unwrap_or_default();
            return Err(TeraflopError::BadStatus { status, body });
        }

        Ok(res.json::<Value>().await?)
    }
}
