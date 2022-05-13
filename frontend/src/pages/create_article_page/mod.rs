//! Defines the non-visual content and operations for create create article page.
use crate::elements::dialogs;
use crate::elements::tags;
use crate::{app, connection};
use shared::UpMsg;
use zoon::*;

mod view;

// ------ ------
//    States
// ------ ------

/// Error message shown to user. Initialized with an empty String.
#[static_ref]
fn error_message() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}

/// Title text for new article. Initialized with an empty String.
#[static_ref]
fn title_text() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}

/// Content text for new article. Initialized with an empty String.
#[static_ref]
fn content_text() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}

// ------ ------
//    Commands
// ------ ------

/// Starts an async Task that tells backend handler "add_article" to add an article to DB.
/// Dialog shown if there's an error in connection between frontend and backend.
pub fn add_article() {
    Task::start(async {
        let msg = UpMsg::AddArticle {
            title: title_text().get_cloned(),
            content: content_text().get_cloned(),
            author: app::logged_in_user().get_cloned().unwrap().username,
            tags: tags::tags().lock_mut().to_vec(),
        };
        if let Err(error) = connection::connection().send_up_msg(msg).await {
            dialogs::message_dialog(error.to_string().as_str())
        }
    });
}

// ------ ------
//     Helpers
// ------ ------

/// Sets text of content_text()
fn set_content_text(content: String) {
    content_text().set(content);
}

// Sets text of title_text()
fn set_title(title: String) {
    title_text().set(title);
}

// ------ ------
//     View
// ------ ------

pub fn view() -> RawElement {
    view::page().into_raw_element()
}
