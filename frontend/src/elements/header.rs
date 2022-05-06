//! Defines the header of the website.
use crate::app::logged_user_name;
use crate::elements::button;
use crate::pages::home_page;
use crate::router::router;
use crate::{app, router::Route};
use zoon::{named_color::*, *};

// ------ ------
//     States
// ------ ------

/// State of search query.
#[static_ref]
fn search_query() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}

// ------ ------
//     Commands
// ------ ------

/// Sets search query back to empty and sends user to front page.
fn on_logo_click() {
    set_search_query("".to_string());
    home_page::reset_articles();
}

/// Resets article vector, sends user to front page (if not already there),
/// and then searches all articles for query.
/// Searches articles' titles, authors, contributors and tags.
pub fn search() {
    home_page::reset_articles();
    router().go(Route::Home);
    let query = search_query().get_cloned().to_lowercase();
    home_page::articles().lock_mut().retain(|art| {
        art.title.to_lowercase().contains(&query)
            || art.author.to_lowercase().contains(&query)
            || search_inner_vec(art.tags.to_vec(), &query)
            || search_inner_vec(art.contributors.to_vec(), &query)
    });
}

/// Used to search a vector.
///
/// # Arguments
/// * `vec` - A vector of Strings to search through.
/// * `query` - A reference to a String word or phrase to search for.
pub fn search_inner_vec(vec: Vec<String>, query: &String) -> bool {
    let filtered: Vec<String> = vec
        .iter()
        .map(|tag| tag.to_lowercase())
        .filter(|tag| tag.contains(query.as_str()))
        .collect();
    if (filtered.len()) > 0 {
        true
    } else {
        false
    }
}

/// Sets the search query and if there is no search query, gets complete list of articles
pub fn set_search_query(query: String) {
    search_query().set(query);
    if search_query().get_cloned().eq("") {
        home_page::reset_articles();
    }
}

// ------ ------
//     View
// ------ ------

/// Returns a Row that works as header of the website.
pub fn header() -> impl Element {
    Row::new()
        .id("rustiki_header")
        .s(Height::new(100))
        .s(Background::new().color(GRAY_4))
        .s(Padding::all(20))
        .item(logo())
        .item(search_panel())
        .item(button_panel())
}

/// Returns a Row representing the logo of the website.
/// If clicked, user is sent to front page.
fn logo() -> impl Element {
    Row::new().item(
        Link::new()
            .s(Font::new().size(50).weight(FontWeight::Bold))
            .label("Rustiki")
            .to(Route::Home)
            .on_click(on_logo_click)
            .focus(true)
            .on_key_down_event(|event| event.if_key(Key::Enter, || router().go(Route::Home))),
    )
}

/// Returns a Row containing both the search_bar and the search_button.
fn search_panel() -> impl Element {
    Row::new()
        .s(Width::fill())
        .s(Align::new().center_x().center_y())
        .item(
            Row::new()
                .s(Align::new().center_x())
                .item(search_bar())
                .item(search_button())
                .s(Spacing::new(6)),
        )
}

/// Returns a TextInput that is used as a searchbar.
fn search_bar() -> impl Element {
    TextInput::new()
        .s(Align::new().center_x())
        .s(Padding::all(10))
        .s(Width::new(350).min(150))
        .s(Font::new().size(16))
        .s(RoundedCorners::new().right(15).left(15))
        .focus(true)
        .on_change(set_search_query)
        .label_hidden("New message text")
        .placeholder(Placeholder::new("Search for wiki"))
        .on_key_down_event(|event| event.if_key(Key::Enter, search))
        .text_signal(search_query().signal_cloned())
}

/// Returns a Row containing buttons to be displayed in header.
/// If logged in, you see a "log out" button.
/// If logged out, you see a "registration" button and "login" button.
fn button_panel() -> impl Element {
    Row::new()
        .s(Align::new().center_y().right())
        .s(Width::min(Default::default(), 200))
        .s(Spacing::new(6))
        .item(
            Row::new()
                .s(Align::new().right())
                .s(Spacing::new(6))
                .item(
                    Paragraph::new()
                        .content_signal(logged_user_name().signal_cloned())
                        .s(Align::new().right()),
                )
                .item_signal(app::is_user_logged_signal().map_true(log_out_button)),
        )
        .item_signal(app::is_user_logged_signal().map_false(registration_button))
        .item_signal(app::is_user_logged_signal().map_false(log_in_button))
}

/// Returns a Link element that looks like a Button from "button" module.
fn search_button() -> impl Element {
    button::header_button("search_button", "Search", Route::Home, Some(search))
}

/// Returns a Link element that looks like a Button from "button" module.
fn log_out_button() -> impl Element {
    button::header_button("log_out_button", "Log out", Route::Home, Some(app::log_out))
}

/// Returns a Link element that looks like a Button from "button" module.
fn log_in_button() -> impl Element {
    button::header_button("log_in", "Log in", Route::LogIn, None)
}

/// Returns a Link element that looks like a Button from "button" module.
fn registration_button() -> impl Element {
    button::header_button(
        "registration_button",
        "Registration",
        Route::Registration,
        None,
    )
}
