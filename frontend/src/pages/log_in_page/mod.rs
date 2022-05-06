//! Defines the content and operations for login page.
use crate::connection;
use crate::elements::button;
use crate::elements::panel;
use shared::UpMsg;
use zoon::named_color::GRAY_0;
use zoon::{eprintln, *};

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
            eprintln!("Login request failed: {}", error);
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

// TODO: Use when moving view functions into view module
// pub fn view() -> RawElement {
//     view::page().into_raw_element()
// }

/// Returns a Column representing the whole login page.
pub fn page() -> impl Element {
    email_text().set("".to_string());
    password_text().set("".to_string());
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
                        .content("Log in")
                        .s(Font::new().size(20))
                        .s(Padding::bottom(Default::default(), 20)),
                )
                .item(email_panel())
                .item(password_panel())
                .item(Text::with_signal(login_error().signal_cloned())),
        )
        .item(button_panel())
}

/// Returns a Column containing a label and TextInput element as defined in "elements::panel" module.
fn email_panel() -> impl Element {
    let id = "user_name_input";
    panel::input_panel(
        id,
        "Email address:",
        set_email,
        "Your email address",
        InputType::text(),
        email_text().signal_cloned(),
        None,
    )
}

/// Returns a Column containing a label and TextInput element as defined in "elements::panel" module.
fn password_panel() -> impl Element {
    let id = "password_input";
    panel::input_panel(
        id,
        "Password:",
        set_password,
        "Your password",
        InputType::password(),
        password_text().signal_cloned(),
        Some(login),
    )
}

/// Returns a Row containing log_in_button().
fn button_panel() -> impl Element {
    Row::new()
        .item(log_in_button())
        .s(Spacing::new(10))
        .s(Align::center())
}

/// Returns a Button element as defined in "elements::button" module.
fn log_in_button() -> impl Element {
    button::button("log_in", "Log in", login)
}
