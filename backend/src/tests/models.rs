use serde::Deserialize;

#[derive(Deserialize, PartialEq, Debug)]
pub struct TestWeatherData {
    pub name: String,
    pub timezone: i32,
    pub coord: Coords,
    pub cod: i32,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Coords {
    pub lon: f32,
    pub lat: f32,
}
