use zoon::{*, Element};
use shared::LocalUser;
use crate::{ router::{router, Route}, pages::{home_page, log_in_page, registration_page, view_article_page, create_article_page, edit_article_page}};
use crate::elements::footer::footer;
use crate::elements::header::header;

// ------ ------
//    States
// ------ ------

/// State of logged in user
#[static_ref]
pub fn logged_in_user() -> &'static Mutable<Option<LocalUser>> {
    Mutable::new(None)
}

/// The username of the logged in user
#[static_ref]
pub fn logged_user_name() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}

/// Holds the auth token when a user is logged in
#[static_ref]
pub fn auth_token() -> &'static Mutable<Option<String>> {
    Mutable::new(None)
}

#[static_ref]
fn page_name() -> &'static Mutable<PageName> {
    Mutable::new(PageName::Unknown)
}

// ------ ------
//    Signals
// ------ ------

/// Used to decide if user sees logged in or out view of site
pub fn is_user_logged_signal() -> impl Signal<Item = bool> {
    logged_in_user().signal_ref(Option::is_some)
}

// ------ ------
//    Commands
// ------ ------

/// Removes token and user info so the user is logged out of the site
pub fn log_out() {
    auth_token().set(None);
    logged_in_user().set(None);
    logged_user_name().set("".to_string());
}

/// Sets user info and token when user logs in. Called from connection.rs
pub fn set_logged_in_user_and_token(user: LocalUser) {
    logged_in_user().set(Some(user.clone()));
    logged_user_name().set(Some(user.clone()).unwrap().username);
    auth_token().set(Some(user.clone().auth_token));
    router().go(Route::Home);
}

pub fn set_page_name(new_page_name: PageName) {
    page_name().set_neq(new_page_name);
}

// ------ ------
//     Types
// ------ ------

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum PageName {
    Home,
    Registration,
    NewArticle,
    LogIn,
    Unknown,
    EditArticle,
    ViewArticle,
}

// ------ ------
//     View
// ------ ------

pub fn root() -> impl Element {
    Column::new()
        .s(Height::screen())
        .s(Width::fill())
        .item(header()).s(Align::top(Default::default())) //navbar placeholder
        .item(page())
        .item(footer()).s(Align::bottom(Default::default()))
}

fn page() -> impl Element {
    El::new().child_signal(page_name().signal().map(|page_name| match page_name {
        PageName::Home => home_page::page().into_raw_element(),
        PageName::Unknown => El::new().child("404").into_raw_element(),
        PageName::EditArticle => edit_article_page::page().into_raw_element(),
        PageName::NewArticle => create_article_page::page().into_raw_element(),
        PageName::Registration => registration_page::page().into_raw_element(),
        PageName::LogIn => log_in_page::page().into_raw_element(),
        PageName::ViewArticle => view_article_page::page().into_raw_element(),
    }))
}