use zoon::{format, *, Element, eprintln};
use zoon::*;
use zoon::named_color::GRAY_0;
use shared::{Article, DownMsg, UpMsg, User};
use crate::{new_article_page, registration_page, header::header, log_in_page, router::{previous_route, router, Route}};
use crate::footer::footer;
use crate::connection::connection;

// ------ page names ------

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
        .item(header())//navbar placeholder
        .item(page())
        .item(footer())

}

#[static_ref]
fn user() -> &'static Mutable<String> {
    Mutable::new("no user".to_string())
}
fn articles() -> &'static Mutable<Vec<Article>> {
    Mutable::new("no article".to_string())
}

pub fn set_user(usr: User) {
    user().set(usr.email.to_string());
}

pub fn test_login() {
    Task::start(async {
        let msg = UpMsg::Login {
            email: "linda@linda".to_string(),
            password: "oigiojgoie".to_string(),
        };
        if let Err(error) = connection().send_up_msg(msg).await {
            let error = error.to_string();
            // eprintln!("login request failed: {}", error);
        }
    })
}

pub fn set_article(rtcl: Article) {
    article().set(rtcl.id.to_string());
}

pub fn test_get_article() {
    Task::start(async {
        let msg = UpMsg::Article {
            id: "artikkel ID ".to_string(),
        };
        if let Err(error) = connection().send_up_msg(msg).await {
            let error = error.to_string();
            // eprintln!("login request failed: {}", error);
        }
    })
}

// ------ front page content ------

fn front_page() -> impl Element {
    Column::new()
        .s(Padding::new().top(50))
        .item(placeholder_text())
        .item(Button::new()
            .label("Get user")
            .s(Background::new().color(GRAY_0))
            .on_press(test_login))
        .item(Button::new()
            .label("Get article")
            .s(Background::new().color(GRAY_0))
            .on_press(test_get_article())
        .item(Text::with_signal(user().signal_cloned()))
}

fn placeholder_text() -> impl Element {
    El::new()
        .s(Padding::top(Default::default(), 500))
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