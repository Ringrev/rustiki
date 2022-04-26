use zoon::*;
use shared::UpMsg;
use crate::connection;
use crate::elements::panel;
use crate::elements::button;

mod view;

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
    let id = "user_name_input";
    panel::input_panel(id,
                       "Email address:",
                       set_email,
                       "Your email address ",
                       InputType::text(),
                       email_text().signal_cloned(),
                       None)
}


fn set_email(email: String) {
    email_text().set(email);
}

// ------ state of user_name

#[static_ref]
fn user_name_text() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}

// ------ user_name label and input combined

fn user_name_panel() -> impl Element {
    let id = "user_name_input";
    panel::input_panel(id,
                       "Username:",
                       set_user_name,
                       "Choose a username",
                       InputType::text(),
                       user_name_text().signal_cloned(),
                       None)
}


fn set_user_name(user_name: String) {
    user_name_text().set(user_name);
}

// ------ state of password

#[static_ref]
fn password_text() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}

// ------ password label and input combined

fn password_panel() -> impl Element {
    let id = "password_input";
    panel::input_panel(id, "Password:", set_password, "Your password...",
                       InputType::password(),
                       password_text().signal_cloned(), None)
}

fn set_password(password: String) {
    password_text().set(password);
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
    panel::input_panel(id, "Confirm password:", set_retyped_password, "Type password again...",
                       InputType::password(),
                       retyped_password_text().signal_cloned(), Some(register_user))
}

fn set_retyped_password(retyped_password: String) {
    retyped_password_text().set(retyped_password);
}

fn button_panel() -> impl Element {
    Row::new()
        .item(register_button())
        .s(Align::center())
}

fn register_button() -> impl Element {
    button::button("register_user","Register", register_user)
}