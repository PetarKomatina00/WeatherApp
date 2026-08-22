// This is actually calling Python API not CLAUDE API

use rocket::{serde::json::Json, State};

use shared::{ClaudeRequest, ClaudeResponse};
use crate::{jwt::jwt_guards::User, models::ClaudeServiceRequest, repositories::claude_repository::ClaudeRepository};

#[post("/ask-claude", format="json", data = "<request>")]
pub async fn chat(_user: User, request: Json<ClaudeRequest>, claude_repository: &State<ClaudeRepository>) -> Result<Json<ClaudeResponse>, String>{
    println!("Got to the endpoint");

    let parsed_request = request.into_inner();
    

    let conversation_id = _user.0.sub;
    let claude_service_request = ClaudeServiceRequest {
        conversation_id,
        question: parsed_request.question,
        use_mcp_weather: parsed_request.use_mcp_weather,
        weather_data : parsed_request.weather_data
    };
    
    let response: Result<ClaudeResponse, String> = claude_repository.ask_claude(&claude_service_request)
    .await
    .map_err(|error| {
        eprintln!("Error communicatin with python claude {}", error);
        error
    });
    println!("Endpoint finished");
    match response {
        Ok(result) => {
            Ok(Json(result))
        },
        Err(e) => {
            eprintln!("Error communicatin with python claude {}", e);
            Err(e)
        }
    }
}