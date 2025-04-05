use std::{fmt::format, rc::Rc};

use web_sys::{js_sys::{Math::{self, random}, Reflect::get}, window};
use yew::prelude::*;
use gloo::console::log;

use wasm_bindgen::prelude::*;
use crate::assets::utility;

#[function_component(FrontImage)]
pub fn front_image() -> Html{

    let chosen_image = utility::get_image_related_to_width();
    let random_number: Rc<usize> = use_memo((), |_| {
        utility::get_random_int(5.0)
    }); 
    let random_number = *random_number;
    let custom_background_image = String::from(format!("background-image: url('{}')", chosen_image[random_number]));
    html!{
        <>
            <div style = {custom_background_image} class = {classes!("bg-image")}>
                <p >{"Petar Komatina"}</p>
            </div>
        
        </>
    }
}