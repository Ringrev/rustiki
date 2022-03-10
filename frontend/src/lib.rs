use zoon::{format, *};

// ------ ------
//    modules
// ------ ------

mod app;
mod router;
mod registration_page;
mod new_article_page;
mod header;

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
     router::router();
     start_app("app", app::root);
}
