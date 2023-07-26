mod app;
mod game;

use app::App;
use yew::prelude::*;
use lazy_static::lazy_static;

pub fn window() -> web_sys::Window {
    return web_sys::window().unwrap();
}
pub fn document() -> web_sys::Document {
    return window().document().unwrap();
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
