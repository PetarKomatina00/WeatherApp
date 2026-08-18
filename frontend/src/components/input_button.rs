
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::html;

use crate::api::api::ButtonContent;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub data: Callback<ButtonContent>,
    pub is_loading: bool
}
#[function_component(InputButton)]
pub fn generate_button(props: &Props) -> Html {
    const DEFAULT_PLACEHOLDER: &str = "Enter city";

    let input_value = use_state(|| ButtonContent::default());

    {
        let input_value = input_value.clone();

        use_effect_with(props.is_loading, move |is_loading| {
            if !*is_loading {
                input_value.set(ButtonContent {
                    content: String::new(),
                });
            }
        });
    }
    let on_change_input_value = {
        let input_value = input_value.clone();

        Callback::from(move |e: InputEvent| {
            let input = e.target_dyn_into::<HtmlInputElement>();

            if let Some(input) = input {
                input_value.set(ButtonContent {
                    content: input.value(),
                });
            }
        })
    };

    let on_click = {
        let data = props.data.clone();
        let input_value = input_value.clone();

        Callback::from(move |_event: MouseEvent| {
            data.emit((*input_value).clone());
        })
    };

    let onkeydown = {
        let data = props.data.clone();
        let input_value = input_value.clone();

        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                data.emit((*input_value).clone());
            }
        })
    };

    html! {
        <div class="position-absolute top-50 start-50 translate-middle bg-white bg-opacity-75 rounded-3 shadow-lg">
            <div class="input-group">

                <input
                    oninput={on_change_input_value}
                    type="text"
                    class="form-control"
                    {onkeydown}
                    placeholder={DEFAULT_PLACEHOLDER}
                    value={(*input_value).content.clone()}
                />

                <button
                    onclick={on_click}
                    class="futuristic-button"
                    type="button"
                >
                    if !props.is_loading {
                        {"Search"}
                    } else {
                        <span
                            class="spinner-border spinner-border-sm me-2"
                            role="status"
                        />
                    }
                </button>

            </div>
        </div>
    }
}