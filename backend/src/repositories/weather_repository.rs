use diesel::PgConnection;

pub struct WeatherRepository;

impl WeatherRepository{
    pub fn get_city_weather_by_name(c: &PgConnection, name: String) -> Result<String, String>{
        // TO DO in REDIS 
        //Calling THE API
        Ok(String::from("Function call is good"))
    }
}