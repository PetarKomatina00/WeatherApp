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

use diesel::prelude::Queryable;
use serde::{Serialize, Deserialize};


    /*
        "coord":{"lon":2.159,"lat":41.3888},
        "weather":[{"id":801,"main":"Clouds","description":"few clouds","icon":"02n"}],
        "base":"stations",
        "main":{"temp":11.44,"feels_like":10.42,"temp_min":10.06,"temp_max":12.4,"pressure":1019,"humidity":68,"sea_level":1019,"grnd_level":1011},
        "visibility":10000,
        "wind":{"speed":3.09,"deg":120},
        "clouds":{"all":20},
        "dt":1740770131,
        "sys":{"type":2,"id":18549,"country":"ES","sunrise":1740724059,"sunset":1740764428},
        "timezone":3600,
        "id":3128760,
        "name":"Barcelona",
        "cod":200
     */


#[derive(Queryable, Deserialize, Serialize, Debug)]
pub struct WeatherData{
    id: i32,  
    pub name: String,
    pub main: Main,
    pub sys: Sys,
    pub timezone: i32,
    pub coord: Coords, 
    pub cod: i32
}

#[derive(Queryable, Deserialize, Serialize, Debug)]
pub struct Main{
    pub humidity: i32,
    pub temp: f32,
    pub feels_like: f32,
    pub temp_min: f32,
    pub temp_max: f32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Sys{
    pub sunrise: i64,
    pub sunset: i64,
}

#[derive(Deserialize, PartialEq, Serialize, Debug)]
pub struct Coords{
    pub lon: f32,
    pub lat: f32
}
