use std::env;

use super::models::Coords;
use crate::tests::models::TestWeatherData;

use dotenv::var;
#[allow(unused_imports)]
use reqwest::StatusCode;
pub fn create_expected_weather_data() -> TestWeatherData {
    let dummy_weather_data = TestWeatherData {
        name: String::from("Barcelona"),
        timezone: 7200,
        coord: Coords {
            lon: 2.159,
            lat: 41.3888,
        },
        cod: 200,
    };
    dummy_weather_data
}

#[async_test]
async fn test_get_weather_api() {
    //todo!("Write a mock function")
    let city = String::from("Barcelona");
    let client = reqwest::Client::new();

    let backend_kuber_url = env::var("BACKEND_KUBER_URL").expect("Could not load backend kuber url");
    let backend_dev_url = env::var("BACKEND_DEV_URL").expect("Could not load backend kuber url");
    
    let url = format!("{}/{}", backend_kuber_url,city);
    let response = client
        .get(&url)
        .bearer_auth("secret-token")
        .send()
        .await
        .expect("Test: Failed to send request");
    assert_eq!(response.status(), StatusCode::OK);

    let expected_weather_data: TestWeatherData = create_expected_weather_data();
    let response_body: TestWeatherData = response
        .json::<TestWeatherData>()
        .await
        .expect("Test: Failed to deseriazlize as TestWeatherData");

    assert_eq!(response_body, expected_weather_data);
}
