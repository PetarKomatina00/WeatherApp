// This is actually calling Python API not CLAUDE API

use rocket::{http::Status, serde::json::Json, State};

use shared::{AskRequest, AskResponse};
use crate::repositories::claude_repository::ClaudeRepository;

#[post("/ask-claude", format="json", data = "<request>")]
pub async fn chat(request: Json<AskRequest>, claude_repository: &State<ClaudeRepository>) -> Result<Json<AskResponse>, String>{
    println!("Got to the endpoint");
    let response: Result<AskResponse, String> = claude_repository.ask_claude(&request.into_inner())
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