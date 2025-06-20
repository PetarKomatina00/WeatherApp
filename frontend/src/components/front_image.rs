use std::rc::Rc;
use yew::prelude::*;

use crate::{api::api::ButtonContent, assets::utility, components::input_button::InputButton};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub data: Callback<ButtonContent>,
}
#[function_component(FrontImage)]
pub fn front_image(props: &Props) -> Html {
    let chosen_image = utility::get_image_related_to_width();
    let random_number: Rc<usize> = use_memo((), |_| utility::get_random_int(5.0));
    let random_number = *random_number;
    let custom_background_image = String::from(format!(
        "background-image: url('{}')",
        chosen_image[random_number]
    ));
    html! {
        <>
            <div style = {custom_background_image} class = {classes!("bg-image")}>
                <InputButton data = {props.data.clone()}/>
            </div>
        </>
    }
}
