use reqwasm::http::Request;
use web_sys::RequestCredentials;

use shared::{AskRequest, AskResponse};

pub async fn send_chat_message(question: &str) -> Result<AskResponse, reqwasm::Error> {
    let request = AskRequest{
        question: question.to_string(),
    };

    let body = serde_json::to_string(&request);
    match body{
        Ok(body) => {
            let response = Request::post("http://127.0.0.1:8000/ask-claude")
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .credentials(RequestCredentials::Include)
            .body(body)
            .send()
            .await?;

            let chat_response = response.json::<AskResponse>().await?;
            Ok(chat_response)
        }
        Err(err) => {
            println!("Something went wrong with the call {err}");
            Err(reqwasm::Error::SerdeError(err))
        }
    }
}