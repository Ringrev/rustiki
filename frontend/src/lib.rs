use zoon::{format, *};

mod app;

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
     start_app("app", app::root);
}
