
use wasm_bindgen_futures::spawn_local;
use yew_notifications::{Notification, NotificationFactory, NotificationsProvider};
use assets::utility::{switch, Route};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::assets::config::{AppConfig, load_config};
mod api;
mod assets;
mod components;
mod pages;
mod models;
#[function_component]
fn App(props: &AppConfig) -> Html {

    let component_creator = NotificationFactory;
    html! {
        <ContextProvider<AppConfig> context={props.clone()}>
            <NotificationsProvider<Notification, NotificationFactory> {component_creator}>
                <BrowserRouter>
                    <Switch<Route> render={switch} />
                </BrowserRouter>
            </NotificationsProvider<Notification, NotificationFactory>>
        </ContextProvider<AppConfig>>

    }
}
fn main() {
    spawn_local(async {
        let config: assets::config::AppConfig = load_config()
        .await.expect("Failed to load config.json");

        yew::Renderer::<App>::with_props(config).render();
    });
}
