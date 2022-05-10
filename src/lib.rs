use wasm_bindgen::prelude::*;

pub mod app;
pub mod login;

pub const API: &'static str = "https://impostro-api.herokuapp.com/";
pub const API_VALIDATE_SESSION: &'static str = "https://impostro-api.herokuapp.com/validate_session_id";

#[wasm_bindgen]
pub fn start() {
    yew::Renderer::<app::App>::new().render();
}