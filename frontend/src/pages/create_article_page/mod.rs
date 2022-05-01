use crate::elements::button;
use crate::elements::dialogs;
use crate::elements::panel;
use crate::elements::tags;
use crate::router::{router, Route};
use crate::{app, connection};
use shared::UpMsg;
use zoon::named_color::GRAY_0;
use zoon::*;

mod view;

// ------ ------
//    States
// ------ ------

#[static_ref]
fn error_message() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}

#[static_ref]
fn title_text() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}

#[static_ref]
fn content_text() -> &'static Mutable<String> {
    Mutable::new("".to_string())
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
    if dialogs::confirm_dialog(
        "Your article will not be saved. Are you sure you want to leave the page?",
    ) {
        router().go(Route::Home);
    } else {
        return;
    }
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

// ------ ------
//     View
// ------ ------

// TODO: Use when moving view functions into view module
// pub fn view() -> RawElement {
//     view::page().into_raw_element()
// }

pub fn page() -> impl Element {
    title_text().set("".to_string());
    content_text().set("".to_string());
    tags::tags().lock_mut().clear();
    Column::new()
        .s(Align::center())
        .s(Width::new(800))
        .s(Background::new().color(GRAY_0))
        .item(
            Column::new()
                .s(Align::left(Default::default()))
                .s(Align::center())
                .s(Padding::new().x(100).y(20))
                .item(
                    Paragraph::new()
                        .content("Create new article")
                        .s(Font::new().size(20))
                        .s(Padding::bottom(Default::default(), 20)),
                )
                .item(title_panel())
                .item(content_text_panel())
                .item(tags::tag_panel("tag_input_create_article"))
                .item(tags::tags_view()),
        )
        .item(button_panel())
}

fn title_panel() -> impl Element {
    let id = "title_input";
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

fn content_text_panel() -> impl Element {
    panel::textarea_panel(
        "textarea_create_article",
        set_content_text,
        content_text().signal_cloned(),
    )
}

fn button_panel() -> impl Element {
    Row::new()
        .item(cancel_button())
        .item(publish_button())
        .s(Spacing::new(10))
        .s(Align::center())
}

fn publish_button() -> impl Element {
    button::button("publish", "Publish", add_article)
}

fn cancel_button() -> impl Element {
    button::button("cancel", "Cancel", cancel)
}


