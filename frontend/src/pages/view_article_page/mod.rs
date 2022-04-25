use zoon::*;
use zoon::named_color::*;
use shared::{LocalArticle, UpMsg};
use crate::{app, connection};
use crate::pages::edit_article_page;
use crate::elements::dialogs::{confirm_dialog, message_dialog};
use crate::router::{Route, router};
use crate::elements::button;

mod view;

pub fn page() -> impl Element {
    Column::new()
        .s(Align::center())
        .s(Width::new(800))
        .s(Background::new().color(hsluv!(0,0,0,5)))
        .item(article_view())
        .item(button_panel())
}

fn article_view() -> impl Element {
    Column::new()
        .s(Align::left(Default::default()))
        .s(Align::center())
        .s(Padding::new().x(100).y(20))
        .item(Row::new()
            .item(Paragraph::new().content("Author: ").s(Font::new().size(12)))
            .item(author_view()))
        .item(Row::new()
            .item(Paragraph::new().content("Contributors: ").s(Font::new().size(12)))
            .item(contributors_view()))
        .item(Paragraph::new().content(view_article().get_cloned().title).s(Font::new().size(20))
            .s(Padding::new().top(20)))
        .item(Paragraph::new().content(view_article().get_cloned().content)
            .s(Padding::new().bottom(20).top(10)))
        .item(Row::new()
            .item(Paragraph::new().content("Tags: ").s(Font::new().size(12)))
            .item(tags_view()))
        .item(time_view())
}

fn edit_button() -> impl Element {
    button::button("Edit", edit_article)
}

fn delete_button() -> impl Element {
    button::button("Delete", delete_article)
}

pub fn edit_article() {
    edit_article_page::set_edit_article(view_article().get_cloned());
    router().go(Route::EditArticle);
}

//------ View Article -------
#[static_ref]
pub fn view_article() -> &'static Mutable<LocalArticle> {
    Mutable::new(
        LocalArticle::new_empty()
    )
}

#[static_ref]
fn article_id() -> &'static Mutable<u32> {
    Mutable::new(0)
}

pub fn set_view_article(art: LocalArticle) {
    tags().lock_mut().replace_cloned(art.tags.to_vec());
    view_article().set(art.clone().to_owned());
    article_id().set(art.id.to_owned());
    contributors().lock_mut().replace_cloned(art.contributors.clone())
}

pub fn delete_article() {
    super::delete_article(view_article().get_cloned().author.as_str(), article_id().get_cloned());
}

fn button_panel() -> impl Element {
    Row::new()
        .item_signal(app::is_user_logged_signal().map_true(edit_button))
        .item_signal(app::is_user_logged_signal().map_true(delete_button))
        .s(Spacing::new(10))
        .s(Align::center())
}

#[static_ref]
fn tags() -> &'static MutableVec<String> {
    MutableVec::new()
}

fn tags_view() -> impl Element {
    Row::new()
        .items_signal_vec(tags().signal_vec_cloned().map(tag))
        .s(Spacing::new(10))
        .s(Padding::new().y(5))
}

fn tag(tag: String) -> impl Element {
    // let (hovered, hovered_signal) = Mutable::new_and_signal(false);

    Row::new()
        .item(Label::new()
            .label(tag.clone().to_string())
            .s(Padding::new().x(10))
            .s(Font::new().size(12))
            .s(Background::new().color(GRAY_2))
            .s(RoundedCorners::all(10)))
}

#[static_ref]
fn contributors() -> &'static MutableVec<String> {
    MutableVec::new()
}

fn contributors_view() -> impl Element {
    Row::new()
        .multiline()
        // .s(Background::new().color(GRAY_0))
        .items_signal_vec(contributors().signal_vec_cloned().map(contributor))
        .s(Spacing::new(10))
}

fn contributor(cont: String) -> impl Element {
    Row::new()
        .item(Label::new()
            .label(cont.clone().to_string())
            .s(Font::new().size(12))
            .s(Padding::new().x(10))
            .s(Background::new().color(GRAY_2))
            .s(RoundedCorners::all(10)))
}

fn author_view() -> impl Element {
    Row::new()
        .item(Label::new()
            .label(view_article().get_cloned().author)
            .s(Font::new().size(12))
            .s(Padding::new().x(10))
            .s(Background::new().color(GRAY_2))
            .s(RoundedCorners::all(10))
        )
        .s(Padding::new().bottom(5))
}

fn time_view() -> impl Element {
    Column::new()
        .item(Paragraph::new()
            .content("Last updated: ".to_string() + view_article().get_cloned().updated_time.as_str())
            .s(Font::new().size(12)))
        .s(Padding::new().bottom(5))
        .item(Paragraph::new().content("Created: ".to_string() + view_article().get_cloned().created_time.as_str()).s(Font::new().size(12)))
        .s(Align::left(Default::default()))
}
