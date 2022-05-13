//! Defines the visual content for front page.
use crate::router::{router, Route};
use shared::LocalArticle;
use zoon::named_color::{GRAY_2, GRAY_3};
use zoon::*;

/// Returns a Column representing the whole front page.
pub fn page() -> impl Element {
    Column::new()
        .s(Padding::new().top(50))
        .s(Width::fill())
        .item(panel())
}

/// Returns a multiline Row displaying all articles from articles() vector.
/// Button for creating new article is displayed using empty_card() element.
/// Each of the articles are displayed using a card element.
fn articles_view() -> impl Element {
    Row::new()
        .item_signal(super::is_user_logged_signal().map_true(empty_card))
        .items_signal_vec(super::filtered_articles().map(card))
        .s(Spacing::new(50))
        .multiline()
        .s(Width::fill())
}

/// Returns a Column containing articles_view() if there are existing articles in articles() vector.
fn panel() -> impl Element {
    Column::new()
        .item_signal(super::articles_exist().map_true(articles_view))
        .s(Padding::new().left(50))
}

/// Returns a button which visually represents an article.
///
/// # Arguments
/// * `article` - The LocalArticle to be displayed.
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
        .on_click(move || super::view_article(article))
        .focus(true)
        .on_key_down_event(|event| event.if_key(Key::Enter, || super::view_article(extra_article)))
        .s(Align::new().top())
}

/// Returns a button which is visually similar to an article,
/// but with the text "Create new article" instead of a title,
/// and a plus sign instead of a thumbnail image.
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

/// Returns a Column representing the attributes in common for card and empty_card.
///
/// # Arguments
/// * `element` - An element to show above the text.
/// * `text` - The text the card should show.
fn card_template(element: impl Element, text: String) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Column::new()
        .s(Padding::new().x(10).y(20))
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
