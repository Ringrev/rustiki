//! Defines the visual content for login page.
use crate::elements::button;
use crate::elements::layouts;
use crate::elements::layouts::common_layout;
use crate::elements::panel;
use zoon::*;

/// Returns a Column representing the whole login page.
pub fn page() -> impl Element {
    super::email_text().set("".to_string());
    super::password_text().set("".to_string());
    common_layout(
        Column::new()
            .s(Align::center())
            .s(Padding::new().x(100).y(20))
            .item(layouts::page_title_view("Log in"))
            .item(email_panel())
            .item(password_panel())
            .item(Text::with_signal(super::login_error().signal_cloned())),
        button_panel(),
    )
}

/// Returns a Column containing a label and TextInput element as defined in "elements::panel" module.
fn email_panel() -> impl Element {
    let id = "user_name_input";
    panel::input_panel(
        id,
        "Email address:",
        super::set_email,
        "Your email address",
        InputType::text(),
        super::email_text().signal_cloned(),
        None,
    )
}

/// Returns a Column containing a label and TextInput element as defined in "elements::panel" module.
fn password_panel() -> impl Element {
    let id = "password_input";
    panel::input_panel(
        id,
        "Password:",
        super::set_password,
        "Your password",
        InputType::password(),
        super::password_text().signal_cloned(),
        Some(super::login),
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
    button::button("log_in", "Log in", super::login)
}
