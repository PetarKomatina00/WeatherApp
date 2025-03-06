use std::env;

use rocket::{futures::TryFutureExt, response::Responder, serde::{self, json::{self, serde_json::json, Json}}};
use serde_json::Value;
use reqwest::{self, Response};
use crate::{repositories::weather_repository::WeatherRepository, DbConnection};
use crate::models::weather::WeatherData;



#[get("/fetch/<city>")]
pub async fn get_weather_api(city: String) -> Result<String, String>{
    //todo!("Do something with data...");
    WeatherRepository::get_city_weather_by_name(&city).await
}