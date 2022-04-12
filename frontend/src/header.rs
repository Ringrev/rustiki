use std::borrow::{Borrow, Cow};
use crate::{app, router::Route};
use zoon::{named_color::*, *};
use zoon::dominator::routing::go_to_url;
use shared::User;
use crate::app::{logged_user_name, PageName};
use crate::router::Route::Root;
use crate::router::router;

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
        .item(search_box())
        .item(button_row())
}

/// If clicked, user is sent to front page. Gets all articles from database too.
fn logo() -> impl Element {
    Link::new()
        .s(Font::new().size(50).weight(FontWeight::Bold))
        .label("Rustiki")
        .to(Route::Root)
        .on_click(app::reset_articles)
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

pub fn search() {
    app::articles().lock_mut().retain(|art| art.title.to_lowercase().contains(search_query().get_cloned().to_lowercase().as_str()));
}

#[static_ref]
fn search_query() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}

/// Sets the search query and if there is no search query, gets complete list of articles
pub fn set_search_query(query: String) {
    search_query().set(query);
    if search_query().get_cloned().eq("") {
        app::reset_articles();
    }
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
        .on_change(set_search_query)
        .label_hidden("New message text")
        .placeholder(Placeholder::new("Search for Wiki"))

}

fn search_button() -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Link::new()
        .s(Padding::all(20))
        .s(RoundedCorners::new().right(5))
        .s(Background::new().color_signal(hovered_signal.map_bool(|| GRAY_5, || GRAY_9)))
        .s(Font::new().color(GRAY_0).size(17))
        .s(Align::new().left())
        .s(RoundedCorners::new().right(25).left(25))
        .on_hovered_change(move |is_hovered| hovered.set(is_hovered))
        //.on_press(super::send_message)
        .on_click(search)
        .label("Search")
        .to(Route::Root)
}

fn button_row() -> impl Element {
    Row::new()
        .s(Align::new().bottom().right())
        .s(Spacing::new(6))
        // .item(Paragraph::new().content(logged_user_name()).s(Font::new().color(GRAY_9)))
        .item(Text::with_signal(logged_user_name().signal_cloned()))
        .item_signal(app::is_user_logged_signal().map_false(registration_button))
        .item_signal(app::is_user_logged_signal().map_false(log_in_button))
        .item_signal(app::is_user_logged_signal().map_true(log_out_button))
        .item_signal(app::is_user_logged_signal().map_true(new_article_button))
}

// fn logged_in_button_row() -> impl Element {
//     Row::new()
//         .s(Align::new().bottom().right())
//         .s(Spacing::new(6))
//         // .item()
//         .item(Paragraph::new().content(logged_user_name()))
//         .item(log_out_button())
//         .item(new_article_button())
// }

// fn username_text() -> impl Element {
//     Paragraph::new().content(logged_in_username())
// }

fn log_out_button() -> impl Element {
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
        .label("Log out")
        .on_click(app::log_out)
        .to(Route::Root)
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