//! Defines the visual content for registration page.
use crate::elements::button;
use crate::elements::layouts::common_layout;
use crate::elements::{layouts, panel};
use crate::pages::registration_page;
use zoon::named_color::GRAY_0;
use zoon::*;

/// Returns a Column representing the whole registration page.
pub fn page() -> impl Element {
    super::clear_inputs();
    common_layout(
        Column::new()
            .s(Align::center())
            .s(Padding::new().x(100).y(20))
            .item(layouts::page_title_view("Create a user account"))
            .item(email_panel())
            .item(user_name_panel())
            .item(password_panel())
            .item(retyped_password_panel())
            .item(Text::with_signal(
                registration_page::error_message().signal_cloned(),
            )),
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
        "Your email address ",
        InputType::text(),
        super::email_text().signal_cloned(),
        None,
    )
}

/// Returns a Column containing a label and TextInput element as defined in "elements::panel" module.
fn user_name_panel() -> impl Element {
    let id = "user_name_input";
    panel::input_panel(
        id,
        "Username:",
        super::set_user_name,
        "Choose a username",
        InputType::text(),
        super::user_name_text().signal_cloned(),
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
        None,
    )
}

/// Returns a Column containing a label and TextInput element as defined in "elements::panel" module.
fn retyped_password_panel() -> impl Element {
    let id = "retyped_password_input";
    panel::input_panel(
        id,
        "Confirm password:",
        super::set_retyped_password,
        "Type password again",
        InputType::password(),
        super::retyped_password_text().signal_cloned(),
        Some(super::register_user),
    )
}

/// Returns a Row containing register_button().
fn button_panel() -> impl Element {
    Row::new().item(register_button()).s(Align::center())
}

/// Returns a Button element as defined in "elements::button" module.
fn register_button() -> impl Element {
    button::button("register_user", "Register", super::register_user)
}
