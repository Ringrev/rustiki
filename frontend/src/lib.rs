use zoon::*;

// ------ ------
//    Modules
// ------ ------

mod app;
mod connection;
mod elements;
mod pages;
mod router;

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    router::router();
    start_app("app", app::root);
}
