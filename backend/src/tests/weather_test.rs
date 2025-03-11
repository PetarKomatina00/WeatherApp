
use super::models::Coords;
use crate::tests::models::TestWeatherData;

#[allow(unused_imports)]
use reqwest::StatusCode;
pub fn create_expected_weather_data() -> TestWeatherData{
    let dummy_weather_data = TestWeatherData{
        name : String::from("Barcelona"),
        timezone: 3600,
        coord: Coords {
            lon: 2.159, 
            lat: 41.3888
        },
        cod: 200
    };
    dummy_weather_data
}

#[async_test]
async fn test_get_weather_api(){

    //todo!("Write a mock function")
    let city = String::from("Barcelona");
    let client = reqwest::Client::new();

    let url = format!("http://127.0.0.1:8000/fetch/{}", city);
    let response = client.get(&url).send().await.expect("Test: Failed to send request");
    assert_eq!(response.status(), StatusCode::OK);

    
    let expected_weather_data: TestWeatherData = create_expected_weather_data();
    let response_body: TestWeatherData = response.json::<TestWeatherData>().await.expect("Test: Failed to deseriazlize as TestWeatherData");

    assert_eq!(response_body, expected_weather_data);
}


