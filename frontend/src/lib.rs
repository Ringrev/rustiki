//! Defines frontend modules and the entry point of the app.
use zoon::*;

// ------ ------
//    Modules
// ------ ------

mod app;
mod router;
mod pages;
mod connection;
mod elements;
mod rich_text;

// ------ ------
//     Start
// ------ ------

/// Starts app.
#[wasm_bindgen(start)]
pub fn start() {
     router::router();
     start_app("app", app::root);
}
