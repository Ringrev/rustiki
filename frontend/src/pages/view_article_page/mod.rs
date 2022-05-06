//! Defines the content and operations for view edit article page.
use crate::app;
use crate::elements::button;
use crate::pages::edit_article_page;
use crate::router::{router, Route};
use shared::LocalArticle;
use zoon::named_color::*;
use zoon::*;

mod view;

// ------ ------
//     States
// ------ ------

/// Current article. Initialized with an empty LocalArticle.
#[static_ref]
pub fn view_article() -> &'static Mutable<LocalArticle> {
    Mutable::new(LocalArticle::new_empty())
}

/// Current article's id. Initialized with 0.
#[static_ref]
fn article_id() -> &'static Mutable<u32> {
    Mutable::new(0)
}

/// Current article's tags.
#[static_ref]
fn tags() -> &'static MutableVec<String> {
    MutableVec::new()
}

/// Current article's contributors.
#[static_ref]
fn contributors() -> &'static MutableVec<String> {
    MutableVec::new()
}

// ------ ------
//     Commands
// ------ ------

/// Sets the article to be edited to the one being viewed, and directs user to edit page.
pub fn edit_article() {
    edit_article_page::set_edit_article(view_article().get_cloned());
    router().go(Route::EditArticle);
}

/// Sets which LocalArticle should be viewed.
///
/// # Arguments
/// * `art` - The LocalArticle to be viewed.
pub fn set_view_article(art: LocalArticle) {
    tags().lock_mut().replace_cloned(art.tags.to_vec());
    view_article().set(art.clone().to_owned());
    article_id().set(art.id.to_owned());
    contributors()
        .lock_mut()
        .replace_cloned(art.contributors.clone())
}

/// Deletes an article from database as defined in pages::mod module.
pub fn delete_article() {
    super::delete_article(
        view_article().get_cloned().author.as_str(),
        article_id().get_cloned(),
    );
}

// ------ ------
//     View
// ------ ------

// TODO: Use when moving view functions into view module
// pub fn view() -> RawElement {
//     view::page().into_raw_element()
// }

/// Returns a Column representing the whole view article page.
pub fn page() -> impl Element {
    Column::new()
        .s(Align::center())
        .s(Width::new(800))
        .s(Background::new().color(GRAY_1))
        .item(article_view())
        .item(button_panel())
}

/// Returns a Column containing the visual display of an article.
fn article_view() -> impl Element {
    Column::new()
        .s(Align::center())
        .s(Padding::new().x(100).y(20))
        .item(labels_view("Author:", author_view()))
        .item(labels_view("Contributors:", contributors_view()))
        .item(title_view())
        .item(content_view())
        .item(labels_view("Tags:", tags_view()))
        .item(time_view())
}

/// Returns a Paragraph element containing the visual display of the article's title.
fn title_view() -> impl Element {
    Paragraph::new()
        .content(view_article().get_cloned().title)
        .s(Font::new().size(20))
        .s(Padding::new().top(20))
}

/// Returns a Paragraph element containing the visual display of the article's content.
fn content_view() -> impl Element {
    Paragraph::new()
        .content(view_article().get_cloned().content)
        .s(Padding::new().bottom(20).top(10))
}

/// Returns a Row containing edit_button and delete_button.
fn button_panel() -> impl Element {
    Row::new()
        .item_signal(app::is_user_logged_signal().map_true(edit_button))
        .item_signal(app::is_user_logged_signal().map_true(delete_button))
        .s(Spacing::new(10))
        .s(Align::center())
}

/// Returns a Button element as defined in "elements::button" module.
fn edit_button() -> impl Element {
    button::button("edit_article", "Edit", edit_article)
}

/// Returns a Button element as defined in "elements::button" module.
fn delete_button() -> impl Element {
    button::button("delete_article", "Delete", delete_article)
}

/// Returns a Row representing the visual display of the author.
fn author_view() -> impl Element {
    label_template(view_article().get_cloned().author)
}

/// Returns a Row representing the visual display of one single tag.
fn tag(tag: String) -> impl Element {
    label_template(tag.clone().to_string())
}

/// Returns a Row representing the visual display of all of the article's tags.
fn tags_view() -> impl Element {
    Row::new()
        .items_signal_vec(tags().signal_vec_cloned().map(tag))
        .s(Spacing::new(10))
        .s(Padding::new().y(5))
}

/// Returns a Row representing the visual display of one single contributor.
fn contributor(cont: String) -> impl Element {
    label_template(cont.clone().to_string())
}

/// Returns a Row representing the visual display of all of the article's contributors.
fn contributors_view() -> impl Element {
    Row::new()
        .multiline()
        .items_signal_vec(contributors().signal_vec_cloned().map(contributor))
        .s(Spacing::new(10))
}

/// Returns a Row representing the visual design of a single author/tag/contributor.
fn label_template(text: String) -> impl Element {
    Row::new().item(
        Label::new()
            .label(text)
            .s(Padding::new().x(10))
            .s(Font::new().size(12))
            .s(Background::new().color(GRAY_2))
            .s(RoundedCorners::all(10)),
    )
}

/// Returns a Row containing a text label and an element displaying either
/// author, contributors or tags.
///
/// # Arguments
/// * `text` - The text to display.
/// * `item` - The element to display.
fn labels_view(text: &str, item: impl Element) -> impl Element {
    Row::new()
        .item(Paragraph::new().content(text).s(Font::new().size(12)))
        .item(item)
        .s(Padding::new().bottom(5))
}

/// Returns a Column representing visual display of time article was created and updated.
fn time_view() -> impl Element {
    Column::new()
        .item(time_text(
            "Last updated: ",
            view_article().get_cloned().updated_time.as_str(),
        ))
        .s(Padding::new().bottom(5))
        .item(time_text(
            "Created: ",
            view_article().get_cloned().created_time.as_str(),
        ))
        .s(Align::left(Default::default()))
}

/// Returns a Paragraph representing visual display of a date and time.
///
/// # Arguments
/// * `label` - The text to display.
/// * `time` - The time to display.
fn time_text(label: &str, time: &str) -> impl Element {
    Paragraph::new()
        .content(label.to_string() + time)
        .s(Font::new().size(12))
}
