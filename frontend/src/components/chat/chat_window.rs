use gloo_timers::callback::Timeout;
use shared::AskResponse;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::{Callback, Event, Html, InputEvent, KeyboardEvent, MouseEvent, SubmitEvent, TargetCast, function_component, html, use_effect_with, use_mut_ref, use_state};
use gloo_net::http::Request;
use crate::{api::chat::send_chat_message, assets::markdown::markdown_to_html};
#[function_component(ChatWindow)]
pub fn chat_window() -> Html{

    let question = use_state(String::new);
    let response = use_state(|| AskResponse::default());
    let is_claude_loading = use_state(|| false);
    let debouncer_timer = use_mut_ref(|| None::<Timeout>);
    

    let on_input = {
        let debouncer_timer = debouncer_timer.clone();
        let question = question.clone();
        Callback::from(move |event: InputEvent| {
            let question = question.clone();
            let timeout = Timeout::new(500, move || {
                let input: HtmlInputElement = event.target_unchecked_into();
                question.set(input.value());
                web_sys::console::log_1(&JsValue::from_str(&(input.value()).to_string()));
            });
            *debouncer_timer.borrow_mut() = Some(timeout);

        })
    };

    let onkeydown = {
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter"{

            }

        })
    };

    let on_send = {
        let question = question.clone();
        let response = response.clone();
        let is_claude_loading = is_claude_loading.clone();
        
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let question = question.clone();
            let response = response.clone();
            let is_claude_loading = is_claude_loading.clone();

            is_claude_loading.set(true);
            spawn_local(async move {
                let question_string = (*question).clone();
                let result = send_chat_message(&question_string).await;

                match result{
                    Ok(claude_response) => {
                        response.set(claude_response);
                        is_claude_loading.set(false);
                        question.set(String::new())
                    },
                    Err(e) => {
                        
                    }
                }
            });
            
        })
    };
    html! {
        <div class="container mt-5">
            <div class="card p-3">

                <h3>{ "Claude Chat" }</h3>

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