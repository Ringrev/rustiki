//! Defines the visual content for create edit article page.
use crate::elements::layouts::{common_layout, edit_layout};
use crate::elements::{button, dialogs, panel};
use zoon::*;

/// Returns a Column representing the whole edit article page.
pub fn page() -> impl Element {
    common_layout(
        edit_layout(
            "Edit article",
            title_panel(),
            content_text_panel(),
            "tag_input_edit_article",
        ),
        button_panel(),
    )
}

/// Returns a Column containing a label and TextInput element as defined in "elements::panel" module.
fn title_panel() -> impl Element {
    let id = "title_input_edit_article";
    panel::input_panel(
        id,
        "Title:",
        super::set_title,
        "Title of your article",
        InputType::text(),
        super::title_text().signal_cloned(),
        None,
    )
}

/// Returns a Column containing a multiline TextArea element as defined in "elements::panel" module.
fn content_text_panel() -> impl Element {
    panel::textarea_panel(
        "textarea_edit_article",
        super::set_content_text,
        super::content_text().signal_cloned(),
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
    button::button(
        "delete_article_edit_page",
        "Delete article",
        super::delete_article,
    )
}

/// Returns a Button element as defined in "elements::button" module.
fn publish_button() -> impl Element {
    button::button("publish_changes", "Publish changes", super::update_article)
}

/// Returns a Button element as defined in "elements::button" module.
fn cancel_button() -> impl Element {
    button::button("cancel_button_edit_page", "Cancel", dialogs::cancel)
}
