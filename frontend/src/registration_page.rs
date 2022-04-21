use zoon::*;
use zoon::named_color::*;
use zoon::text_input::{InputTypePassword};
use shared::UpMsg;
use crate::connection;

pub fn page() -> impl Element {
    email_text().set("".to_string());
    user_name_text().set("".to_string());
    password_text().set("".to_string());
    retyped_password_text().set("".to_string());
        Column::new()
            .s(Align::center())
            .s(Width::new(800))
            .s(Background::new().color(hsluv!(0,0,0,5)))
        .item(Column::new()
            .s(Align::center())
            .s(Padding::new().x(100).y(20))
            .item(Paragraph::new().content("Create a user account").s(Font::new().size(20)).s(Padding::bottom(Default::default(), 20)))
            .item(email_panel())
            .item(user_name_panel())
            .item(password_panel())
            .item(retyped_password_panel())
            .item(Text::with_signal(error_message().signal_cloned()))
        )
        .item(button_panel())
}



#[static_ref]
fn error_message() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}

pub fn set_error_msg(msg: String) {
    error_message().set(msg);
}

fn passwords_match() -> bool {
    if !password_text().get_cloned().eq(&retyped_password_text().get_cloned()) {
        set_error_msg(String::from("Passwords do not match. Please try again."));
        false
    } else {
        true
    }
}


fn check_password() -> bool {
    // Doesn't work:
    // if !password_text().get_cloned().len()>5 {
    // Works:
    if !(password_text().get_cloned().len()>5) {
        set_error_msg(String::from("Password must be at least 6 characters long."));
        false
    } else {
        true
    }
}

fn register_user() {
    if passwords_match()&&check_password() {
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

// ------ state of email

#[static_ref]
fn email_text() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}

// ------ email label and input combined

fn email_panel() -> impl Element {
    let id = "email_input";
    Column::new()
        .s(Spacing::new(15))
        .item(email_text_label(id))
        .s(Spacing::new(0))
        .item(email_text_input(id))
    // .s(Padding::all(0))
}

// ------ user_name label

fn email_text_label(id: &str) -> impl Element {
    Label::new()
        .s(Font::new().color(hsluv!(0,0,0,100)))
        .s(Padding::all(0))
        .for_input(id)
        .label("Email:")
}

fn set_email(email: String) {
    email_text().set(email);
}

// ------ user_name text input

fn email_text_input(id: &str) -> impl Element {
    TextInput::new()
        .s(Width::new(300))
        .s(Padding::new().x(10).y(6))
        .s(Shadows::new(vec![Shadow::new()
            .inner()
            .y(1)
            .blur(2)
            .color(hsluv!(0,0,0,20))]))
        .id(id)
        .on_change(set_email)
        .placeholder(Placeholder::new("Your email "))
        .text_signal(email_text().signal_cloned())
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
        .label("Username:")
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
}

// Password end

// retyped_password start

// ------ state of retyped_password

#[static_ref]
fn retyped_password_text() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}

// ------ retyped_password label and input combined

fn retyped_password_panel() -> impl Element {
    let id = "retyped_password_input";
    Column::new()
        .s(Spacing::new(15))
        .item(retyped_password_text_label(id))
        .s(Spacing::new(0))
        .item(retyped_password_text_input(id))
    // .s(Padding::all(0))
}

// ------ retyped_password label

fn retyped_password_text_label(id: &str) -> impl Element {
    Label::new()
        .s(Font::new().color(hsluv!(0,0,0,100)))
        .s(Padding::all(0))
        .for_input(id)
        .label("Please retype your password:")
}

fn set_retyped_password(retyped_password: String) {
    retyped_password_text().set(retyped_password);
}

// ------ retyped_password text input

fn retyped_password_text_input(id: &str) -> impl Element {
    TextInput::new()
        .s(Width::new(300))
        .s(Padding::new().x(10).y(6))
        .s(Shadows::new(vec![Shadow::new()
            .inner()
            .y(1)
            .blur(2)
            .color(hsluv!(0,0,0,20))]))
        .id(id)
        .on_change(set_retyped_password)
        .placeholder(Placeholder::new("Your password"))
        .text_signal(retyped_password_text().signal_cloned())
        .input_type(InputTypePassword::default())
        .on_key_down_event(|event| event.if_key(Key::Enter, register_user))
}

// Retyped retyped_password end

// ------


fn button_panel() -> impl Element {
    Row::new()
        .item(create_button())
        .s(Spacing::new(10))
        .s(Align::center())
}

fn create_button() -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Font::new().size(16).color(GRAY_0))
        .s(Background::new()
            .color_signal(hovered_signal.map_bool(|| GRAY_5, || GRAY_9)))
        .s(Padding::new().y(10).x(15))
        .on_hovered_change(move |is_hovered| hovered.set(is_hovered))
        .label("Create user")
        .on_press(register_user)
}