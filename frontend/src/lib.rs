use zoon::*;

// ------ ------
//    modules
// ------ ------

mod app;
mod router;
mod pages;
mod registration_page;
mod new_article_page;
mod header;
mod log_in_page;
mod footer;
mod connection;
mod view_article_page;
mod edit_article_page;
mod tags;
mod elements;

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
     router::router();
     start_app("app", app::root);
}
