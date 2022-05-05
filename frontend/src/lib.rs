//! Defines frontend modules and the entry point of the app.

use zoon::*;

// ------ ------
//    Modules
// ------ ------

pub mod app;
pub mod connection;
pub mod elements;
pub mod pages;
pub mod router;

// ------ ------
//     Start
// ------ ------

/// Starts app.
#[wasm_bindgen(start)]
pub fn start() {
    router::router();
    start_app("app", app::root);
}
