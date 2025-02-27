use std::env;

use diesel::{Connection, PgConnection};
use rocket_sync_db_pools::database;

#[macro_use] extern crate rocket;


pub mod common;
pub mod rocket_routes;
pub mod repositories;
pub mod models;
mod schema;
 


#[database("postgres")]
pub struct DbConnection(PgConnection);


pub fn establish_connection() -> PgConnection{
    let database_url = env::var(common::POSTGRES_URL).expect("Database url must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("error connection to {}", database_url))
}


#[rocket::main]
async fn main() -> Result<(), rocket::Error>{
    // let weatherapi: &str = common::OPENWEATHER_API_KEY;
    // println!("Hello, world! {:?}", weatherapi);
    let _ = rocket::build()
        .mount("/", routes![
            rocket_routes::weather_route::get_weather_api
            
            ])
        .attach(DbConnection::fairing())
        .launch()
        .await?;
    Ok(())
}
