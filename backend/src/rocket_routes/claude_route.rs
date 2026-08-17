// This is actually calling Python API not CLAUDE API

use rocket::{http::Status, serde::json::Json, State};

use shared::{AskRequest, AskResponse};
use crate::{models::ClaudeRequest, repositories::claude_repository::ClaudeRepository};

#[post("/ask-claude", format="json", data = "<request>")]
pub async fn chat(request: Json<AskRequest>, claude_repository: &State<ClaudeRepository>) -> Result<Json<AskResponse>, String>{
    println!("Got to the endpoint");

    let claude_request = ClaudeRequest {
        conversation_id : "test".to_string(), 
        question: request.into_inner().question
    };
    let response: Result<AskResponse, String> = claude_repository.ask_claude(&claude_request)
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
            Err(e)
        }
    }
}