use std::env;

use diesel::{Connection, PgConnection};
use repositories::api_logs_repository;
use rocket_oauth2::OAuth2;
use rocket_routes::{api_logs_route, weather_route::ApiDoc};
use rocket_sync_db_pools::database;
use utoipa::OpenApi;

use utoipa_swagger_ui::SwaggerUi;
#[macro_use]
extern crate rocket;

pub mod auth0_routes;
pub mod config_cors;
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
    let cors: rocket_cors::Cors = config_cors::cors::cors().expect("Cannot create CORS");
    let _ = rocket::build()
        .attach(cors)
        .attach(OAuth2::<auth0_routes::auth0::Auth0>::fairing("auth0"))
        .attach(DbConnection::fairing())
        .attach(api_logs_repository::ApiLogger)
        .mount("/", routes![
            rocket_routes::weather_route::get_weather_api,
            api_logs_route::get_api_logs,
            jwt::jwt_routes::get_user_claim, 
            jwt::jwt_routes::who_am_i
            ])
        .mount(
            "/",
            SwaggerUi::new("/swagger-ui/<_..>").url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
        .mount(
            "/auth0",
            routes![
                auth0_routes::auth0::login,
                auth0_routes::auth0::logout,
                auth0_routes::auth0::callback,
                auth0_routes::auth0::api_token
            ],
        )
        .launch()
        .await?;
    Ok(())
}
