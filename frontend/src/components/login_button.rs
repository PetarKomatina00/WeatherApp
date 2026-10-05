use gloo::console::log;
use web_sys::window;
use yew::prelude::*;

use crate::assets::config::AppConfig;

#[derive(Clone, PartialEq)]
pub enum Auth0Action {
    Login,
    Logout,
}

#[derive(Properties, PartialEq)]
pub struct Auth0Props {
    pub action: Auth0Action,
    pub onclick: Callback<MouseEvent>,
}

#[function_component(LoginButton)]
pub fn login_button(props: &Auth0Props) -> Html {

    let config = use_context::<AppConfig>().expect("Could not load config.json");
    let backend_url = config.backend_url;
    let mut url = String::default();
    let mut label = String::default();
    match props.action {
        Auth0Action::Login => {
            url = format!("{}/auth0/login", backend_url);
            label = format!("Log in");
        }
        Auth0Action::Logout => {
            url = format!("{}/auth0/logout", backend_url);
            label = format!("Log out");
        }
    };
    web_sys::console::log_1(&format!("Auth0 login URL: {}", url).into());

    let on_click = Callback::from(move |_e: MouseEvent| {
        log!(&format!("Button clicked"));
        window()
            .unwrap()
            .location()
            .set_href(&url)
            .expect("Something went wront with redirection");
    });

    html! {
        <button class = {classes!("futuristic-button")}onclick = {on_click} type = "button">{label}</button>
    }
}
