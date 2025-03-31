use std::env;

use deadpool_redis::{Config, Pool, Runtime};
use lazy_static::lazy_static;
use redis::{aio::MultiplexedConnection, AsyncCommands};
//use crate::models::weather::WeatherData;
use shared::WeatherData;
pub struct Utility;

lazy_static! {
    pub static ref REDIS_POOL: Pool = {
        dotenv::dotenv().ok();
        let redis_url = env::var("REDIS_URL").expect("Cannot get redis url");
        let cfg: Config = Config::from_url(redis_url);
        let pool_con = cfg.create_pool(Some(Runtime::Tokio1)).expect("Failed to create Redis Pool");

        pool_con
    };
}

impl Utility{
    pub async fn get_cached_weather_data(key: &str) -> Option<WeatherData>{

        //let mut redis_conn = redis_test::REDIS_POOL.get().await.expect("Failed to get redis connection");
        //let client_redis = redis::Client::open(REDIS_POOL).unwrap();

        let mut con = REDIS_POOL.get().await.expect("Failed to get redis pool");
        //let mut con: MultiplexedConnection = client_redis.get_multiplexed_async_connection().await.expect("RedisUtility: Error");

        let cached_json: Option<String> = con.get(key).await.unwrap();
        
        println!("{:?}", cached_json);
        println!("Get cached weather data ended");

        match cached_json {
            None => return None,
            Some(_) => {
                return Some(serde_json::from_str(&cached_json.unwrap()).expect("Utility: Failed to parse to WeatherData"));
            }

        }
    }

    pub async fn store_data_in_redis(weather_data: &WeatherData){
        println!("Storing data in redis started...");

        let mut con = REDIS_POOL.get().await.expect("Failed to get redis connection");
        // let client_redis = redis::Client::open("redis://backend-redis-1:6379/").unwrap();
        // let mut con: MultiplexedConnection = client_redis.get_multiplexed_async_connection().await.expect("RedisUtility: Error connection to redis");
    
        let key = format!("{}", weather_data.name);

    
        let json_data = serde_json::to_string(&weather_data).expect("Utility: Failed to convert to string");
        let result: redis::RedisResult<()> = con.set_ex(&key, json_data, 600).await;
        
        println!("Storing data in ended...");
        match result {
            Ok(_) => println!("Weather data successfully stored in redis"),
            Err(err) => println!("Error storing data in Redis: {}", err),
        }
    }
    pub async fn delete_data_in_redis(key: &str) -> Result<(), String>{

        let mut con = REDIS_POOL.get().await.expect("Failed to get redis connection");
        println!("Deleting data in redis started...");
        //let client_redis = redis::Client::open("redis://backend-redis-1:6379/").unwrap();
        //let mut con: MultiplexedConnection = client_redis.get_multiplexed_async_connection().await.expect("RedisUtility: Error connection to redis");

        //todo!("Call function get_del. Need to implement FromRedisValue trait")
        let deleted_item: String = con.del(key).await.unwrap();

        println!("Item deleted successfully {}", deleted_item);
        if !String::is_empty(&deleted_item) {
            Ok(())
        }
        else{
            Err(String::from("Error"))
        }
    }
    pub fn make_connection_pool_redis() -> Pool{
        let cfg: Config = Config::from_url(env::var("REDIS_URL").unwrap());
        let pool = cfg.create_pool(Some(Runtime::Tokio1)).unwrap();
        
        pool
    
    }
    
}