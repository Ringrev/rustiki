//! Defines the non-visual content and operations for login page.
use crate::connection;
use shared::UpMsg;
use zoon::*;

mod view;

// ------ ------
//     States
// ------ ------

/// Login error to display.
#[static_ref]
fn login_error() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}

/// User's password.
#[static_ref]
fn password_text() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}

/// User's email.
#[static_ref]
fn email_text() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}

// ------ ------
//     Commands
// ------ ------

/// Starts an async Task that tells backend handler "login" to log user into Firebase.
/// /// Dialog shown if there's an error in connection between frontend and backend.
pub fn login() {
    Task::start(async {
        set_login_error("".to_string());
        let msg = UpMsg::Login {
            email: email_text().get_cloned(),
            password: password_text().get_cloned(),
        };
        if let Err(error) = connection::connection().send_up_msg(msg).await {
            let error = error.to_string();
            set_login_error(error.clone());
        }
    });
}

// ------ ------
//     Helpers
// ------ ------

/// Sets password_text().
fn set_password(password: String) {
    password_text().set(password);
}

/// Sets email_text().
fn set_email(email: String) {
    email_text().set(email);
}

/// Sets login_error().
pub fn set_login_error(err: String) {
    login_error().set(err);
}

// ------ ------
//     View
// ------ ------

pub fn view() -> RawElement {
    view::page().into_raw_element()
}
