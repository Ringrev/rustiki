use zoon::{format, *};

mod app;
mod router;
mod header;


// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
     router::router();
     start_app("app", app::root);
}
