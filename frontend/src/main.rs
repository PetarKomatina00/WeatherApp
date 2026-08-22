
use yew_notifications::{Notification, NotificationFactory, NotificationsProvider};
use assets::utility::{switch, Route};
use yew::prelude::*;
use yew_router::prelude::*;
mod api;
mod assets;
mod components;
mod pages;
mod models;
#[function_component]
fn App() -> Html {
    let component_creator = NotificationFactory;
    html! {
            <NotificationsProvider<Notification, NotificationFactory> {component_creator}>
                <BrowserRouter>
                    <Switch<Route> render={switch} />
                </BrowserRouter>
            </NotificationsProvider<Notification, NotificationFactory>>

    }
}
fn main() {
    yew::Renderer::<App>::new().render();
}
