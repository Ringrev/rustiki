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

fn articles() -> impl Element {
    Column::new()
        .items_signal_vec(super::filtered_articles().map(card))
        .s(Spacing::new(50))
}

fn panel() -> impl Element {
    Column::new()
        .item_signal(super::articles_exist().map_true(articles))
}

fn card(article: Article) -> impl Element {
    Column::new()
        .s(Background::new().color(hsluv!(0, 0, 100)))
        .s(Spacing::new(5))
        .s(Font::new().size(24))
        .item(Image::new().url("https://i.guim.co.uk/img/media/3d7d923db999d53074642f9e8051812a186c765a/0_0_2048_1463/master/2048.jpg?width=700&quality=85&auto=format&fit=max&s=2227033b1ae471edf46f7559ab517d1f").description("Placeholder picture").s(Width::max(Default::default(), 200)))
        .item(Paragraph::new().content(article.title.clone()))
        .item(Paragraph::new().content(article.content.clone()))
        .on_click(move || super::edit_article(article))
}

// ------ content visible on all pages ------

pub fn root() -> impl Element {
    Column::new()
        .s(Height::screen())
        .item(header()).s(Align::top(Default::default())) //navbar placeholder
        .item(super::page())
        .item(footer()).s(Align::bottom(Default::default()))
}

// ------ front page content ------

pub(crate) fn front_page() -> impl Element {
    super::test_get_articles();
    Column::new()
        .s(Padding::new().top(50))
        .item(placeholder_text())
        .item(panel())
}

fn placeholder_text() -> impl Element {
    El::new()
        // .s(Padding::top(Default::default(), 500))
        // .child("Rustiki!").s(Font::new().size(40).color(hsluv!(18,100,48,100)))
        .s(Align::new().center_x())
        .s(Align::new().center_y())
}

// fn content() -> impl Element {
//     Column::new()
//         .s(Width::max_fill(Default::default()))
//         .item(panel())
// }