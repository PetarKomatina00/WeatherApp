use chrono::NaiveDateTime;
use serde::Deserialize;
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
