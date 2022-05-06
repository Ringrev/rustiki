//! Defines the content and operations for create edit article page.
use crate::app::logged_user_name;
use crate::connection;
use crate::elements::dialogs::*;
use crate::elements::{button, dialogs, panel, tags};
use crate::router::{router, Route};
use shared::{LocalArticle, UpMsg};
use zoon::named_color::GRAY_0;
use zoon::*;

mod view;

// ------ ------
//     States
// ------ ------

/// Title text of the article. Initialized with an empty String.
#[static_ref]
fn title_text() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}

/// The content of the article. Initialized with an empty String.
#[static_ref]
fn content_text() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}

/// The current article. Initialized with an empty LocalArticle.
#[static_ref]
fn edit_article() -> &'static Mutable<LocalArticle> {
    Mutable::new(LocalArticle::new_empty())
}

/// The current article's id. Initialized with 0.
#[static_ref]
fn article_id() -> &'static Mutable<u32> {
    Mutable::new(0)
}

/// Vector of current article's contributors. Initialized with empty vector.
#[static_ref]
fn contributors() -> &'static MutableVec<String> {
    MutableVec::new()
}

// ------ ------
//    Commands
// ------ ------

/// Starts an async Task that tells backend handler "edit_article" to update the current article in the database.
/// Dialog shown if there's an error in connection between frontend and backend.
pub fn update_article() {
    add_contributor();
    Task::start(async {
        let msg = UpMsg::EditArticle {
            id: article_id().get_cloned(),
            new_title: title_text().lock_mut().to_string(),
            new_content: content_text().lock_mut().to_string(),
            new_contributors: contributors().lock_mut().to_vec(),
            new_tags: tags::tags().lock_mut().to_vec(),
        };
        if let Err(error) = connection::connection().send_up_msg(msg).await {
            message_dialog(error.to_string().as_str());
        }
    });
}

/// Deletes an article from database as defined in pages::mod module.
pub fn delete_article() {
    super::delete_article(
        edit_article().get_cloned().author.as_str(),
        article_id().get_cloned(),
    );
}

/// Sets which LocalArticle should be edited.
///
/// # Arguments
/// * `art` - The LocalArticle to be edited.
pub fn set_edit_article(art: LocalArticle) {
    edit_article().set(art.clone().to_owned());
    title_text().set(art.title.clone());
    content_text().set(art.content.clone());
    article_id().set(art.id.to_owned());
    tags::tags().lock_mut().replace_cloned(art.tags.clone());
    contributors()
        .lock_mut()
        .replace_cloned(art.contributors.clone());
}

/// If logged_in_user is not the author of the article they are editing,
/// they are added as a contributor (unless they are already listed as one).
fn add_contributor() {
    let logged_in_user = logged_user_name().get_cloned().to_string();
    if !edit_article()
        .get_cloned()
        .author
        .to_string()
        .eq(logged_in_user.clone().as_str())
    {
        if !contributors().lock_mut().contains(&logged_in_user.clone()) {
            contributors()
                .lock_mut()
                .push_cloned(logged_in_user.clone());
        }
    }
}

// ------ ------
//     Helpers
// ------ ------

// Sets text of title_text()
fn set_title(title: String) {
    title_text().set(title);
}

/// Sets text of content_text()
fn set_content_text(content: String) {
    content_text().set(content);
}

// ------ ------
//     View
// ------ ------

// TODO: Use when moving view functions into view module
// pub fn view() -> RawElement {
//     view::page().into_raw_element()
// }

/// Returns a Column representing the whole edit article page.
pub fn page() -> impl Element {
    Column::new()
        .s(Align::center())
        .s(Width::new(800))
        .s(Background::new().color(GRAY_0))
        .item(
            Column::new()
                .s(Align::center())
                .s(Padding::new().x(100).y(20))
                .item(
                    Paragraph::new()
                        .content("Edit article")
                        .s(Font::new().size(20))
                        .s(Padding::bottom(Default::default(), 20)),
                )
                .item(title_panel())
                .item(content_text_panel())
                .item(tags::tag_panel("tag_input_edit_article"))
                .item(tags::tags_view()),
        )
        .item(button_panel())
}

/// Returns a Column containing a label and TextInput element as defined in "elements::panel" module.
fn title_panel() -> impl Element {
    let id = "title_input_edit_article";
    panel::input_panel(
        id,
        "Title:",
        set_title,
        "Title of your article",
        InputType::text(),
        title_text().signal_cloned(),
        None,
    )
}

/// Returns a Column containing a multiline TextArea element as defined in "elements::panel" module.
fn content_text_panel() -> impl Element {
    panel::textarea_panel(
        "textarea_edit_article",
        set_content_text,
        content_text().signal_cloned(),
    )
}

/// Returns a Row containing cancel_button and publish_button.
fn button_panel() -> impl Element {
    Row::new()
        .item(delete_button())
        .item(cancel_button())
        .item(publish_button())
        .s(Spacing::new(10))
        .s(Align::center())
}

/// Returns a Button element as defined in "elements::button" module.
fn delete_button() -> impl Element {
    button::button("delete_article", "Delete article", delete_article)
}

/// Returns a Button element as defined in "elements::button" module.
fn publish_button() -> impl Element {
    button::button("publish_changes", "Publish changes", update_article)
}

/// Returns a Button element as defined in "elements::button" module.
fn cancel_button() -> impl Element {
    button::button("cancel", "Cancel", dialogs::cancel)
}
