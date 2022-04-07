use zoon::*;
use zoon::events::Input;
use zoon::named_color::*;
use zoon::Tag::Header;
use zoon::text_input::InputTypeText;
use zoon::web_sys::HtmlTextAreaElement;


pub fn page() -> impl Element {
        Column::new()
            .s(Align::center())
            .s(Width::new(800))
            .s(Background::new().color(hsluv!(0,0,0,5)))
        .item(Column::new()
            .s(Align::left(Default::default()))
            .s(Align::center())
            .s(Padding::new().x(100).y(20))
            .item(Paragraph::new().content("Create user id: "))
            .item(user_name_panel())
            .item(password_panel())
            .item(retyped_password_panel())
        )
        .item(button_panel())
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
}

// Retyped retyped_password end

// ------

fn button_panel() -> impl Element {
    Row::new()
        .item(cancel_button())
        .item(create_button())
        .s(Spacing::new(10))
        .s(Align::right(Default::default()))
}

fn create_button() -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Font::new().size(16).color(GRAY_0))
        .s(Background::new()
            .color_signal(hovered_signal.map_bool(|| GRAY_5, || GRAY_9)))
        .s(Padding::new().y(10).x(15))
        .on_hovered_change(move |is_hovered| hovered.set(is_hovered))
        .label("Create")
    // .on_press()
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