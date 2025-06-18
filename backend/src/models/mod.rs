
use chrono::DateTime;
use chrono::NaiveDateTime;
use chrono::Utc;
use diesel::prelude::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::schema::*;
#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub sub: String,
    pub email: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct ApiLogs{
    pub logs_id: Uuid,
    pub trace_id: String,
    pub func_call: String,
    pub timestamp: NaiveDateTime,
    pub status: String,
    pub location: Option<String>,
    pub error_message: Option<String>,
}

#[derive(Insertable)]
#[table_name="api_logs"]
pub struct NewApiLogs{
    pub func_call: String,
    pub status: String,
    pub location: Option<String>,
    pub error_message: Option<String>
}
