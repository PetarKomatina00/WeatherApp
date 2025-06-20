
use chrono::NaiveDateTime;
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
    pub created_at: NaiveDateTime,
    pub status: String,
    pub location: Option<String>,
    pub error_message: Option<String>,
}

#[derive(Insertable, Debug)]
#[table_name="api_logs"]
pub struct NewApiLogs{
    pub trace_id: String,
    pub func_call: String,
    pub status: String,
    pub location: Option<String>,
    pub error_message: Option<String>
}

#[derive(Serialize)]
pub struct TokenRequest<'a> {
    pub grant_type: &'a str,
    pub client_id: &'a str,
    pub client_secret: &'a str,
    pub audience: &'a str,
    pub redirect_uri: &'a str,
}

#[derive(Deserialize, Debug)]
pub struct TokenResponse {
    _access_token: String,
    #[allow(dead_code)]
    token_type: String,
    #[allow(dead_code)]
    expires_in: u64,
    #[allow(dead_code)]
    id_token: String,
}
#[derive(Debug, Deserialize, Default, PartialEq)]
pub struct JsonWebKey {
    pub kty: String,
    #[serde(rename = "use")]
    pub key_use: String,
    pub n: String,
    pub e: String,
    pub kid: String,
    pub x5t: String,
    pub x5c: Vec<String>,
    pub alg: String,
}

#[derive(Debug, Deserialize)]
pub struct VecJsonWebKey {
    #[serde(rename = "keys")]
    pub jwk_vec: Vec<JsonWebKey>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Claims {
    pub sub: String,
    iss: String,
    aud: Vec<String>,
    exp: usize,
    iat: usize,
    scope: Option<String>,
    #[serde(rename = "https://rust-weather-api.com/roles")]
    user_type: Vec<String>,
    permissions: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Profile {
    pub name: String,
    pub sub: String,
    pub email: String,
    pub family_name: String,
    pub given_name: String,
    pub email_verified: bool,
}
