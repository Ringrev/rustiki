use zoon::{Align, Background, Column, Element, hsluv, Padding, Paragraph, Placeholder, Row, Style, Styleable, Text, TextInput, Width};
use crate::app;


use crate::serde::__private::de::Content::Str;
use zoon::{*, named_color::*};

pub fn page() -> impl Element {
    Column::new()
        .s(Background::new().color(PURPLE_4))
        .item(picture())

}

fn picture() -> impl Element {
    Image::new()
        .url("https://www.catastrophicreations.com/wp-content/uploads/2021/02/IMG_7465-2glance.jpg%22")
        .description("A cat")
        .s(Width::new(600))
        .s(Padding::top(Default::default(), 50))
}




