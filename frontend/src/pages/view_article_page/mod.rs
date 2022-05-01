use crate::elements::button;
use crate::pages::edit_article_page;
use crate::router::{router, Route};
use crate::app;
use shared::{LocalArticle};
use zoon::named_color::*;
use zoon::*;

mod view;

// ------ ------
//     States
// ------ ------

#[static_ref]
pub fn view_article() -> &'static Mutable<LocalArticle> {
    Mutable::new(LocalArticle::new_empty())
}

#[static_ref]
fn article_id() -> &'static Mutable<u32> {
    Mutable::new(0)
}

#[static_ref]
fn tags() -> &'static MutableVec<String> {
    MutableVec::new()
}

#[static_ref]
fn contributors() -> &'static MutableVec<String> {
    MutableVec::new()
}

// ------ ------
//     Commands
// ------ ------

pub fn edit_article() {
    edit_article_page::set_edit_article(view_article().get_cloned());
    router().go(Route::EditArticle);
}

pub fn set_view_article(art: LocalArticle) {
    tags().lock_mut().replace_cloned(art.tags.to_vec());
    view_article().set(art.clone().to_owned());
    article_id().set(art.id.to_owned());
    contributors()
        .lock_mut()
        .replace_cloned(art.contributors.clone())
}

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

pub fn page() -> impl Element {
    Column::new()
        .s(Align::center())
        .s(Width::new(800))
        .s(Background::new().color(GRAY_1))
        .item(article_view())
        .item(button_panel())
}

fn labels_view(text: &str, item: impl Element) -> impl Element {
    Row::new()
        .item(Paragraph::new().content(text).s(Font::new().size(12)))
        .item(item)
}

fn article_view() -> impl Element {
    Column::new()
        .s(Align::left(Default::default()))
        .s(Align::center())
        .s(Padding::new().x(100).y(20))
        .item(labels_view("Author:", author_view()))
        .item(labels_view("Contributors:", contributors_view()))
        .item(title_view())
        .item(content_view())
        .item(labels_view("Tags:", tags_view()))
        .item(time_view())
}

fn title_view() -> impl Element {
    Paragraph::new()
        .content(view_article().get_cloned().title)
        .s(Font::new().size(20))
        .s(Padding::new().top(20))
}

fn content_view() -> impl Element {
    Paragraph::new()
        .content(view_article().get_cloned().content)
        .s(Padding::new().bottom(20).top(10))
}

fn button_panel() -> impl Element {
    Row::new()
        .item_signal(app::is_user_logged_signal().map_true(edit_button))
        .item_signal(app::is_user_logged_signal().map_true(delete_button))
        .s(Spacing::new(10))
        .s(Align::center())
}

fn edit_button() -> impl Element {
    button::button("edit_article", "Edit", edit_article)
}

fn delete_button() -> impl Element {
    button::button("delete_article", "Delete", delete_article)
}

fn time_view() -> impl Element {
    Column::new()
        .item(
            Paragraph::new()
                .content(
                    "Last updated: ".to_string()
                        + view_article().get_cloned().updated_time.as_str(),
                )
                .s(Font::new().size(12)),
        )
        .s(Padding::new().bottom(5))
        .item(
            Paragraph::new()
                .content(
                    "Created: ".to_string() + view_article().get_cloned().created_time.as_str(),
                )
                .s(Font::new().size(12))
                .lang(Lang::English),
        )
        .s(Align::left(Default::default()))
}

fn contributors_view() -> impl Element {
    Row::new()
        .multiline()
        .items_signal_vec(contributors().signal_vec_cloned().map(contributor))
        .s(Spacing::new(10))
}

fn contributor(cont: String) -> impl Element {
    Row::new().item(
        Label::new()
            .label(cont.clone().to_string())
            .s(Font::new().size(12))
            .s(Padding::new().x(10))
            .s(Background::new().color(GRAY_2))
            .s(RoundedCorners::all(10)),
    )
}

fn author_view() -> impl Element {
    Row::new()
        .item(
            Label::new()
                .label(view_article().get_cloned().author)
                .s(Font::new().size(12))
                .s(Padding::new().x(10))
                .s(Background::new().color(GRAY_2))
                .s(RoundedCorners::all(10)),
        )
        .s(Padding::new().bottom(5))
}

fn tags_view() -> impl Element {
    Row::new()
        .items_signal_vec(tags().signal_vec_cloned().map(tag))
        .s(Spacing::new(10))
        .s(Padding::new().y(5))
}

fn tag(tag: String) -> impl Element {
    // let (hovered, hovered_signal) = Mutable::new_and_signal(false);

    Row::new().item(
        Label::new()
            .label(tag.clone().to_string())
            .s(Padding::new().x(10))
            .s(Font::new().size(12))
            .s(Background::new().color(GRAY_2))
            .s(RoundedCorners::all(10)),
    )
}