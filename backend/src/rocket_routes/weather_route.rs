use std::env;

use diesel::{Connection, PgConnection};
use rocket::{http::Status, serde::json::Json};
use crate::{models::NewApiLogs, repositories::api_logs_repository::ApiLogsRepository};
use serde::Deserialize;
//use crate::models::weather::WeatherData;
use shared::WeatherData;
use utoipa::OpenApi;

use crate::{
    jwt::jwt_utility, models::UserInfo, repositories::weather_repository::WeatherRepository,
};
// use crate::models::weather::WeatherData;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct TokenResponse {
    access_token: String,
    token_type: String,
    expires_in: i32,
}

#[utoipa::path(
    get,
    path = "/fetch/{city}",
    responses(
        (status = 200, description = "WeatherData found successfully", body = WeatherData),
        (status = NOT_FOUND, description = "WeatherData was not found")
    ),
    params(
        ("city" = String, Path, description = "Enter city to call API"),
    ),
    tag = "Get Operation"
)]
#[get("/fetch/<city>")]
pub async fn get_weather_api(user: jwt_utility::User, city: String) -> Json<WeatherData> {
    let weather_data = WeatherRepository::get_city_weather_by_name(&city)
        .await
        .unwrap();

    // dotenv::dotenv().ok();
    // let database_url = env::var("DATABASE_URL").expect("Could not get DATABASE URL for POSTGRES");
    // let mut conn = PgConnection::establish(&database_url).expect("Error establishing connection");
    // let created_api_log = ApiLogsRepository::create_api_log(&mut conn, NewApiLogs{
    //     trace_id: user.0.sub,
    //     func_call: String::from("get_weather_api"),
    //     status: String::from("OK"),
    //     location: Some(city),
    //     error_message: None
    // });
    // match created_api_log{
    //     Ok(_) => {println!("Successfully added a row in db");}
    //     Err(e) => {println!("Oops something went wrong: {}", e)}
    // }
    Json(weather_data)
}

pub async fn fetch_access_token() -> Result<(), String> {
    dotenv::dotenv().ok();

    let client_id = env::var("CLIENT_ID").expect("Cannot get client id");
    let client_secret = env::var("CLIENT_SECRET").expect("Cannot get client secret");
    let audience = env::var("AUDIENCE").expect("Cannot get audience");

    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .expect("Could not create client to send HTTP");

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Accept",
        "application/x-www-form-urlencoded"
            .parse()
            .expect("Could not parse http"),
    );

    let request = client
        .request(
            reqwest::Method::POST,
            "https://dev-kr7vi67c2vo4vs3w.eu.auth0.com/oauth/token",
        )
        .header("content-type", "application/x-www-form-urlencoded")
        .query(&[
            ("client_id", client_id),
            ("client_secret", client_secret),
            ("audience", audience),
        ]);
    let response = request.send().await.expect("Could not send HTTP");

    println!("Response : {:?}", response);

    let body = response
        .json::<TokenResponse>()
        .await
        .expect("Cannot convert");
    println!("Body : {:?}", body);
    Ok(())
}

pub async fn fetch_auth0_userinfo(access_token: &str) -> Result<UserInfo, String> {
    let client = reqwest::Client::builder()
        .build()
        .expect("Could not create client to send HTTP");

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Accept",
        "application/json".parse().expect("Could not parse http"),
    );

    println!("Headers: {:?}", headers);

    let request = client
        .request(
            reqwest::Method::GET,
            "https://dev-kr7vi67c2vo4vs3w.eu.auth0.com/userinfo",
        )
        .bearer_auth(access_token);
    let response = request.send().await.expect("Could not send HTTP");

    println!("Response : {:?}", response);
    let body = response
        .json::<UserInfo>()
        .await
        .expect("Could get BODY from response");

    println!("Body is: {:?}", body);
    Ok(body)
}

#[derive(OpenApi)]
#[openapi(
    paths(
        get_weather_api
    ),
    components(
        schemas(WeatherData)
    ),
    tags(
        (name = "Petar Komatina", description = "Endpoints for fetchin data")
    )
)]
pub struct ApiDoc;
