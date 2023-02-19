use leptos::*;
use wasm_bindgen::prelude::wasm_bindgen;

mod app;
use app::*;

#[wasm_bindgen(start)]
pub fn main() {
    let _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|cx| {
        view! {
            cx, <App />
        }
    });
}
