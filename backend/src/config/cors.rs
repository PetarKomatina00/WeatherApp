use rocket_cors::{AllowedOrigins, AllowedHeaders};
use rocket::{fairing::{Fairing, Info, Kind}, http::Method};
use rocket::{Request, Response};
use rocket::http::Header;
use std::{env, error::Error};



pub fn cors() -> Result<rocket_cors::Cors, Box<dyn Error>>{
    dotenv::dotenv().ok();
    //let host_url = env::var("FRONTEND_URL").expect("Cannot get HOST URL");
    let allowed_origins = AllowedOrigins::some_exact(&[
        "http://127.0.0.1:8001",
        "http://localhost:8001",
        ]);
    let cors: rocket_cors::Cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post, Method::Put, Method::Delete, Method::Options].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept", "Content-Type"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()?;

    Ok(cors)
}