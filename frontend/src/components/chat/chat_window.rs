

use time::Duration;
use gloo_timers::callback::Timeout;
use shared::{ClaudeResponse, WeatherData};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::{Callback, Event, Html, InputEvent, KeyboardEvent, MouseEvent, Properties, SubmitEvent, TargetCast, function_component, html, use_effect_with, use_mut_ref, use_state};
use gloo_net::http::Request;
use yew_notifications::{Notification, NotificationType, use_notification};
use crate::{api::chat::send_chat_message, assets::markdown::markdown_to_html};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub weather_data: Option<WeatherData>,
}
#[function_component(ChatWindow)]
pub fn chat_window(props: &Props) -> Html{

    let question = use_state(String::new);
    let response = use_state(|| ClaudeResponse::default());
    let is_claude_loading = use_state(|| false);
    let debouncer_timer = use_mut_ref(|| None::<Timeout>);
    let use_mcp_weather = use_state(|| false);
    
    let notifications_manager = use_notification::<Notification>();


    let on_use_mcp_weather_change = {
        let use_mcp_weather = use_mcp_weather.clone();

        Callback::from(move |e: Event|{
            let input: HtmlInputElement = e.target_unchecked_into();

            use_mcp_weather.set(input.checked());
        })
    };
    let on_input = {
        let debouncer_timer = debouncer_timer.clone();
        let question = question.clone();
        Callback::from(move |event: InputEvent| {
            let question = question.clone();
            let timeout = Timeout::new(300, move || {
                let input: HtmlInputElement = event.target_unchecked_into();
                question.set(input.value());
                web_sys::console::log_1(&JsValue::from_str(&(input.value()).to_string()));
            });
            *debouncer_timer.borrow_mut() = Some(timeout);

        })
    };


    let on_send = {
        let question = question.clone();
        let response = response.clone();
        let is_claude_loading = is_claude_loading.clone();
        let weather_data = props.weather_data.clone();
        let notifications_manager = notifications_manager.clone();
        let use_mcp_weather = use_mcp_weather.clone();
        
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let question = question.clone();
            let response = response.clone();
            let is_claude_loading = is_claude_loading.clone();
            let weather_data = weather_data.clone();
            let notifications_manager = notifications_manager.clone();
            let use_mcp_weather = use_mcp_weather.clone();

            is_claude_loading.set(true);
            spawn_local(async move {
                notifications_manager.spawn(Notification::new(NotificationType::Info, String::from("Zahtev Claude-u"), String::from("Uspesno poslat"), Duration::seconds(5)));
                let question_string = (*question).clone();
                let result = send_chat_message(&question_string, (*use_mcp_weather).clone(), weather_data).await;

                match result{
                    Ok(claude_response) => {
                        response.set(claude_response);
                        is_claude_loading.set(false);
                        question.set(String::new())
                    },
                    Err(e) => {
                        is_claude_loading.set(false);
                        web_sys::console::log_1(&format!("Question: {}, MCP: {}", *question, *use_mcp_weather).into());
                        notifications_manager.spawn(Notification::new(NotificationType::Error, String::from("Zahtev zavrsen"), String::from(format!("{}", e)), Duration::seconds(5)));
                    }
                }
            });
            
        })
    };
    html! {
        <div class="container mt-5">
            <div class="card p-3">

                <h3>{ "Claude Chat" }</h3>

                <div class="mcp-toggle-wrapper">
                    <span class="mcp-toggle-text">
                        {"MCP Tool"}
                    </span>

                    <input
                        id="mcp-weather-toggle"
                        class="mcp-toggle-input"
                        type="checkbox"
                        checked={*use_mcp_weather}
                        onchange={on_use_mcp_weather_change}
                    />

                    <label
                        class="mcp-toggle"
                        for="mcp-weather-toggle"
                    >
                    </label>
                </div>

                <div class="mb-3">
                    if *is_claude_loading {
                        <p>{ "Loading..." }</p>
                    } else {
                        <p>{ markdown_to_html(&response.answer.clone())}</p>
                    }
                </div>

                <form onsubmit={on_send}>
                    <div class="d-flex gap-2">
                        <input
                            type="text"
                            class="form-control"
                            value={(*question).clone()}
                            oninput={on_input}
                            placeholder="Postavi pitanje..."
                        />

                        <button
                            type="submit"
                            class="btn btn-primary"
                            disabled={*is_claude_loading}
                        >
                            {
                                if *is_claude_loading {
                                    html! {
                                        <>
                                            <span
                                                class="spinner-border spinner-border-sm me-2"
                                                role="status"
                                            ></span>
                                        </>
                                    }
                                } else {
                                    html! {
                                        {"Send"}
                                    }
                                }
                            }
                        </button>
                    </div>
                </form>

            </div>
        </div>
    }
}