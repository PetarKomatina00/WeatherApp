use crate::components::login::{Auth0Action, LoginButton};
use yew::{classes, function_component, html, Callback, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct LoginModalProps {
    pub login_model_open: Callback<()>,
}

#[function_component(LoginModal)]
pub fn login_modal(props: &LoginModalProps) -> Html {
    let on_close = {
        let cb = props.login_model_open.clone();
        Callback::from(move |_| cb.emit(()))
    };
    html! {
        <div class="modal-backdrop">
            <div class="modal-window">
                <div class="modal-header">
                    <h2>{ "Log in with Auth0 to use the App" }</h2>
                    <button class="close-button" onclick={on_close.clone()}>{ "×" }</button>
                </div>
                <div class="modal-body">
                    // insert your login form fields here
                    <LoginButton action = {Auth0Action::Login} onclick = {Callback::from(move |_| {})}/>
                </div>
            </div>
        </div>
    }
}
