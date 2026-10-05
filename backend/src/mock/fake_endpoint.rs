use std::{sync::atomic::{AtomicUsize, Ordering}, time::Duration};

use rocket::{State, serde::json::Json};
use serde::Serialize;

pub struct FakeWeatherState{
    pub calls: AtomicUsize
}

#[derive(Serialize)]
pub struct FakeWeatherData{
    city: String,
    temperature: f64,
    description: String
}
#[get("/weather?<city>")]
pub async fn get_fake_weather_data(city: String, state: &State<FakeWeatherState>) -> Json<FakeWeatherData>{
    let call_number = state.calls.fetch_add(1, Ordering::Relaxed);

    println!("Call number ${call_number} to {city}");

    rocket::tokio::time::sleep(Duration::from_millis(500)).await;

    Json(FakeWeatherData {
        city : "Barcelona".to_string(),
        temperature : 28.7,
        description : "clear sky".to_string()
    })
}
#[get("/calls")]
pub fn get_number_of_calls(state: &State<FakeWeatherState>) -> String{
    state.calls.load(Ordering::Relaxed).to_string()
}
#[post("/calls/reset")]
pub fn reset_state_counter(state: &State<FakeWeatherState>){
    state.calls.store(0, Ordering::Relaxed);
}
