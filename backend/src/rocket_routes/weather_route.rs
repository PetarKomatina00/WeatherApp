use rocket::serde::json::{serde_json::json, Value};

use crate::{repositories::weather_repository::WeatherRepository, DbConnection};

#[get("/fetch/<city>")]
pub fn get_weather_api(db: DbConnection, city: String) -> Value{
    db.run(|c| {
        WeatherRepository::get_city_weather_by_name(&c, city)
    });



    println!("Hello from the weather app");


    json!([
        
    ])
}