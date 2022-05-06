//! Defines the content and operations for registration page.
use crate::connection;
use crate::elements::button;
use crate::elements::layouts;
use crate::elements::panel;
use shared::UpMsg;
use zoon::named_color::GRAY_0;
use zoon::*;

mod view;

// ------ ------
//     States
// ------ ------

/// Error message to display.
#[static_ref]
fn error_message() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}

/// User's email.
#[static_ref]
fn email_text() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}

/// User's username.
#[static_ref]
fn user_name_text() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}

/// User's password.
#[static_ref]
fn password_text() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}

/// User's retyped password.
#[static_ref]
fn retyped_password_text() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}

// ------ ------
//     Helpers
// ------ ------

/// Sets email_text().
fn set_email(email: String) {
    email_text().set(email);
}

/// Sets user_name_text().
fn set_user_name(user_name: String) {
    user_name_text().set(user_name);
}

/// Sets password_text().
fn set_password(password: String) {
    password_text().set(password);
}

/// Sets error_message().
pub fn set_error_msg(msg: String) {
    error_message().set(msg);
}

/// Sets retyped_password_text().
fn set_retyped_password(retyped_password: String) {
    retyped_password_text().set(retyped_password);
}

// ------ ------
//     Commands
// ------ ------

/// Returns <code>true</code> if passwords are the same.
/// Returns <code>false</code> if passwords are different.
fn passwords_match() -> bool {
    if !password_text()
        .get_cloned()
        .eq(&retyped_password_text().get_cloned())
    {
        set_error_msg(String::from("Passwords do not match. Please try again."));
        false
    } else {
        true
    }
}

/// Returns <code>true</code> if password is longer than 5 characters.
/// Returns <code>false</code> if password is 5 characters or shorter.
fn check_password() -> bool {
    if !(password_text().get_cloned().len() > 5) {
        set_error_msg(String::from("Password must be at least 6 characters long."));
        false
    } else {
        true
    }
}

/// If password and retyped password match, and password is long enough,
/// this function starts a new async Task telling handler "registration" to register new user.
fn register_user() {
    if passwords_match() && check_password() {
        Task::start(async {
            error_message().set("".to_string());
            let msg = UpMsg::Register {
                email: email_text().get_cloned(),
                username: user_name_text().get_cloned(),
                password: password_text().get_cloned(),
            };
            if let Err(error) = connection::connection().send_up_msg(msg).await {
                let error = error.to_string();
                set_error_msg(error.clone());
            }
        });
    }
}

// ------ ------
//     View
// ------ ------

// TODO: Use when moving view functions into view module
// pub fn view() -> RawElement {
//     view::page().into_raw_element()
// }

/// Makes sure input fields are cleared.
fn clear_inputs() {
    email_text().set("".to_string());
    user_name_text().set("".to_string());
    password_text().set("".to_string());
    retyped_password_text().set("".to_string());
}

/// Returns a Column representing the whole registration page.
pub fn page() -> impl Element {
    clear_inputs();
    Column::new()
        .s(Align::center())
        .s(Width::new(800))
        .s(Background::new().color(GRAY_0))
        .item(
            Column::new()
                .s(Align::center())
                .s(Padding::new().x(100).y(20))
                .item(layouts::title_view("Create a user account"))
                .item(email_panel())
                .item(user_name_panel())
                .item(password_panel())
                .item(retyped_password_panel())
                .item(Text::with_signal(error_message().signal_cloned())),
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
        "Your email address ",
        InputType::text(),
        email_text().signal_cloned(),
        None,
    )
}

/// Returns a Column containing a label and TextInput element as defined in "elements::panel" module.
fn user_name_panel() -> impl Element {
    let id = "user_name_input";
    panel::input_panel(
        id,
        "Username:",
        set_user_name,
        "Choose a username",
        InputType::text(),
        user_name_text().signal_cloned(),
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
        None,
    )
}

/// Returns a Column containing a label and TextInput element as defined in "elements::panel" module.
fn retyped_password_panel() -> impl Element {
    let id = "retyped_password_input";
    panel::input_panel(
        id,
        "Confirm password:",
        set_retyped_password,
        "Type password again",
        InputType::password(),
        retyped_password_text().signal_cloned(),
        Some(register_user),
    )
}

/// Returns a Row containing register_button().
fn button_panel() -> impl Element {
    Row::new().item(register_button()).s(Align::center())
}

/// Returns a Button element as defined in "elements::button" module.
fn register_button() -> impl Element {
    button::button("register_user", "Register", register_user)
}
