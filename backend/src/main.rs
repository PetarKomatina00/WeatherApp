use std::env;

use diesel::{Connection, PgConnection};
use repositories::api_logs_repository::{self, ApiLogsRepository};
use rocket_oauth2::OAuth2;
use rocket_routes::{api_logs_route, weather_route::ApiDoc};
use rocket_sync_db_pools::database;
use utoipa::OpenApi;

use utoipa_swagger_ui::SwaggerUi;
#[macro_use]
extern crate rocket;

pub mod auth0;
pub mod common;
pub mod config;
pub mod jwt;
pub mod models;
pub mod redis_utility;
pub mod repositories;
pub mod rocket_routes;
mod schema;
pub mod swagger;
pub mod tests;


#[database("postgres")]
pub struct DbConnection(PgConnection);

pub fn establish_connection() -> PgConnection {
    dotenv::dotenv().ok();
    let database_url = env::var("POSTGRES_URL").expect("Database url must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("error connection to {}", database_url))
}
#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _city_name = String::from("Barcelona");
    let cors: rocket_cors::Cors = config::cors::cors().expect("Cannot create CORS");
    let _ = rocket::build()
        .attach(cors)
        .mount("/", routes![rocket_routes::weather_route::get_weather_api,])
        .mount(
            "/",
            SwaggerUi::new("/swagger-ui/<_..>").url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
        .mount(
            "/auth0",
            routes![
                auth0::auth0::login,
                auth0::auth0::logout,
                auth0::auth0::callback,
                auth0::auth0::api_token
            ],
        )
        .mount(
            "/",
            routes![
                api_logs_route::get_api_logs
            ]
        )
        .mount(
            "/",
            routes![jwt::jwt_utility::get_user_claim, jwt::jwt_utility::who_am_i],
        )
        .attach(OAuth2::<auth0::auth0::Auth0>::fairing("auth0"))
        .attach(DbConnection::fairing())
        .attach(api_logs_repository::ApiLogger)
        .launch()
        .await?;
    Ok(())
}
