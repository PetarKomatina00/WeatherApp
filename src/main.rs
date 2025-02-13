#[macro_use] extern crate rocket;


pub mod common;
pub mod rocket_routes;


#[rocket::main]
async fn main() -> Result<(), rocket::Error>{
    // let weatherapi: &str = common::OPENWEATHER_API_KEY;
    // println!("Hello, world! {:?}", weatherapi);
    let _ = rocket::build()
        .mount("/", routes![rocket_routes::get])
        .launch()
        .await?;

    Ok(())
}
