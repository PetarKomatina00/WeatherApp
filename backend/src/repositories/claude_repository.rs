use std::env;
use reqwest::Response;
use shared::{ClaudeRequest, ClaudeResponse};

pub struct ClaudeRepository{
    client: reqwest::Client,
    python_api_url: String
}
impl ClaudeRepository{
    pub fn new() -> Self{
        dotenv::dotenv().ok();
        let python_api_url = env::var("PYTHON_API_URL").expect("Python API URL not set");
        Self { 
            client: reqwest::Client::new(),
            python_api_url }
    }


    pub async fn ask_claude(&self, claude_request: &ClaudeRequest) -> Result<ClaudeResponse, String> {

        println!("Starting in repository");
        let url = format!("{}/ask-claude", self.python_api_url);

        println!("Claude Request: {:?}", claude_request);
        
        let response_result = self
        .client.post(url).json(claude_request).send()
        .await.map_err(|e| {
            eprintln!("Failed to send request to Claude: {e:?}");
            e
        });

        let response: Response = match response_result{
            Ok(response) => {
                response
            },
            Err(e) => {
                let error_message = format!("Error reading response message: {}", e);
                return Err(error_message);
            }
        };

        let status = response.status();

        let body = response.text().await.map_err(|e| {
            eprintln!("Failed to read from body {}", e);
            e
        });

        if !status.is_success(){
            eprintln!("Failed to read response body {}", body.unwrap());
            return Err(format!("Something went wrong with claude {}", status));
        }

        

        match body{
            Ok(body_string) => {
                let response: Result<ClaudeResponse, serde_json::Error> = serde_json::from_str(&body_string);
                match response{
                    Ok(response_result) => {
                        return Ok(response_result);
                    },
                    Err(e) => {
                        let error_message = format!("Failed to deserialize body: {}", e);
                        return Err(error_message);
                    }
                }
            },
            Err(e) =>{
                let error_message = format!("Error converting response to body {}", e);
                return Err(error_message);
            }
        }
    }
}