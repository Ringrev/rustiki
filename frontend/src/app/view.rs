use std::sync::Arc;
use zoon::{format, *, Element, eprintln};
use zoon::*;
use zoon::named_color::{GRAY_0, GRAY_1, GRAY_2, GRAY_3, GRAY_4};
use shared::{DownMsg, UpMsg, User, Article};
use crate::{new_article_page, registration_page, log_in_page, router::{previous_route, router, Route}};
use crate::app::{articles_count, articles_exist};
use crate::footer::footer;
use crate::connection::connection;
use crate::header::{header};

fn articles() -> impl Element {
    Row::new()
        .items_signal_vec(super::filtered_articles().map(card))
        .s(Spacing::new(50))
        .multiline()
        .s(Width::new(1300))
}

fn panel() -> impl Element {
    Column::new()
        .item_signal(super::articles_exist().map_true(articles))
        .s(Align::new().center_x())
}

fn card(article: Article) -> impl Element {
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
        .on_click(move || super::view_article(article))
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
        .item(super::page())
        .item(footer()).s(Align::bottom(Default::default()))
}

// ------ front page content ------

pub(crate) fn front_page() -> impl Element {
    super::test_get_articles();
    Column::new()
        .s(Padding::new().top(50))
        .s(Width::fill())
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