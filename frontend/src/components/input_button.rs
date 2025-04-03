
use gloo::console::log;
use yew::prelude::*;
use web_sys::{console::log, HtmlInputElement};

use crate::api::api::ButtonContent;



#[derive(Properties, PartialEq)]
pub struct Props{
    pub data: Callback<ButtonContent>
}
#[function_component(InputButton)]
pub fn generate_button(props: &Props) -> Html{
    let input_value = use_state(|| ButtonContent::default());

    let on_change_input_value = {
        let input_value = input_value.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_dyn_into::<HtmlInputElement>();
            if let Some(input) = input {
                let bc = ButtonContent {
                    content: input.value()
                };
                input_value.set(bc);
            }
        })
    };
    let on_click = {
        let data = props.data.clone();
        Callback::from(move |_event: MouseEvent|{
            data.emit((*input_value).clone());
        })
    };

    html! {
        <>
            <div style = "display:flex; border:1px solid #bbb">
                <label for="cautious-input">
                    { "Moje dugme:" }
                    <input oninput={on_change_input_value}
                        id="cautious-input"
                        type="text"
                        
                    />
                </label>
            </div>
            <div>
                <button onclick={on_click}>
                {"Click me"}
                </button>
            </div>
        </>
    }
}
