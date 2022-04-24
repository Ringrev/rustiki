use zoon::*;
use zoon::signal::MutableSignalCloned;
use zoon::text_input::{InputTypeTrait};

pub fn input_panel(id: &str,
                   label_text: &str,
                   function: fn(String),
                   placeholder: &str,
                   input_type: impl InputTypeTrait,
                   text_signal: MutableSignalCloned<String>,
                   on_key: Option<fn()>) -> impl Element {
    Column::new()
        .s(Spacing::new(15))
        .item(input_label(id.clone(), label_text))
        .s(Spacing::new(0))
        .item(text_input(id, function, placeholder, input_type, text_signal, on_key))
}

// ------ label

pub fn input_label(id: &str, label_text: &str) -> impl Element {
    Label::new()
        .s(Font::new().color(hsluv!(0,0,0,100)))
        .s(Padding::all(0))
        .for_input(id)
        .label(label_text)
}

// ------  text input

pub fn text_input(id: &str, function: fn(String), placeholder: &str, input_type: impl InputTypeTrait, text_signal: MutableSignalCloned<String>, on_key: Option<fn()>) -> impl Element {
    TextInput::new()
        .s(Width::new(300))
        .s(Padding::new().x(10).y(6))
        .s(Shadows::new(vec![Shadow::new()
            .inner()
            .y(1)
            .blur(2)
            .color(hsluv!(0,0,0,20))]))
        .id(id.clone())
        .on_change(function)
        .placeholder(Placeholder::new(placeholder))
        .input_type(input_type)
        .text_signal(text_signal)
        .on_key_down_event(move |event| event.if_key(Key::Enter, on_key.unwrap()))
}

// ------ title label and input combined

pub fn textarea_panel(function: fn(String), text_signal: MutableSignalCloned<String>) -> impl Element {
    let id = "content_input";
    Column::new()
        .s(Spacing::new(15))
        .item(textarea_label(id.clone()))
        .s(Spacing::new(0))
        .item(textarea_input(id, function, text_signal))
}

// ------ title label

fn textarea_label(id: &str) -> impl Element {
    Label::new()
        .s(Font::new().color(hsluv!(0,0,0,100)))
        .s(Padding::all(0))
        .for_input(id)
        .label("Article content:")
}

fn textarea_input(id: &str, function: fn(String), text_signal: MutableSignalCloned<String>) -> impl Element {
    TextArea::new()
        .s(Width::new(600))
        .s(Height::new(400))
        .s(Padding::all(10))
        .s(Shadows::new(vec![Shadow::new()
            .inner()
            .y(1)
            .blur(2)
            .color(hsluv!(0,0,0,20))]))
        .id(id)
        .on_change(function)
        .placeholder(Placeholder::new("Content of your article..."))
        .text_signal(text_signal)
}