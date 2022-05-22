//! Defines the non-visual content and operations for front page.
use crate::app::is_user_logged_signal;
use crate::connection::connection;
use crate::elements::dialogs::message_dialog;
use crate::router;
use crate::router::{router, Route};
use shared::{LocalArticle, UpMsg};
use std::collections::VecDeque;
use std::ops::Deref;
use zoon::*;

mod view;

// ------ ------
//    States
// ------ ------

/// Currently displayed articles.
#[static_ref]
pub fn articles() -> &'static MutableVec<LocalArticle> {
    MutableVec::new()
}

/// Original vector of articles.
#[static_ref]
pub fn original_articles() -> &'static MutableVec<LocalArticle> {
    MutableVec::new()
}

// ------ ------
//     Commands
// ------ ------

/// Returns the article that should be displayed at current route (Ex. /article/9324347283423)
/// Gets the current route and collects the LocalArticle with corresponding id from articles() vector.
pub fn get_article_from_route() -> LocalArticle {
    let route = router::route_history()
        .deref()
        .get_cloned()
        .front()
        .cloned()
        .unwrap();
    let route_text = route.to_owned().into_cow_str();
    let mut text: String = "".to_string();
    text = route_text.to_string().replace("/article/", "");
    let mut articles_vec = articles().lock_mut().to_vec();
    articles_vec.retain(|art| art.id.to_string().eq(text.trim()));
    articles_vec.get(0).unwrap().clone()
}

/// Directs user to view article page and sets the article that will be viewed to the one that was clicked.
///
/// # Arguments
/// * `article` - The article that was clicked/pressed.
pub fn view_article(article: LocalArticle) {
    router().go(Route::ViewArticle {
        article_id: article.id.to_string(),
    });
}

/// Sets the content of articles() and original_articles() to vector received from backend.
///
/// # Arguments
/// * `vector` - The vector of LocalArticle received from backend.
pub fn set_articles(vector: Vec<LocalArticle>) {
    let mut vec = VecDeque::new();
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

/// Starts an async task that asks backend handler "article" to get all articles from db.
pub fn get_articles() {
    Task::start(async {
        let msg = UpMsg::GetArticles;
        if let Err(error) = connection().send_up_msg(msg).await {
            message_dialog(error.to_string().as_str())
        }
    })
}

/// Sets the articles() vector back to the original articles collected from db.
pub fn reset_articles() {
    articles().update_mut(|art| {
        art.clear();
        art.extend(original_articles().lock_mut().to_vec());
    });
}

// ------ ------
//     Signals
// ------ ------

/// Returns LocalArticle from articles() vector.
fn filtered_articles() -> impl SignalVec<Item = LocalArticle> {
    articles()
        .signal_vec_cloned()
        .map(|article| article.clone())
}

/// Returns a usize count of items in articles()
fn articles_count() -> impl Signal<Item = usize> {
    articles().signal_vec_cloned().len()
}

/// Returns a boolean.
/// Returns <code>true</code> if count is not 0.
/// Returns <code>false</code> if count is 0.
fn articles_exist() -> impl Signal<Item = bool> {
    articles_count().map(|count| count != 0).dedupe()
}

// ------ ------
//     View
// ------ ------

pub fn view() -> RawElement {
    view::page().into_raw_element()
}
