/*
    -- Your SQL goes here
CREATE TABLE weather(
    id SERIAL PRIMARY KEY,
    city_name TEXT NOT NULL,
    temperature FLOAT NOT NULL,
    description TEXT NOT NULL,
    humidity INT,
    wind_speed FLOAT,
    sunrise BIGINT,
    sunset BIGINT
)

*/


#[derive(Queryable)]
struct Weather{
    id: i32,  
    city_name: String,
    temperature: String,
    temperature: f32,
    description: String, 
    humidity: Option<i32>,
    wind_speed: Option<i32>,
    sunrise: Option<i64>,
    sunset: Option<i64>
}

