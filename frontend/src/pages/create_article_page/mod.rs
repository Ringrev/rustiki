use zoon::*;
use shared::UpMsg;
use crate::{app, connection};
use crate::elements::dialogs;
use crate::router::{Route, router};
use crate::elements::tags;
use crate::elements::panel;
use crate::elements::button;
use crate::rich_text::TextEditor;

mod view;

fn rich_text_editor() -> impl Element {
    Column::new()
        .s(Background::new().color(hsluv!(0,0,0,0)))
        .s(Width::new(600))
        .s(Height::new(600))
        .item(TextEditor::new()
            .on_change(|json| {
                contents().set(format!("{json:#}"));
            }))

}

fn contents_display() -> impl Element {
    El::new()
        .s(Padding::all(10))
        .s(Font::new().family([FontFamily::Monospace]))
        .child_signal(contents().signal_cloned())
}

#[static_ref]
fn contents() -> &'static Mutable<String> {
    Mutable::new(String::new())
}

// ------ ------
//    States
// ------ ------

#[static_ref]
fn error_message() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}

// ------ state of title

#[static_ref]
fn title_text() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}

// ------ state: content text
#[static_ref]
fn content_text() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}



pub fn page() -> impl Element {
    title_text().set("".to_string());
    content_text().set("".to_string());
    tags::tags().lock_mut().clear();
    Column::new()
        .s(Align::center())
        .s(Width::new(800))
        .s(Background::new().color(hsluv!(0,0,0,5)))
        .item(Column::new()
            .s(Align::left(Default::default()))
            .s(Align::center())
            .s(Padding::new().x(100).y(20))
            .item(Paragraph::new().content("Create new article").s(Font::new().size(20)).s(Padding::bottom(Default::default(), 20)))
            .item(title_panel())
            .item(rich_text_editor())
            .item(contents_display())
            // .item(content_text_panel())
            .item(tags::tag_panel())
            .item(tags::tags_view())
        )
        .item(button_panel())
}

// ------ ------
//    Commands
// ------ ------

pub fn add_article() {
    Task::start(async {
        let msg = UpMsg::AddArticle {
            title: title_text().get_cloned(),
            // TODO: change content when implemented in frontend with js quill.
            content: content_text().get_cloned(),
            author: app::logged_in_user().get_cloned().unwrap().username,
            tags: tags::tags().lock_mut().to_vec(),
        };
        if let Err(error) = connection::connection().send_up_msg(msg).await {
            dialogs::message_dialog(error.to_string().as_str())
        }
    });
}

fn cancel() {
    if dialogs::confirm_dialog("Your article will not be saved. Are you sure you want to leave the page?") {
        router().go(Route::Home);
    } else {
        return;
    }
}

// ------ ------
//     View
// ------ ------

// ------ title label and input

fn title_panel() -> impl Element {
    let id = "title_input";
    panel::input_panel(id, "Title:", set_title, "Title of your article", InputType::text(), title_text().signal_cloned(), None)
}

// ------ content label and input

fn content_text_panel() -> impl Element {
    panel::textarea_panel(set_content_text, content_text().signal_cloned())
}

fn button_panel() -> impl Element {
    Row::new()
        .item(cancel_button())
        .item(publish_button())
        .s(Spacing::new(10))
        .s(Align::center())
}

fn publish_button() -> impl Element {
    button::button("publish","Publish", add_article)
}

fn cancel_button() -> impl Element {
    button::button("cancel","Cancel", cancel)
}

// ------ ------
//     Helpers
// ------ ------

// pub fn set_error_msg(msg: String) {
//     error_message().set(msg);
// }

fn set_content_text(content: String) {
    content_text().set(content);
}

fn set_title(title: String) {
    title_text().set(title);
}