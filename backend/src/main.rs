use std::env;

use diesel::{Connection, PgConnection};
use repositories::weather_repository::WeatherRepository;
use rocket_sync_db_pools::database;

#[macro_use] extern crate rocket;


pub mod redis_utility;
pub mod common;
pub mod rocket_routes;
pub mod repositories;
pub mod models;
pub mod tests;
mod schema;
 


#[database("postgres")]
pub struct DbConnection(PgConnection);


pub fn establish_connection() -> PgConnection{
    dotenv::dotenv().ok();
    let database_url = env::var("POSTGRES_URL").expect("Database url must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("error connection to {}", database_url))
}


#[rocket::main]
async fn main() -> Result<(), rocket::Error>{
    // let weatherapi: &str = common::OPENWEATHER_API_KEY;
    // println!("Hello, world! {:?}", weatherapi);
    println!("Hello from main");
    let city_name = String::from("Barcelona");

    let _ = rocket::build()
        .mount("/", routes![
            rocket_routes::weather_route::get_weather_api
            ])
        .attach(DbConnection::fairing())
        .launch()
        .await?;


        // println!("Calling weather repository");
        // let result = WeatherRepository::get_city_weather_by_name(city_name).await;
        // match result {
        // Ok(message) => println!("Success {}", message),
        // Err(err) => println!("Error: {}", err)
        //}
    Ok(())
}
