use zoon::{format, *, Element};
use zoon::*;
use crate::{new_article_page, registration_page, header::header, router::{previous_route, router, Route}};
use crate::footer::footer;

// ------ page names ------

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum PageName {
    Home,
    Registration,
    NewArticle,
    Unknown,
}

// ------ content visible on all pages ------

pub fn root() -> impl Element {
    Column::new()
        .item(header())//navbar placeholder
        .item(page())
        .item(footer())
}

// ------ front page content ------

fn front_page() -> impl Element {
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

// ------ page routing ------

fn page() -> impl Element {
    El::new().child_signal(page_name().signal().map(|page_name| match page_name {
        PageName::Home => front_page().into_raw_element(),
        PageName::Unknown => El::new().child("404").into_raw_element(),
        PageName::NewArticle => new_article_page::page().into_raw_element(),
        PageName::Registration => registration_page::page().into_raw_element(),
    }))
}

#[static_ref]
fn page_name() -> &'static Mutable<PageName> {
    Mutable::new(PageName::Unknown)
}

pub fn set_page_name(new_page_name: PageName) {
    page_name().set_neq(new_page_name);
}