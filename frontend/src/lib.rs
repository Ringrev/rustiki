use zoon::*;

// ------ ------
//    Modules
// ------ ------

mod app;
mod router;
mod pages;
mod connection;
mod elements;

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
     router::router();
     start_app("app", app::root);
}
