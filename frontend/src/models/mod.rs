use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct Profile {
    pub name: String,
    #[allow(dead_code)]
    pub sub: String,
    #[allow(dead_code)]
    pub email: String,
    #[allow(dead_code)]
    pub family_name: String,
    #[allow(dead_code)]
    pub given_name: String,
    #[allow(dead_code)]
    pub email_verified: bool,
}

#[derive(Debug, Deserialize)]
pub struct ApiLogs {
    #[allow(dead_code)]
    pub logs_id: Uuid,
    pub trace_id: String,
    pub func_call: String,
    pub created_at: NaiveDateTime,
    pub status: String,
    pub location: Option<String>,
    pub error_message: Option<String>,
}


#[derive(Deserialize, Debug)]
pub struct GeoResult{
    pub lat: String,
    pub lon: String,
}

#[derive(Clone, PartialEq)]
pub enum MessageRole{
    User,
    Assistant
}

#[derive(Clone, PartialEq)]
pub struct ChatMessage{
    pub role: MessageRole,
    pub content: String,
}

#[derive(Serialize)]
pub struct AskRequest{
    pub question: String
}

#[derive(Deserialize)]
pub struct AskResponse{
    pub answer: String
}