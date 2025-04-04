use yew::prelude::*;

#[function_component(FrontPage)]
pub fn front_page() -> Html{
    html!{
        <>
            <div class = "bg-image d-flex align-items-center justify-content-center">
                <p class = {classes!("paragraf")}>{"Petar Komatina"}</p>
                
            </div>
        
        </>
    }
}