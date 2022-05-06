//! Defines reusable inputs and panels.
use zoon::signal::MutableSignalCloned;
use zoon::text_input::InputTypeTrait;
use zoon::*;

/// Returns a Column that represents a panel containing input label and input field.
///
/// # Arguments
/// * `id` - A String slice that holds the element's unique HTML id.
/// * `label_text` - A String slice that holds the label to be shown.
/// * `function` - A function that takes String as an argument.
/// * `placeholder` - A String slice that holds the placeholder text for input field.
/// * `input_type` - An InputType defining if input is plain text or a password.
/// * `text_signal` - The cloned signal of a function that holds the state of a String.
/// * `on_key` - An optional function taking no arguments that is called when a key is pressed.
pub fn input_panel(
    id: &str,
    label_text: &str,
    function: fn(String),
    placeholder: &str,
    input_type: impl InputTypeTrait,
    text_signal: MutableSignalCloned<String>,
    on_key: Option<fn()>,
) -> impl Element {
    Column::new()
        .s(Spacing::new(15))
        .item(input_label(id.clone(), label_text))
        .s(Spacing::new(0))
        .item(text_input(
            id,
            function,
            placeholder,
            input_type,
            text_signal,
            on_key,
        ))
}

/// Returns a Label
///
/// # Arguments
/// * `id` - A String slice that holds the element's unique HTML id.
/// * `label_text` - A String slice that holds the text the label should display.
pub fn input_label(id: &str, label_text: &str) -> impl Element {
    Label::new()
        .s(Font::new().color(hsluv!(0, 0, 0, 100)))
        .s(Padding::all(0))
        .for_input(id)
        .label(label_text)
}

/// Returns a single-line TextInput element.
///
/// # Arguments
/// * `id` - A String slice that holds the element's unique HTML id.
/// * `function` - A function that takes String as an argument.
/// * `placeholder` - A String slice that holds the placeholder text for input field.
/// * `input_type` - An InputType defining if input is plain text or a password.
/// * `text_signal` - The cloned signal of a function that holds the state of a String.
/// * `on_key` - An optional function taking no arguments that is called when a key is pressed.
pub fn text_input(
    id: &str,
    function: fn(String),
    placeholder: &str,
    input_type: impl InputTypeTrait,
    text_signal: MutableSignalCloned<String>,
    on_key: Option<fn()>,
) -> impl Element {
    TextInput::new()
        .s(Width::new(300))
        .s(Padding::new().x(10).y(6))
        .s(Shadows::new(vec![Shadow::new()
            .inner()
            .y(1)
            .blur(2)
            .color(hsluv!(0, 0, 0, 20))]))
        .id(id.clone())
        .on_change(function)
        .placeholder(Placeholder::new(placeholder))
        .input_type(input_type)
        .text_signal(text_signal)
        .on_key_down_event(move |event| event.if_key(Key::Enter, on_key.unwrap()))
}

/// Returns a column containing input_label and textarea_input.
///
/// # Arguments
/// * `id` - A String slice that holds the element's unique HTML id.
/// * `function` - A function that takes String as an argument.
/// * `text_signal` - The cloned signal of a function that holds the state of a String.
pub fn textarea_panel(
    id: &str,
    function: fn(String),
    text_signal: MutableSignalCloned<String>,
) -> impl Element {
    Column::new()
        .s(Spacing::new(15))
        .item(input_label(id.clone(), "Article content:"))
        .s(Spacing::new(0))
        .item(textarea_input(id, function, text_signal))
}

/// Returns a multiline TextArea element.
///
/// # Arguments
/// * `id` - A String slice that holds the element's unique HTML id.
/// * `function` - A function that takes String as an argument.
/// * `text_signal` - The cloned signal of a function that holds the state of a String.
fn textarea_input(
    id: &str,
    function: fn(String),
    text_signal: MutableSignalCloned<String>,
) -> impl Element {
    TextArea::new()
        .s(Width::new(600))
        .s(Height::new(400))
        .s(Padding::all(10))
        .s(Shadows::new(vec![Shadow::new()
            .inner()
            .y(1)
            .blur(2)
            .color(hsluv!(0, 0, 0, 20))]))
        .id(id)
        .on_change(function)
        .placeholder(Placeholder::new("Content of your article..."))
        .text_signal(text_signal)
}
