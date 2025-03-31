use std::env;

// use crate::models::weather::WeatherData;

use shared::WeatherData;
use crate::redis_utility::utility::Utility;
pub struct WeatherRepository;

impl WeatherRepository{
    pub async fn get_city_weather_by_name(city: &String) -> Result<WeatherData, String>{
        // 1. Attempt to fetch data from Redis.

        let mut weather_data: WeatherData = WeatherData::default();
        if let Some(weather_data) = Utility::get_cached_weather_data(&city).await{
            println!("Data from redis cache: {:?}", weather_data);
            //return Ok(String::from("Data from cache is stored!"));
            //todo!("Data is fetched from redis...Procceed");
        }
        else{
            println!("Fetching data...");
            weather_data = Self::fetch_data_weather_api(&city).await.unwrap();
        
            println!("Storing data in redis...");
            Utility::store_data_in_redis(&weather_data).await;

            
            
        }

        // 4. Return the data (Ok) or error (Err).
        
        //Ok(weather_data)
        Ok(weather_data)
    }

    async fn fetch_data_weather_api(city: &str) -> Result<WeatherData, String>{
        println!("Fetching data started...");
        dotenv::dotenv().ok();

        
        let api_key = env::var("WEATHER_API_KEY").expect(" WeatherRepository: Missing OpenWeather API KEY");
        let url = format!(
            "http://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
            city, api_key
        );
        println!("Hello!");
        println!("Hello Again");
        println!("API KEY is: {}", api_key);
        let response = reqwest::get(&url).await.expect("WeatherRepository: Failed to get response from GET Request").text().await.expect("WeatherRepository: Failed to convert to text");
        let weather_data: WeatherData = serde_json::from_str(&response).expect("WeatherRepository: Failed to deserialize response");


        println!("Fetching data ended...");
        Ok(weather_data)
    }
    
}

