use gloo::console::log;
use web_sys::window;
use yew::prelude::*;

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
    let mut url = String::default();
    let mut label = String::default();
    match props.action {
        Auth0Action::Login => {
            url = format!("http://127.0.0.1:8000/auth0/login");
            label = format!("Log in");
        }
        Auth0Action::Logout => {
            url = format!("http://127.0.0.1:8000/auth0/logout");
            label = format!("Log out");
        }
    };

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
