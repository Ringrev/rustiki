use zoon::{format, *, Element, eprintln};
use zoon::*;
use zoon::named_color::GRAY_0;
use shared::{DownMsg, UpMsg, User};
use crate::{new_article_page, registration_page, log_in_page, router::{previous_route, router, Route}};
use crate::footer::footer;
use crate::connection::connection;
use crate::header::{header};

pub static USER_TOKEN: &str = "token";

// ------ page names ------

pub fn is_user_logged_signal() -> impl Signal<Item = bool> {
    logged_user().signal_ref(Option::is_some)
}

pub fn is_user_logged() -> bool {
    logged_user().map(Option::is_some)
}

#[static_ref]
pub fn logged_user() -> &'static Mutable<Option<User>> {
    Mutable::new(None)
}

#[static_ref]
pub fn logged_user_name() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}

pub fn log_out() {
    auth_token().set(None);
    logged_user().set(None);
    logged_user_name().set("".to_string());
}

#[static_ref]
pub fn auth_token() -> &'static Mutable<Option<String>> {
    Mutable::new(None)
}

pub fn set_logged_in_user_and_token(user: User) {
    logged_user().set(Some(user.clone()));
    logged_user_name().set(Some(user.clone()).unwrap().username);
    auth_token().set(Some(user.clone().auth_token));
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum PageName {
    Home,
    Registration,
    NewArticle,
    LogIn,
    Unknown,
}

// ------ content visible on all pages ------

pub fn root() -> impl Element {
    Column::new()
        .s(Height::screen())
        .item(header())//navbar placeholder
        .item(page())
        .item(footer()).s(Align::bottom(Default::default()))
}

// ------ front page content ------

fn front_page() -> impl Element {
    Column::new()
        .s(Padding::new().top(50))
        .item(placeholder_text())
}

fn placeholder_text() -> impl Element {
    El::new()
        // .s(Padding::top(Default::default(), 500))
        // .child("Rustiki!").s(Font::new().size(40).color(hsluv!(18,100,48,100)))
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
        PageName::LogIn => log_in_page::page().into_raw_element(),
    }))
}

#[static_ref]
fn page_name() -> &'static Mutable<PageName> {
    Mutable::new(PageName::Unknown)
}

pub fn set_page_name(new_page_name: PageName) {
    page_name().set_neq(new_page_name);
}