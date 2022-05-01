use crate::app::is_user_logged_signal;
use crate::connection::connection;
use crate::elements::dialogs::message_dialog;
use crate::router;
use crate::router::{router, Route};
use shared::{LocalArticle, UpMsg};
use std::collections::VecDeque;
use std::ops::Deref;
use zoon::named_color::{GRAY_2, GRAY_3};
use zoon::*;

mod view;

// ------ ------
//    States
// ------ ------

#[static_ref]
pub fn articles() -> &'static MutableVec<LocalArticle> {
    MutableVec::new()
}

#[static_ref]
pub fn original_articles() -> &'static MutableVec<LocalArticle> {
    MutableVec::new()
}

// ------ ------
//     Commands
// ------ ------

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

pub fn view_article(article: LocalArticle) {
    router().go(Route::ViewArticle {
        article_id: article.id.to_string(),
    });
}

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

pub fn get_articles() {
    Task::start(async {
        let msg = UpMsg::GetArticles;
        if let Err(error) = connection().send_up_msg(msg).await {
            message_dialog(error.to_string().as_str())
        }
    })
}

pub fn reset_articles() {
    articles().update_mut(|art| {
        art.clear();
        art.extend(original_articles().lock_mut().to_vec());
    });
}

// ------ ------
//     Signals
// ------ ------

fn filtered_articles() -> impl SignalVec<Item = LocalArticle> {
    articles()
        .signal_vec_cloned()
        .map(|article| article.clone())
}

fn articles_count() -> impl Signal<Item = usize> {
    articles().signal_vec_cloned().len()
}

fn articles_exist() -> impl Signal<Item = bool> {
    articles_count().map(|count| count != 0).dedupe()
}

// ------ ------
//     View
// ------ ------

// TODO: Use when moving view functions into view module
// pub fn view() -> RawElement {
//     view::page().into_raw_element()
// }

pub fn page() -> impl Element {
    get_articles();
    Column::new()
        .s(Padding::new().top(50))
        .s(Width::fill())
        .item(panel())
}

fn articles_view() -> impl Element {
    Row::new()
        .item_signal(is_user_logged_signal().map_true(empty_card))
        .items_signal_vec(filtered_articles().map(card))
        .s(Spacing::new(50))
        .multiline()
        .s(Width::fill())
}

fn panel() -> impl Element {
    Column::new()
        .item_signal(articles_exist().map_true(articles_view))
        .s(Padding::new().left(50))
}

fn card(article: LocalArticle) -> impl Element {
    let extra_article = article.clone();
    let mut id = article.clone().title + article.clone().created_time.as_str();
    id = id.replace(" ", "-");
    Button::new()
        .id(id)
        .label(card_template(
            Image::new()
                .url("https://rustacean.net/assets/rustacean-flat-happy.png")
                .description("Placeholder picture")
                .s(Width::new(200))
                .s(Height::new(130))
                .s(Background::new().color(GRAY_3)),
            article.clone().title,
        ))
        .on_click(move || view_article(article))
        .focus(true)
        .on_key_down_event(|event| event.if_key(Key::Enter, || view_article(extra_article)))
        .s(Align::new().top())
}

fn empty_card() -> impl Element {
    let id = "create_new_article";
    Button::new()
        .id(id)
        .label(card_template(
            Row::new()
                .s(Width::new(200))
                .s(Height::new(130))
                .s(Background::new().color(GRAY_3))
                .item(
                    Paragraph::new()
                        .content("+")
                        .s(Align::new().center_x().center_y())
                        .s(Font::new().size(100)),
                ),
            "Create new article".to_string(),
        ))
        .on_click(move || router().go(Route::NewArticle))
        .focus(true)
        .on_key_down_event(|event| event.if_key(Key::Enter, || router().go(Route::NewArticle)))
        .s(Align::new().top())
}

fn card_template(element: impl Element, text: String) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Column::new()
        .s(Padding::new().x(10).y(20))
        // .s(Spacing::new(5))
        .s(Font::new().size(24))
        .s(
            Background::new()
                .color_signal(hovered_signal.map_bool(|| GRAY_2, || hsluv!(0, 0, 100))),
        )
        .item(element)
        .item(
            Row::new()
                .multiline()
                .s(Width::new(200))
                .item(Paragraph::new().content(text)),
        )
        .on_hovered_change(move |is_hovered| hovered.set(is_hovered))
}