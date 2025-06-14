use crate::redis_utility::utility::{Utility, REDIS_POOL};
use shared::WeatherData;

#[async_test]
pub async fn test_redis_store_data() {
    //todo!("Write a test double for redis")
    let mut redis_conn = REDIS_POOL
        .get()
        .await
        .expect("Failed to get redis connection");

    let weather_data: WeatherData = WeatherData::default();
    let _ = Utility::store_data_in_redis(&weather_data).await;

    //let weather_data_from_redis = utility::get_cached_weather_data(&weather_data.name).await.unwrap();

    let data_from_redis: String = redis::cmd("GET")
        .arg(&weather_data.name)
        .query_async(&mut redis_conn)
        .await
        .expect("Failed to delete test data from redis");

    let weather_data_from_redis: WeatherData = serde_json::from_str(&data_from_redis).unwrap();
    let _ = Utility::delete_data_in_redis(&weather_data.name)
        .await
        .unwrap();

    let _x: String = redis::cmd("DEL")
        .arg(&weather_data.name)
        .query_async(&mut redis_conn)
        .await
        .expect("Failed to delete test data from redis");

    assert_eq!(weather_data, weather_data_from_redis);
}

#[async_test]
pub async fn test_redis_get_cached_weather_data() {
    let mut redis_conn = REDIS_POOL
        .get()
        .await
        .expect("Failed to get redis connection");

    // let client_redis = redis::Client::open("redis://backend-redis-1:6379/").unwrap();
    // let mut con: MultiplexedConnection = client_redis.get_multiplexed_async_connection().await.expect("RedisUtility: Error");

    let test_data = WeatherData::default();
    let json = serde_json::to_string(&test_data).unwrap();

    let _x: String = redis::cmd("SET")
        .arg(&test_data.name)
        .arg(json)
        .query_async(&mut redis_conn)
        .await
        .unwrap();

    let weather_data_from_redis: WeatherData = Utility::get_cached_weather_data(&test_data.name)
        .await
        .unwrap();
    assert_eq!(
        test_data, weather_data_from_redis,
        "Data from redis is not equal"
    );

    let _x: String = redis::cmd("DEL")
        .arg(&test_data.name)
        .query_async(&mut redis_conn)
        .await
        .expect("Failed to delete test data from redis");
}

#[async_test]
pub async fn test_redis_delete_data() {
    let mut redis_conn = REDIS_POOL
        .get()
        .await
        .expect("Failed to get redis connection");

    // let client_redis = redis::Client::open("redis://backend-redis-1:6379/").unwrap();
    // let mut con: MultiplexedConnection = client_redis.get_multiplexed_async_connection().await.expect("RedisUtility: Error");

    let test_data = WeatherData::default();
    let json = serde_json::to_string(&test_data).unwrap();

    let _x: String = redis::cmd("SET")
        .arg(&test_data.name)
        .arg(json)
        .query_async(&mut redis_conn)
        .await
        .unwrap();

    let _ = Utility::delete_data_in_redis(&test_data.name)
        .await
        .expect("Failed to delete data from redis");

    let result: Option<String> = redis::cmd("GET")
        .arg(test_data.name)
        .query_async(&mut redis_conn)
        .await
        .unwrap();

    assert!(result.is_none(), "Key should have been deleted from redis");
}
