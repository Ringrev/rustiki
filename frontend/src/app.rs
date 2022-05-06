//! Defines root element of app and sets the page displayed as decided by the routing.
//! Also keeps track of whether a user is logged in or not.
use crate::elements::footer::footer;
use crate::elements::header::header;
use crate::pages::home_page::get_article_from_route;
use crate::{
    pages::{
        create_article_page, edit_article_page, home_page, log_in_page, registration_page,
        view_article_page,
    },
    router::{router, Route},
};
use shared::LocalUser;
use zoon::{Element, *};

// ------ ------
//    States
// ------ ------

/// State of logged in user. Initiated with no value.
/// Set to LocalUser struct when user logs in.
#[static_ref]
pub fn logged_in_user() -> &'static Mutable<Option<LocalUser>> {
    Mutable::new(None)
}

/// State of logged in user's username. Initialized with empty String.
#[static_ref]
pub fn logged_user_name() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}

/// The state of authentication token. Initialized with no value.
/// Set to a String token when a user is logged in.
#[static_ref]
pub fn auth_token() -> &'static Mutable<Option<String>> {
    Mutable::new(None)
}

// The state of the current page. Initialized with unknown page.
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

/// Sets user info and token when user logs in. Called from connection.rs.
pub fn set_logged_in_user_and_token(user: LocalUser) {
    logged_in_user().set(Some(user.clone()));
    logged_user_name().set(Some(user.clone()).unwrap().username);
    auth_token().set(Some(user.clone().auth_token));
    router().go(Route::Home);
}

/// Sets page to display
pub fn set_page_name(new_page_name: PageName) {
    page_name().set_neq(new_page_name);
}

// ------ ------
//     Types
// ------ ------

/// Represents the different pages available on the website.
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

/// The root element of the website.
/// This decides the general layout of the website, except from the page content.
/// All articles in database are loaded when you first load website.
pub fn root() -> impl Element {
    home_page::get_articles();
    Column::new()
        .s(Height::screen())
        .s(Width::fill())
        .item(header())
        .item(
            Column::new()
                .s(Height::fill())
                .s(Scrollbars::both())
                .item(page())
                .item(footer()),
        )
}

/// When the route changes, this function changes the displayed page.
fn page() -> impl Element {
    El::new().child_signal(page_name().signal().map(|page_name| match page_name {
        PageName::Home => home_page::page().into_raw_element(),
        PageName::Unknown => El::new().child("404").into_raw_element(),
        PageName::EditArticle => edit_article_page::page().into_raw_element(),
        PageName::NewArticle => create_article_page::page().into_raw_element(),
        PageName::Registration => registration_page::page().into_raw_element(),
        PageName::LogIn => log_in_page::page().into_raw_element(),
        PageName::ViewArticle => {
            view_article_page::set_view_article(get_article_from_route());
            view_article_page::page().into_raw_element()
        }
    }))
}
