use redis::{aio::MultiplexedConnection, RedisResult};

use crate::{models::weather::WeatherData, redis_utility::utility::utility};

#[async_test]
pub async fn test_store_data_in_redis(){

    //todo!("Write a test double for redis")
    let weather_data = WeatherData::default();
    let _ = utility::store_data_in_redis(&weather_data).await;
    let weather_data_from_redis = utility::get_cached_weather_data(&weather_data.name).await.unwrap();

    let _ = utility::delete_data_in_redis(&weather_data.name).await.unwrap();


    println!("WEATHERDATA: {:?}", weather_data);
    println!("REDIS: {:?}", weather_data_from_redis);
    assert_eq!(weather_data, weather_data_from_redis);
}

#[async_test]
pub async fn test_get_cached_weather_data(){
    let client_redis = redis::Client::open("redis://backend-redis-1:6379/").unwrap();
    let mut con: MultiplexedConnection = client_redis.get_multiplexed_async_connection().await.expect("RedisUtility: Error");

    let test_data = WeatherData::default();
    let json = serde_json::to_string(&test_data).unwrap();

    let x: String = redis::cmd("SET")
    .arg(&test_data.name)
    .arg(json)
    .query_async(&mut con)
    .await
    .unwrap();

    let data_from_redis: String = redis::cmd("GET")
    .arg(&test_data.name)
    .query_async(&mut con)
    .await
    .unwrap();

    let json_data_from_redis: WeatherData = serde_json::from_str(&data_from_redis).expect("Failed to deserialize");
    assert_eq!(test_data, json_data_from_redis, "Data from redis is not equal");

    let x: String = redis::cmd("DEL")
    .arg(&test_data.name)
    .query_async(&mut con)
    .await
    .expect("Failed to delete test data from redis");
}

//todo!("Make a connection pool")

#[async_test]
pub async fn test_delete_data_in_redis(){
    let client_redis = redis::Client::open("redis://backend-redis-1:6379/").unwrap();
    let mut con: MultiplexedConnection = client_redis.get_multiplexed_async_connection().await.expect("RedisUtility: Error");

    let test_data = WeatherData::default();
    let json = serde_json::to_string(&test_data).unwrap();

    let x: String = redis::cmd("SET")
    .arg(&test_data.name)
    .arg(json)
    .query_async(&mut con)
    .await
    .unwrap();

    let _ = utility::delete_data_in_redis(&test_data.name).await.expect("Failed to delete data from redis");

    let result: Option<String> = redis::cmd("GET").arg(test_data.name).query_async(&mut con).await.unwrap();

    assert!(result.is_none(), "Key should have been deleted from redis");
}