use reqwasm::http::Request;
use wasm_bindgen::JsValue;
use web_sys::RequestCredentials;

use shared::{ClaudeRequest, ClaudeResponse, WeatherData};

pub async fn send_chat_message(question: &str, use_mcp_weather: bool, weather_data: Option<WeatherData>) -> Result<ClaudeResponse, reqwasm::Error> {
    let request = ClaudeRequest{
        question: question.to_string(),
        use_mcp_weather,
        weather_data: weather_data,
    };

    let body = serde_json::to_string(&request);
    web_sys::console::log_1(&JsValue::from_str(&(body.as_ref().unwrap().clone()).to_string()));
    match body{
        Ok(body) => {
            let response = Request::post("http://127.0.0.1:8000/ask-claude")
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .credentials(RequestCredentials::Include)
            .body(body)
            .send()
            .await?;

            let chat_response = response.json::<ClaudeResponse>().await?;
            Ok(chat_response)
        }
        Err(err) => {
            println!("Something went wrong with the call {err}");
            Err(reqwasm::Error::SerdeError(err))
        }
    }
}