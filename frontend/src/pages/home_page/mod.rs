use std::collections::VecDeque;
use zoon::*;
use zoon::named_color::{GRAY_2, GRAY_3};
use shared::{LocalArticle, UpMsg};
use crate::connection::connection;
use crate::elements::dialogs::message_dialog;
use crate::router::{Route, router};
use crate::pages::view_article_page;

mod view;


// ------ front page content ------

pub fn page() -> impl Element {
    Column::new()
        .s(Padding::new().top(50))
        .s(Width::fill())
        .item(panel())
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

////////////////////////////////////
// ------ article stuff ------
////////////////////////////////////

pub fn view_article(article: LocalArticle) {
    view_article_page::set_view_article(article.clone());
    router().go(Route::ViewArticle {
        article_id: article.id.to_string()
    });
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
