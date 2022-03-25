use zoon::*;
use zoon::events::Input;
use zoon::named_color::{GRAY_0, GRAY_2, GRAY_4};
use zoon::Tag::Header;
use zoon::text_input::InputTypeText;
use zoon::web_sys::HtmlTextAreaElement;


pub fn page() -> impl Element {
    Column::new()
        .s(Align::center())
        .item(Paragraph::new().content("Create new article"))
        .item(title_panel())
        .item(Text::with_signal(title_text().signal_cloned()))
        .item(main_text_panel())
}

// ------ state of title

#[static_ref]
fn title_text() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}

// ------ title label and input combined

fn title_panel() -> impl Element {
    let id = "title_input";
    Column::new()
        .s(Spacing::new(15))
        .item(title_text_label(id))
        .s(Spacing::new(0))
        .item(title_text_input(id))
        // .s(Padding::all(0))
}

// ------ title label

fn title_text_label(id: &str) -> impl Element {
    Label::new()
        .s(Font::new().color(hsluv!(0,0,0,100)))
        .s(Padding::all(0))
        .for_input(id)
        .label("Title:")
}

fn set_title(title: String) {
    title_text().set(title);
}

// ------ title text input

fn title_text_input(id: &str) -> impl Element {
    TextInput::new()
        .s(Width::new(300))
        .s(Padding::new().x(10).y(6))
        .s(Shadows::new(vec![Shadow::new()
            .inner()
            .y(1)
            .blur(2)
            .color(hsluv!(0,0,0,20))]))
        .id(id)
        .on_change(set_title)
        .placeholder(Placeholder::new("Title of your article"))
        .text_signal(title_text().signal_cloned())
}


////////////////////////////////////////////////////////////


// ------ state: main text
#[static_ref]
fn main_text() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}

// ------ title label and input combined

fn main_text_panel() -> impl Element {
    let id = "main_input";
    Column::new()
        .s(Spacing::new(15))
        .item(main_text_label(id))
        .s(Spacing::new(0))
        .item(main_text_input(id))
    // .s(Padding::all(0))
}

// ------ title label

fn main_text_label(id: &str) -> impl Element {
    Label::new()
        .s(Font::new().color(hsluv!(0,0,0,100)))
        .s(Padding::all(0))
        .for_input(id)
        .label("Article text:")
}

fn set_main_text(main: String) {
    main_text().set(main);
}

// ------ title text input


fn main_text_input(id: &str) -> impl Element {
    TextArea::new()
        .s(Width::new(300))
        .s(Padding::all(10))
        .s(Shadows::new(vec![Shadow::new()
            .inner()
            .y(1)
            .blur(2)
            .color(hsluv!(0,0,0,20))]))
        .id(id)
        .on_change(set_main_text)
        .placeholder(Placeholder::new("Main text of your article"))
        .text_signal(main_text().signal_cloned())
}

// ------