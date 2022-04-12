use std::sync::Arc;
use zoon::{format, *, Element, eprintln};
use zoon::*;
use zoon::named_color::GRAY_0;
use shared::{DownMsg, UpMsg, User, Article};
use crate::{new_article_page, registration_page, log_in_page, router::{previous_route, router, Route}};
use crate::app::{articles_count, articles_exist};
use crate::footer::footer;
use crate::connection::connection;
use crate::header::{header};

// fn articles() -> impl Element {
//     // for article in super::articles().to_owned() {
//     //
//     // }
//     Column::new()
//         .items(super::articles())
// }
//
// fn card(article: Article) -> impl Element {
//     Column::new()
//         .item(Image::new().url(article.name.clone()).description(article.name.clone()))
//         .item(Paragraph::new().s(Font::new().size(16).weight(FontWeight::Bold)).content(article.name))
//         .item(Paragraph::new().s(Font::new().size(12)).content(article.id))
// }



fn articles() -> impl Element {
    Column::new()
        .items_signal_vec(super::filtered_articles().map(card))
}

fn panel() -> impl Element {
    Column::new()
        .item_signal(super::articles_exist().map_true(articles))
}

fn content() -> impl Element {
    Column::new()
        .item(panel())
}

fn card(article: Arc<Article>) -> impl Element {
    Row::new()
        .s(Width::fill())
        .s(Background::new().color(hsluv!(0, 0, 100)))
        .s(Spacing::new(5))
        .s(Font::new().size(24))
        .item(Paragraph::new().content(article.name.clone()))
        .item(Paragraph::new().content(article.id.clone()))
}


// ------ content visible on all pages ------

pub fn root() -> impl Element {
    Column::new()
        .s(Height::screen())
        .item(header())//navbar placeholder
        .item(super::page())
        .item(footer()).s(Align::bottom(Default::default()))
}

// ------ front page content ------

pub(crate) fn front_page() -> impl Element {
    Column::new()
        .s(Padding::new().top(50))
        .item(placeholder_text())
        // .item(articles())
        .item(panel())
}

fn placeholder_text() -> impl Element {
    El::new()
        // .s(Padding::top(Default::default(), 500))
        // .child("Rustiki!").s(Font::new().size(40).color(hsluv!(18,100,48,100)))
        .s(Align::new().center_x())
        .s(Align::new().center_y())
}