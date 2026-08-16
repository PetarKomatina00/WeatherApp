use std::env;
use shared::{AskRequest, AskResponse};
pub struct ClaudeRepository{
    client: reqwest::Client,
    python_api_url: String
}
impl ClaudeRepository{
    pub fn new() -> Self{
        dotenv::dotenv().ok();
        python_api_url = env::var("PYTHON_API_URL").expect("Python API URL not set");
        Self { 
            client: reqwest::Client::new(),
            python_api_url: python_api_url }
    }


    pub async fn ask_claude(&self, request: &AskRequest) -> Result<AskResponse, reqwest::Error> {
        let url = format!("{}/ask-claude", self.python_api_url);

        let response = self
        .client.post(url).json(request).send()
        .await?.error_for_status()?.json::<AskResponse>().await?;

        Ok(response)

    }
}