use gloo_timers::callback::Timeout;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::{Callback, Html, InputEvent, TargetCast, function_component, html, use_effect_with, use_mut_ref, use_state};
#[function_component(ChatWindow)]
pub fn chat_window() -> Html{

    let question = use_state(String::new);
    let response = use_state(String::new);
    let is_loading = use_state(|| false);
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

    //





    html! {
        <div class="container mt-5">
            <div class="card p-3">

                <h3>{ "Claude Chat" }</h3>

                <div class="mb-3">
                    if *is_loading {
                        <p>{ "Loading..." }</p>
                    } else {
                        <p>{ (*response).clone() }</p>
                    }
                </div>

                <div class="d-flex gap-2">
                    <input
                        type="text"
                        class="form-control"
                        value={(*question).clone()}
                        oninput={on_input}
                        placeholder="Postavi pitanje..."
                    />

                    // // <button
                    // //     class="btn btn-primary"
                    // //     onclick={on_send}
                    // //     disabled={*loading}
                    // // >
                    //     { "Send" }
                    // </button>
                </div>

            </div>
        </div>
    }
}