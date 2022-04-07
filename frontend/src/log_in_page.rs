use zoon::{*, eprintln};
use zoon::events::Input;
use zoon::named_color::*;
use zoon::Tag::Header;
use zoon::text_input::{InputTypePassword, InputTypeText};
use zoon::web_sys::HtmlTextAreaElement;
use shared::UpMsg;
use crate::{app, connection};


pub fn page() -> impl Element {
    Column::new()
        .s(Align::center())
        .s(Width::new(800))
        .s(Background::new().color(hsluv!(0,0,0,5)))
        .item(Column::new()
            .s(Align::left(Default::default()))
            .s(Align::center())
            .s(Padding::new().x(100).y(20))
            .item(Paragraph::new().content("Log in"))
            .item(user_name_panel())
            .item(password_panel())
            .item(Text::with_signal(login_error().signal_cloned()))
        )

        .item(button_panel())
}

#[static_ref]
fn login_error() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}

pub fn set_login_error(err: String) {
    login_error().set(err);
}

// ----- login ------
pub fn login() {
    Task::start(async {
        set_login_error("".to_string());
        let msg = UpMsg::Login {
            email: user_name_text().get_cloned(),
            password: password_text().get_cloned(),
        };
        if let Err(error) = connection::connection().send_up_msg(msg).await {
            let error = error.to_string();
            set_login_error(error.clone());
            eprintln!("login request failed: {}", error);
            user_name_text().set("".to_string());
            password_text().set("".to_string());
        }
    });
}

// ------ state of user_name

#[static_ref]
fn user_name_text() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}

// ------ user_name label and input combined

fn user_name_panel() -> impl Element {
    let id = "user_name_input";
    Column::new()
        .s(Spacing::new(15))
        .item(user_name_text_label(id))
        .s(Spacing::new(0))
        .item(user_name_text_input(id))
    // .s(Padding::all(0))
}

// ------ user_name label

fn user_name_text_label(id: &str) -> impl Element {
    Label::new()
        .s(Font::new().color(hsluv!(0,0,0,100)))
        .s(Padding::all(0))
        .for_input(id)
        .label("User:")
}

fn set_user_name(user_name: String) {
    user_name_text().set(user_name);
}

// ------ user_name text input

fn user_name_text_input(id: &str) -> impl Element {
    TextInput::new()
        .s(Width::new(300))
        .s(Padding::new().x(10).y(6))
        .s(Shadows::new(vec![Shadow::new()
            .inner()
            .y(1)
            .blur(2)
            .color(hsluv!(0,0,0,20))]))
        .id(id)
        .on_change(set_user_name)
        .placeholder(Placeholder::new("Your user id "))
        .text_signal(user_name_text().signal_cloned())
}
// Password start

// ------ state of password

#[static_ref]
fn password_text() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}

// ------ password label and input combined

fn password_panel() -> impl Element {
    let id = "password_input";
    Column::new()
        .s(Spacing::new(15))
        .item(password_text_label(id))
        .s(Spacing::new(0))
        .item(password_text_input(id))
    // .s(Padding::all(0))
}

// ------ password label

fn password_text_label(id: &str) -> impl Element {
    Label::new()
        .s(Font::new().color(hsluv!(0,0,0,100)))
        .s(Padding::all(0))
        .for_input(id)
        .label("Password:")
}

fn set_password(password: String) {
    password_text().set(password);
}

// ------ password text input

fn password_text_input(id: &str) -> impl Element {
    TextInput::new()
        .s(Width::new(300))
        .s(Padding::new().x(10).y(6))
        .s(Shadows::new(vec![Shadow::new()
            .inner()
            .y(1)
            .blur(2)
            .color(hsluv!(0,0,0,20))]))
        .id(id)
        .on_change(set_password)
        .placeholder(Placeholder::new("Your password"))
        .text_signal(password_text().signal_cloned())
        .input_type(InputTypePassword::default())
        .on_key_down_event(|event| event.if_key(Key::Enter, login))
}

// Password end

// ------

fn button_panel() -> impl Element {
    Row::new()
        .item(cancel_button())
        .item(log_in_button())
        //.item(registration_button())
        .s(Spacing::new(10))
        .s(Align::center())
}

fn log_in_button() -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Font::new().size(16).color(GRAY_0))
        .s(Background::new()
            .color_signal(hovered_signal.map_bool(|| GRAY_5, || GRAY_9)))
        .s(Padding::new().y(10).x(15))
        .on_hovered_change(move |is_hovered| hovered.set(is_hovered))
        .label("Log in")
        .on_press(login)
}

fn cancel_button() -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Font::new().size(16).color(GRAY_0))
        .s(Background::new()
            .color_signal(hovered_signal.map_bool(|| GRAY_5, || GRAY_9)))
        .s(Padding::new().y(10).x(15))
        .on_hovered_change(move |is_hovered| hovered.set(is_hovered))
        .label("Cancel")
    // .on_press()
}

/*fn registration_button() -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Font::new().size(16).color(GRAY_0))
        .s(Background::new()
            .color_signal(hovered_signal.map_bool(|| GRAY_5, || GRAY_9)))
        .s(Padding::new().y(10).x(15))
        .on_hovered_change(move |is_hovered| hovered.set(is_hovered))
        .label("Registration")
    // .on_press()
}*/