
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::api::api::ButtonContent;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub data: Callback<ButtonContent>,
}
#[function_component(InputButton)]
pub fn generate_button(props: &Props) -> Html {
    let input_value = use_state(|| ButtonContent::default());

    let on_change_input_value = {
        let input_value = input_value.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_dyn_into::<HtmlInputElement>();
            if let Some(input) = input {
                let bc = ButtonContent {
                    content: input.value(),
                };
                input_value.set(bc);
            }
        })
    };
    let on_click = {
        let data = props.data.clone();
        Callback::from(move |_event: MouseEvent| {
            data.emit((*input_value).clone());
        })
    };

    html! {
        <>
        <div class = {"position-absolute top-50 start-50 translate-middle bg-white bg-opacity-75 rounded-3 shadow-lg"}>
                    <div class="input-group">
                        <input
                        oninput={on_change_input_value}
                          type="text"
                          class="form-control"
                          placeholder="Enter city"
                        />
                        <button onclick = {on_click} class="futuristic-button" type="button">{"Search"}</button>
                      </div>
                </div>
        </>
    }
}
