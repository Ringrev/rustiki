//! Defines different reusable dialogs.
use crate::router::{router, Route};
use zoon::*;

/// Makes a dialog pop up on the screen
///
/// # Arguments
/// * `text` - A string slice that decides the message to be displayed in the dialog
pub fn message_dialog(text: &str) {
    if let Err(_) = window().alert_with_message(text.clone()) {
        message_dialog(text.clone())
    };
}

/// Makes a dialog pop up on the screen. User must confirm or cancel.
///
/// # Arguments
/// * `text` - A string slice that decides the message to be displayed in the dialog
pub fn confirm_dialog(text: &str) -> bool {
    let res = window().confirm_with_message(text);
    res.unwrap()
}

/// Shows a dialog asking user to confirm they want to cancel.
/// If user confirms they want to cancel, they are sent to front page.
/// If user cancels the operation, they stay on current page.
pub fn cancel() {
    if confirm_dialog("Your changes will not be saved. Are you sure you want to leave the page?") {
        router().go(Route::Home);
    } else {
        return;
    }
}
