use chrono::NaiveDateTime;
use gloo::console::log;
use gloo_net::http::Request;
use serde::Deserialize;
use uuid::Uuid;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::hooks::use_navigator;

use crate::{assets::utility::Route, components::spinner::Spinner};
#[derive(Debug, Deserialize)]
pub struct ApiLogs {
    pub logs_id: Uuid,
    pub trace_id: String,
    pub func_call: String,
    pub created_at: NaiveDateTime,
    pub status: String,
    pub location: Option<String>,
    pub error_message: Option<String>,
}

#[function_component(AdminPage)]
pub fn admin_page() -> Html {
    let navigator = use_navigator().unwrap();
    let api_logs: UseStateHandle<Vec<ApiLogs>> = use_state(|| Vec::new());

    let api_logs_handle = api_logs.clone();
    let api_logs_cleanup = api_logs.clone();

    let is_loading = use_state(|| false);
    let is_loading_handle = is_loading.clone();
    use_effect_with((), move |_| {
        let api_logs_handle = api_logs_handle.clone();
        let is_loading_handle = is_loading_handle.clone();
        spawn_local(async move {
            let api_logs_handle = api_logs_handle.clone();
            //let is_loading_handle = is_loading_handle.clone();\

            is_loading_handle.set(true);
            let url = format!("http://127.0.0.1:8000/api/logs?limit=10");
            let response = Request::get(&url)
                .header("Accept", "applicaton/json")
                .send()
                .await;

            match response {
                Ok(r) if r.ok() => {
                    is_loading_handle.set(false);
                    let api_logs: Vec<ApiLogs> =
                        r.json().await.expect("Cannot convert JSON to Apilogs");
                    api_logs_handle.set(api_logs);
                    log!("Everything is set");
                }
                _ => {}
            }
        });
        move || {
            api_logs_cleanup.set(Vec::new());
        }
    });

    let move_to_home = Callback::from(move |_| {
        navigator.push(&Route::Home);
    });
    //Fetch the data
    html! {
        <>
        if *is_loading{
            <Spinner/>
        }
            if !api_logs.is_empty(){
                <button class="apple-button" id="admin-button" onclick = {move_to_home}>{"Back to home"}</button>
                <div class="table-responsive">
                    <table class="table table-striped table-hover">
                        <thead class="table-dark">
                            <tr>
                                <th>{"trace_id"}</th>
                                <th>{"func_call"}</th>
                                <th>{"Created at"}</th>
                                <th>{"Status"}</th>
                                <th>{"Location"}</th>
                                <th>{"Error message"}</th>
                            </tr>
                        </thead>
                        <tbody>
                            { for (*api_logs).iter().map(|log| html! {
                                <tr>
                                    <td>{ &log.trace_id }</td>
                                    <td>{ &log.func_call }</td>
                                    <td>{ {format!("{}", log.created_at)} }</td>
                                    <td>{ &log.status }</td>
                                    <td>{ &log.location }</td>
                                    <td>{ &log.error_message }</td>
                                </tr>
                            }) }
                        </tbody>
                    </table>
                </div>
            }
            else{
                {"asdasdsad"}
            }
        </>
    }
}

/*    pub logs_id: Uuid,
pub trace_id: String,
pub func_call: String,
pub created_at: NaiveDateTime,
pub status: String,
pub location: Option<String>,
pub error_message: Option<String>, */
