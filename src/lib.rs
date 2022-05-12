use wasm_bindgen::prelude::*;

pub mod app;
pub mod login;

pub const API_WS: &'static str = "wss://impostro-api.herokuapp.com/ws/";

#[wasm_bindgen]
pub fn start() {
    yew::Renderer::<app::App>::new().render();
}