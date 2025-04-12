use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(ToSchema, Serialize, Deserialize)]
pub struct MySwaggerResponse{
    pub response: String, 
    pub value: i32
}