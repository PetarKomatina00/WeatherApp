use shared::WeatherData;
use yew::html;
use yew::{function_component, Html, Properties};

use chrono::{Local, TimeZone};
#[derive(Properties, PartialEq)]
pub struct Props {
    pub weather_data: WeatherData,
}
#[function_component(WeatherCard)]
pub fn weather_card(props: &Props) -> Html {
    let todays_time = Local::now().format("%d/%m").to_string();
    let todays_day = Local::now().format("%A").to_string();
    let sunrise = Local
        .timestamp_opt(props.weather_data.sys.sunrise, 0)
        .unwrap();
    let display_sunrise = sunrise.format("%H:%M").to_string();

    let sunset = Local
        .timestamp_opt(props.weather_data.sys.sunset, 0)
        .unwrap();
    let display_sunset = sunset.format("%H:%M").to_string();
    html! {
        <div class="container py-4">
            <div class="card border-start-red">
              <div class="card-body">
                <div class="d-flex justify-content-between align-items-start flex-wrap">
                  <div class="me-3 text-start">
                    <div class="text-uppercase small fw-semibold">{todays_time}</div>
                    <div class="small text-muted">{todays_day}</div>
                  </div>
                  <div class="d-flex align-items-center flex-wrap gap-2 flex-grow-1">
                    <img src="https://developer.accuweather.com/sites/default/files/02-s.png"
                         alt="Partly cloudy" class="icon-xl" />
                    <div class="fs-temp fw-bold">{props.weather_data.main.temp_max}</div>
                    <div class="fs-5 text-muted">{props.weather_data.main.temp_min}</div>
                  </div>
                </div>
                <p class="mt-3 mb-4 h6">{props.weather_data.name.clone()}</p>
                <div class="alert-card p-3 mb-4 d-flex justify-content-between align-items-start">
                  <div>
                    <div class="d-flex align-items-center mb-1">
                      <i class="bi bi-exclamation-triangle-fill text-danger me-2"></i>
                      <span class="fw-semibold text-danger">{"CAUTION!"}</span>
                    </div>
                    <div class="fw-semibold">{"You are currently seeing the best weather app written in RUST and YEW"}</div>
                    <div class="small text-muted">{"Written by: Petar Komatina "}</div>
                  </div>
                  <i class="bi bi-arrow-right fs-5"></i>
                </div>
                <div class="row text-center g-3">
                  <div class="col-6 col-md-3">
                    <div class="kv-key">{"Reel Feel"}</div>
                    <div class="kv-value">{props.weather_data.main.feels_like}</div>
                  </div>
                  <div class="col-6 col-md-3">
                    <div class="kv-key">{"Humidity"}</div>
                    <div class="kv-value">{props.weather_data.main.humidity}</div>
                  </div>
                  <div class="col-6 col-md-3">
                    <div class="kv-key">{"Sunrise"}</div>
                    <div class="kv-value">{display_sunrise}</div>
                  </div>
                  <div class="col-6 col-md-3">
                    <div class="kv-key">{"Sunset"}</div>
                    <div class="kv-value">{display_sunset}</div>
                  </div>
                </div>
                </div>
            </div>
          </div>
    }
}
