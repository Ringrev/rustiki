use std::collections::VecDeque;
use zoon::{*, Element};
use zoon::named_color::{GRAY_2, GRAY_3};
use shared::{UpMsg, LocalUser, LocalArticle};
use crate::{new_article_page, registration_page, log_in_page, router::{router, Route}, edit_article_page, view_article_page};

use crate::footer::footer;
use crate::connection::connection;
use crate::elements::dialogs::message_dialog;
use crate::header::{header};

////////////////////////////////////
// ------ article stuff ------
////////////////////////////////////

pub fn view_article(article: LocalArticle) {
    view_article_page::set_view_article(article);
    router().go(Route::ViewArticle);
}

fn filtered_articles() -> impl SignalVec<Item = LocalArticle> {
    articles()
        .signal_vec_cloned()
        .map(|article|  article.clone())
}

#[static_ref]
pub fn articles() -> &'static MutableVec<LocalArticle> {
    MutableVec::new()
}

#[static_ref]
pub fn original_articles() -> &'static MutableVec<LocalArticle> {
    MutableVec::new()
}

fn articles_count() -> impl Signal<Item = usize> {
    articles().signal_vec_cloned().len()
}

fn articles_exist() -> impl Signal<Item = bool> {
    articles_count().map(|count| count != 0).dedupe()
}

pub fn set_articles(vector: Vec<LocalArticle>) {
    let mut vec= VecDeque::new();
    for article in vector {
        vec.push_front(article);
    }
    articles().update_mut(|art| {
        art.clear();
        art.extend(vec.clone());
    });
    original_articles().update_mut(|art| {
        art.clear();
        art.extend(vec.clone());
    })
}

pub fn reset_articles() {
    articles().update_mut(|art| {
        art.clear();
        art.extend(original_articles().lock_mut().to_vec());
    });

}

pub fn test_get_articles() {
    Task::start(async {
        let msg = UpMsg::GetArticles;
        if let Err(error) = connection().send_up_msg(msg).await {
            message_dialog(error.to_string().as_str())
        }
    })
}

////////////////////////////////////
// ------ logged in/out state ------
////////////////////////////////////

/// Used to decide if user sees logged in or out view of site
pub fn is_user_logged_signal() -> impl Signal<Item = bool> {
    logged_in_user().signal_ref(Option::is_some)
}

// /// Boolean value which is set to true when logged_user
// pub fn is_user_logged() -> bool {
//     logged_in_user().map(Option::is_some)
// }

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

/// Removes token and user info so the user is logged out of the site
pub fn log_out() {
    auth_token().set(None);
    logged_in_user().set(None);
    logged_user_name().set("".to_string());
}

/// Holds the auth token when a user is logged in
#[static_ref]
pub fn auth_token() -> &'static Mutable<Option<String>> {
    Mutable::new(None)
}

/// Sets user info and token when user logs in. Called from connection.rs
pub fn set_logged_in_user_and_token(user: LocalUser) {
    logged_in_user().set(Some(user.clone()));
    logged_user_name().set(Some(user.clone()).unwrap().username);
    auth_token().set(Some(user.clone().auth_token));
    router().go(Route::Root);
}

/////////////////////////////
// ------ page names ------
/////////////////////////////

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

/////////////////////////////
// ------ page routing ------
/////////////////////////////

fn page() -> impl Element {
    El::new().child_signal(page_name().signal().map(|page_name| match page_name {
        PageName::Home => front_page().into_raw_element(),
        PageName::Unknown => El::new().child("404").into_raw_element(),
        PageName::EditArticle => edit_article_page::page().into_raw_element(),
        PageName::NewArticle => new_article_page::page().into_raw_element(),
        PageName::Registration => registration_page::page().into_raw_element(),
        PageName::LogIn => log_in_page::page().into_raw_element(),
        PageName::ViewArticle => view_article_page::page().into_raw_element(),
    }))
}

#[static_ref]
fn page_name() -> &'static Mutable<PageName> {
    Mutable::new(PageName::Unknown)
}

pub fn set_page_name(new_page_name: PageName) {
    page_name().set_neq(new_page_name);
}


fn articles_view() -> impl Element {
    Row::new()
        .items_signal_vec(filtered_articles().map(card))
        .s(Spacing::new(50))
        .multiline()
        .s(Width::new(1300))
}

fn panel() -> impl Element {
    Column::new()
        .item_signal(articles_exist().map_true(articles_view))
        .s(Align::new().center_x())
}

fn card(article: LocalArticle) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Column::new()
        .s(Padding::new().x(10).y(20))
        .s(Spacing::new(5))
        .s(Font::new().size(24))
        .s(Background::new()
            .color_signal(hovered_signal.map_bool(|| GRAY_2, || hsluv!(0, 0, 100))))
        .item(Image::new().url("https://rustacean.net/assets/rustacean-flat-happy.png")
            .description("Placeholder picture")
            .s(Width::max(Default::default(), 200))
            .s(Background::new().color(GRAY_3)))
        .item(Paragraph::new().content(article.title.clone()))
        .on_hovered_change(move |is_hovered| hovered.set(is_hovered))
        // .item(Paragraph::new().content(article.content.clone()))
        .on_click(move || view_article(article))
}

// ------ content visible on all pages ------

pub fn root() -> impl Element {
    // Stack::new()
    //     .s(Height::screen())
    //     .layer(Column::new()
    //         .item(page())
    //         .item(footer()).s(Align::bottom(Default::default()))
    //         .s(Padding::top(Default::default(), 100))
    //         .s(Height::fill()))
    //     .layer(header())

    Column::new()
        .s(Height::screen())
        .s(Width::fill())
        .item(header()).s(Align::top(Default::default())) //navbar placeholder
        .item(page())
        .item(footer()).s(Align::bottom(Default::default()))
}

// ------ front page content ------

fn front_page() -> impl Element {
    test_get_articles();
    Column::new()
        .s(Padding::new().top(50))
        .s(Width::fill())
        .item(panel())
}