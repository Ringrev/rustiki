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

#[static_ref]
fn login_error() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}

#[static_ref]
fn password_text() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}

#[static_ref]
fn email_text() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}

// ------ ------
//     Commands
// ------ ------

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

fn set_password(password: String) {
    password_text().set(password);
}

fn set_email(email: String) {
    email_text().set(email);
}

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

fn password_panel() -> impl Element {
    let id = "password_input";
    panel::input_panel(
        id,
        "Password:",
        set_password,
        "Your password...",
        InputType::password(),
        password_text().signal_cloned(),
        Some(login),
    )
}

fn button_panel() -> impl Element {
    Row::new()
        .item(log_in_button())
        .s(Spacing::new(10))
        .s(Align::center())
}

fn log_in_button() -> impl Element {
    button::button("log_in", "Log in", login)
}