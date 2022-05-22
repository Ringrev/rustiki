//! Defines the visual content for create create article page.
use crate::elements::button;
use crate::elements::dialogs;
use crate::elements::layouts::{common_layout, edit_layout};
use crate::elements::panel;
use crate::elements::tags;
use crate::rich_text::TextEditor;
use zoon::*;


fn rich_text_editor() -> impl Element {
    Column::new()
        .s(Padding::new().top(15))
        .item(Label::new()
            .s(Font::new().color(hsluv!(0,0,0,100)))
            .s(Padding::all(0))
            .label("Article content:")
        )
        .s(Background::new().color(hsluv!(0,0,0,0)))
        .s(Width::new(600))
        .s(Height::new(600))
        .item(TextEditor::new()
            .on_change(|json| {
                super::contents().set(format!("{json:#}"));
            }))
        // This is only here for testing purposes. This is how
        .item(Paragraph::new()
            .content("The text that appears below here when you type in the text \
            editor is here only for testing purposes. \
            It's the string of text generated when you type in the text editor."))
        .item(contents_display())
}

fn contents_display() -> impl Element {
    El::new()
        .s(Padding::all(10))
        .s(Font::new().family([FontFamily::Monospace]))
        .child_signal(super::contents().signal_cloned())
}

/// Returns a Column representing the whole create article page.
pub fn page() -> impl Element {
    super::title_text().set("".to_string());
    super::content_text().set("".to_string());
    tags::tags().lock_mut().clear();
    common_layout(
        edit_layout(
            "Create new article",
            title_panel(),
            rich_text_editor(),
            "tag_input_create_article_page",
        ),
        button_panel(),
    )
}

/// Returns a single-line TextInput element as defined in "elements::panel" module.
fn title_panel() -> impl Element {
    let id = "title_input_create_article";
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

/// Returns a multiline TextArea element as defined in "elements::panel" module.
fn content_text_panel() -> impl Element {
    panel::textarea_panel(
        "textarea_create_article",
        super::set_content_text,
        super::content_text().signal_cloned(),
    )
}

/// Returns a Row containing cancel_button and publish_button.
fn button_panel() -> impl Element {
    Row::new()
        .item(cancel_button())
        .item(publish_button())
        .s(Spacing::new(10))
        .s(Align::center())
}

/// Returns a Button element as defined in "elements::button" module.
fn publish_button() -> impl Element {
    button::button("publish", "Publish", super::add_article)
}

/// Returns a Button element as defined in "elements::button" module.
fn cancel_button() -> impl Element {
    button::button(
        "cancel_button_create_article_page",
        "Cancel",
        dialogs::cancel,
    )
}
