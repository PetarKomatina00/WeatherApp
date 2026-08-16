// This is actually calling Python API not CLAUDE API

use rocket::{http::Status, serde::json::Json, State};

use shared::{AskRequest, AskResponse};
use crate::repositories::claude_repository::ClaudeRepository;

#[post("/ask-claude", format="json", data = "<request>")]
pub async fn chat(request: Json<AskRequest>, claude_repository: &State<ClaudeRepository>) -> Result<Json<AskResponse>, Status>{
    let response = claude_repository.ask_claude(&request.into_inner())
    .await
    .map_err(|_| Status::BadGateway)?;

    Ok(Json(response))
}