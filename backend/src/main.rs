use std::env;

use diesel::{Connection, PgConnection};
use rocket_oauth2::OAuth2;
use rocket_routes::weather_route::ApiDoc;
use rocket_sync_db_pools::database;
use utoipa::OpenApi;

use utoipa_swagger_ui::SwaggerUi;
#[macro_use] extern crate rocket;


pub mod redis_utility;
pub mod common;
pub mod rocket_routes;
pub mod repositories;
pub mod tests;
mod schema;
pub mod config;
pub mod swagger;
pub mod auth0;
pub mod models;
pub mod jwt;
// #[options("/<_..>")]
// fn all_options() -> rocket::http::Status {
//     rocket::http::Status::Ok
// }

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
    println!("Hello from main AGAIN");
    println!("Yo brooo");
    let _city_name = String::from("Barcelona");
    let cors: rocket_cors::Cors = config::cors::cors().expect("Cannot create CORS");
    let _ = rocket::build()
        .attach(cors)
        .mount("/", routes![
            rocket_routes::weather_route::get_weather_api,
            ])
        .mount(
            "/",
            SwaggerUi::new("/swagger-ui/<_..>").url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
        .mount("/auth0", routes![
            auth0::auth0::login, auth0::auth0::callback, 
            auth0::auth0::api_token
        ])
        .mount("/", routes![jwt::jwt_utility::get_user_claim, 
        jwt::jwt_utility::get_user_info,
        jwt::jwt_utility::validate])
        .attach(OAuth2::<auth0::auth0::Auth0>::fairing("auth0"))
        
        // .attach(DbConnection::fairing())
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
