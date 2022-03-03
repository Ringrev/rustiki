use zoon::{format, *, Element};
use zoon::*;

// --- --- The root page of the website --- ---

pub fn root() -> impl Element {
    Column::new()
        .s(Padding::new().top(50))
        .item(placeholder_text())
}

fn placeholder_text() -> impl Element {
    El::new()
        .s(Padding::top(Default::default(), 250))
        .child("Rustiki!").s(Font::new().size(40).color(hsluv!(18,100,48,100)))
        .s(Align::new().center_x())
        .s(Align::new().center_y())
}