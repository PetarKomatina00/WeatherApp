use gloo_net::http::Request;
use serde::Deserialize;
use yew::Properties;


#[derive(Debug, Clone, PartialEq, Deserialize, Properties)]
pub struct AppConfig{
    pub backend_url: String
}

pub async fn load_config() -> Result<AppConfig, gloo_net::Error> {
    Request::get("/config.json")
        .send()
        .await?
        .json::<AppConfig>()
        .await
}