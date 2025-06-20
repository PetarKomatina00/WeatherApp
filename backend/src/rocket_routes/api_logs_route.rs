use diesel::PgConnection;
use serde_json::Value;

use crate::{repositories::api_logs_repository::{self, ApiLogsRepository}, DbConnection};
use rocket::{http::Status, response::status::Custom, serde::json::serde_json::json};

#[get("/api/logs?<limit>")]
pub async fn get_api_logs(connection: DbConnection, limit: i64) -> Result<Value, Custom<Value>>{
    connection.run(move |c| {
        ApiLogsRepository::find_multiple_api_logs(c, limit)
        .map(|api_log: Vec<crate::models::ApiLogs>| json!(api_log))
        .map_err(|e| Custom(Status::InternalServerError, json!("Something went wrong")))
    }).await
}