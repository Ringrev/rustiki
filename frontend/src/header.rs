use std::borrow::Cow;
use crate::{app, router::Route};
use zoon::{named_color::*, *};
use zoon::dominator::routing::go_to_url;
use crate::app::PageName;

// ------ ------
//     View
// ------ ------

pub fn header() -> impl Element {
    Row::new()
        .s(Background::new().color(GRAY_4))
        .s(Spacing::new(10))
        .s(Padding::all(20))
        .s(Borders::new())
        .item(logo())
        // .item(create_new_article())
        .item(search_box())
        .item(buttons_row())
}

// fn create_new_article() -> impl Element {
//     Link::new()
//         .to(Route::NewArticle)
//         .s(Font::new().size(16).color(GRAY_0))
//         .label("Create new article")
//         .s(Padding::new().x(30))
//         .s(Padding::top(Default::default(), 20))
// }

fn logo() -> impl Element {
    // Image::new()
    //     .url("https://www.catastrophicreations.com/wp-content/uploads/2021/02/IMG_7465-2glance.jpg%22")
    //     .description("A cat")
    //     .s(Width::new(200))

    Link::new()
        .to(Route::Root)
        .s(Font::new().size(50).weight(FontWeight::Bold))
        .label("Rustiki")
}

fn link(label: &str, route: Route) -> impl Element {
    Link::new()
        .s(Font::new().color(BLUE_4).line(FontLine::new().underline()))
        .label(label)
        .to(route)
}

fn search_box() -> impl Element {
    Row::new()
        .s(Align::new().center_x().center_y())
        .item(search_bar())
        .s(Spacing::new(6))
        .item(search_button())

}

fn search_bar() -> impl Element {
    TextInput::new()
        .s(Align::new().center_x())
        .s(Padding::all(10))
        .s(RoundedCorners::new().left(5))
        .s(Width::fill().min(350).max(400))
        .s(Font::new().size(16))
        .s(RoundedCorners::new().right(25).left(25))
        .focus(true)
       // .on_change(super::set_new_message_text)
        .label_hidden("New message text")
        .placeholder(Placeholder::new("Search for Wiki"))

}

fn search_button() -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Padding::all(20))
        .s(RoundedCorners::new().right(5))
        .s(Background::new().color_signal(hovered_signal.map_bool(|| GRAY_5, || GRAY_9)))
        .s(Font::new().color(GRAY_0).size(17))
        .s(Align::new().left())
        .s(RoundedCorners::new().right(25).left(25))
        .on_hovered_change(move |is_hovered| hovered.set(is_hovered))
        //.on_press(super::send_message)
        .label("Search")
}


fn buttons_row() -> impl Element {
    Row::new()
        .s(Align::new().bottom().right())
        .s(Spacing::new(6))
        .item(registration_button())
        .item(log_in_button())
        .item(new_article_button())
}


fn log_in_button() -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    // Commented out button because could not figure out how to send to Route without a "Link" element.
    // We can change this back to button if we figure it out. Nothing else changed
    // Button::new()
    Link::new()
        .s(Font::new().size(20).color(GRAY_0))
        .s(Align::new().right().bottom())
        .s(Spacing::new(20))
        .s(RoundedCorners::new().right(25).left(25))
        .s(Background::new()
            .color_signal(hovered_signal.map_bool(|| GRAY_5, || GRAY_9)))
        .s(Padding::all(17))
        .on_hovered_change(move |is_hovered| hovered.set(is_hovered))
        .label("Log in")
        .to(Route::LogIn)
}


fn new_article_button() -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    // Commented out button because could not figure out how to send to Route without a "Link" element.
    // We can change this back to button if we figure it out. Nothing else changed
    // Button::new()
    Link::new()
        .s(Font::new().size(20).color(GRAY_0))
        .s(Align::new().right().bottom())
        .s(Spacing::new(20))
        .s(RoundedCorners::new().right(25).left(25))
        .s(Background::new()
            .color_signal(hovered_signal.map_bool(|| GRAY_5, || GRAY_9)))
        .s(Padding::all(17))
        .on_hovered_change(move |is_hovered| hovered.set(is_hovered))
        .label("Create New Article")
        .to(Route::NewArticle)
}

fn registration_button() -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    // Commented out button because could not figure out how to send to Route without a "Link" element.
    // We can change this back to button if we figure it out. Nothing else changed
    // Button::new()
    Link::new()
        .s(Font::new().size(20).color(GRAY_0))
        .s(Align::new().right().bottom())
        .s(Spacing::new(20))
        .s(RoundedCorners::new().right(25).left(25))
        .s(Background::new()
            .color_signal(hovered_signal.map_bool(|| GRAY_5, || GRAY_9)))
        .s(Padding::all(17))
        .on_hovered_change(move |is_hovered| hovered.set(is_hovered))
        .label("Registration")
        .to(Route::Registration)
}