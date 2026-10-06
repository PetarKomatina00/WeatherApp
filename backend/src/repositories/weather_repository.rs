use std::{env, sync::atomic::{AtomicUsize, Ordering}};

// use crate::models::weather::WeatherData;

use crate::redis_utility::{utility::Utility, weather_single_flight::{FlightRole, WeatherSingleFlight}};
use redis::RedisError;
use reqwest::StatusCode;
use shared::WeatherData;

pub struct WeatherRepository;

static OPENWEATHER_CALLS: AtomicUsize = AtomicUsize::new(0);

impl WeatherRepository {
    //Implementing single flight for redis. 1 Connection will call openweather api and the other X number of connections will wait for redis reading


    //Attempt to fetch data from redis
    pub async fn get_city_weather_by_name_sf(city: &str, single_flight: &WeatherSingleFlight) -> Result<WeatherData, String>{
        //First normalize BARCELONA and Barcelona
        let key_city = city.trim().to_ascii_lowercase();
    
        if let Some(weather_data) = Utility::get_cached_weather_data(&key_city).await{
            print!("Cache HIT!");
            return Ok(weather_data);
        }
        println!("Cache MISS for City {city}");

        
        match single_flight.join(&key_city).await{
            FlightRole::Leader(flight) => {
                //Double checking redis for race condition
                if let Some(weather_data) = Utility::get_cached_weather_data(&key_city).await
                {
                    flight.complete_and_notify_followers();
                    single_flight.remove_flight_from_hashmap(&key_city, &flight).await;

                    return Ok(weather_data);

                }
                println!("Hello i am Leader {key_city}");

                let weather_data = Self::fetch_data_weather_api(&key_city).await;

                match weather_data{
                    Ok(weather_data) => {
                        let redis_result = Utility::store_data_in_redis(&key_city, &weather_data).await;

                        flight.complete_and_notify_followers();

                        single_flight.remove_flight_from_hashmap(&key_city, &flight).await;

                        println!("Goodbye no longer leader {key_city}");
                        match redis_result {
                            Ok(_) => return Ok(weather_data),
                            Err(err) => return Err(err.to_string())
                        }

                    }
                    Err(err) => {
                        flight.complete_and_notify_followers();

                        single_flight.remove_flight_from_hashmap(&key_city, &flight).await;

                        return Err(err);
                    }
                }
            }
            FlightRole::Follower(flight) => {
                println!("I am a follower {:?}", single_flight.num_followers);
                flight.wait_for_leader().await;
                single_flight.num_followers.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
                println!("I am no longer a follower");

                if let Some(weather_data) = Utility::get_cached_weather_data(&key_city).await{
                    return Ok(weather_data);
                }
            }
        }
        
        //When Request A finishes request B needs to again check redis.
        if let Some(weather_data) = Utility::get_cached_weather_data(&key_city).await
        {
            println!("CACHE HIT AFTER WAIT: {city}");
            return Ok(weather_data);
        }

        let weather_data = Self::fetch_data_weather_api(city).await;
        
        match weather_data{
            Ok(weather_data) => {
                println!("Storing data for {city}");

                let result = Utility::store_data_in_redis(&key_city, &weather_data).await;

                match result{
                    Ok(_) => {
                        Ok(weather_data)
                    }
                    Err(err) =>{
                        println!("Redis Error: {:?}", err);
                        return Err(err.to_string());
                    }
                }
            }
            Err(err) =>{
                print!("Error with openweather API");
                return Err(err);
            }
        }



        
    }


    // This function works and can be tested with cargo test
    pub async fn get_city_weather_by_name(city: &String) -> Result<WeatherData, String> {
        // 1. Attempt to fetch data from Redis.

        let mut weather_data: WeatherData = WeatherData::default();
        if let Some(weather_data) = Utility::get_cached_weather_data(&city).await {
            println!("Data from redis cache: {:?}", weather_data);
            //return Ok(String::from("Data from cache is stored!"));
            //todo!("Data is fetched from redis...Procceed");
            return Ok(weather_data);
        } else {
            println!("Fetching data...");
            weather_data = Self::fetch_data_weather_api(&city)
                .await?;

            println!("Storing data in redis...");
            let key_city = &city.trim().to_ascii_lowercase();
            let _x = Utility::store_data_in_redis(key_city, &weather_data).await;
        }

        // 4. Return the data (Ok) or error (Err).

        //Ok(weather_data)

        Ok(weather_data)
    }
    pub async fn fetch_data_weather_api(city: &str) -> Result<WeatherData, String> {
        println!("Fetching data started...");
        dotenv::dotenv().ok();

        let api_key =
            env::var("WEATHER_API_KEY").expect(" WeatherRepository: Missing OpenWeather API KEY");
        let url = format!(
            "http://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
            city, api_key
        );

        let number_of_calls = OPENWEATHER_CALLS.fetch_add(1, Ordering::Relaxed) + 1;
        println!("Openweather API CALL number {number_of_calls} for {city}");
        let response: reqwest::Response = reqwest::get(&url)
            .await
            .expect("WeatherRepository: Failed to get response from GET Request");
        //println!("WeatherDataFromRepository: {:?}", response);

        //println!("{:?}", response.status());
        if response.status() == StatusCode::OK {
            let response_string = response.text().await.expect("Could not convert to text");
            let weather_data: WeatherData = serde_json::from_str(&response_string)
                .expect("WeatherRepository: Failed to deserialize response");
            return Ok(weather_data);
        }

        println!("Fetching data ended...");
        Ok(WeatherData::default())
    }
}

async fn _fetch_data_weather_api(city: &str) -> Result<WeatherData, String> {
    println!("Fetching data started...");
    dotenv::dotenv().ok();

    let api_key =
        env::var("WEATHER_API_KEY").expect(" WeatherRepository: Missing OpenWeather API KEY");
    let url = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
        city, api_key
    );
    let response = reqwest::get(&url)
        .await
        .expect("WeatherRepository: Failed to get response from GET Request")
        .text()
        .await
        .expect("WeatherRepository: Failed to convert to text");
    //println!("WeatherDataFromRepository: {}", response);
    let weather_data: WeatherData =
        serde_json::from_str(&response).expect("WeatherRepository: Failed to deserialize response");

    println!("Fetching data ended...");
    Ok(weather_data)
}
