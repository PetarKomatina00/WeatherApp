use gloo_net::http::{Request, Response};
pub enum FetchError {
    Unauthorized,
}
#[derive(Clone, PartialEq, Default)]
pub struct ButtonContent {
    pub content: String,
}

pub async fn fetch_weather_data(data: &ButtonContent, backend_url: String) -> Result<Response, FetchError> {
    println!("Fetchin data started...");
    let response = Request::get(&format!("{}/fetch/{}", backend_url, data.content))
        .credentials(reqwasm::http::RequestCredentials::Include)
        .send()
        .await
        .map_err(|error| error.to_string());
    let response = response.unwrap();
    // if !response.ok(){
    //     log!(format!("The user needs to be authenticated. Failed to fetch weather, status: {}", response.status()));
    //     alert("The user needs to be authenticated!");
    //     FetchError::Unauthorized
    // }
    //println!("Response is: {:?}", response);
    match response.status() {
        200..=299 => Ok(response),
        _ => Err(FetchError::Unauthorized),
    }
}
