use std::sync::Arc;
use zoon::*;
use zoon::events::Input;
use zoon::named_color::*;
use zoon::Tag::Header;
use zoon::text_input::InputTypeText;
use zoon::web_sys::HtmlTextAreaElement;
use uuid::Uuid;


pub fn page() -> impl Element {
    Column::new()
        .s(Align::center())
        .s(Width::new(800))
        .s(Background::new().color(hsluv!(0,0,0,5)))
        .item(Column::new()
            .s(Align::left(Default::default()))
            .s(Align::center())
            .s(Padding::new().x(100).y(20))
            .s(Width::max(Default::default(), 600))
            .item(Paragraph::new().content("Create new article"))
            .item(title_panel())
            .item(main_text_panel())
            .item(tag_panel())
            .item(tags_view())
        )
        .item(button_panel())
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
        .s(Width::new(600))
        .s(Height::new(400))
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

fn button_panel() -> impl Element {
    Row::new()
        .item(cancel_button())
        .item(publish_button())
        .s(Spacing::new(10))
        .s(Align::right(Default::default()))
}

fn publish_button() -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Font::new().size(16).color(GRAY_0))
        .s(Background::new()
            .color_signal(hovered_signal.map_bool(|| GRAY_5, || GRAY_9)))
        .s(Padding::new().y(10).x(15))
        .on_hovered_change(move |is_hovered| hovered.set(is_hovered))
        .label("Publish")
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




// ------ state: tags
// #[static_ref]
// fn tag_text() -> &'static Mutable<String> {
//     Mutable::new("".to_string())
// }

// ------ tag label and input combined

fn tag_panel() -> impl Element {
    let id = "tag_input";
    Column::new()
        .s(Spacing::new(15))
        .item(tag_label(id))
        .s(Spacing::new(0))
        .item(tag_input(id))
    // .s(Padding::all(0))
}

// ------ tag label

fn tag_label(id: &str) -> impl Element {
    Label::new()
        .s(Font::new().color(hsluv!(0,0,0,100)))
        .s(Padding::all(0))
        .for_input(id)
        .label("Add a tag:")
}

fn set_tag_text(tag: String) {
   new_tag().set(tag);
}

// ------ tag input


fn tag_input(id: &str) -> impl Element {
    TextInput::new()
        .s(Width::new(300))
        .s(Padding::new().x(10).y(6))
        .s(Shadows::new(vec![Shadow::new()
            .inner()
            .y(1)
            .blur(2)
            .color(hsluv!(0,0,0,20))]))
        .id(id)
        .on_change(set_tag_text)
        .placeholder(Placeholder::new("Tag..."))
        .text_signal(new_tag().signal_cloned())
        .on_key_down_event(|event| event.if_key(Key::Enter, add_tag))
}



#[static_ref]
fn tags() -> &'static MutableVec<Arc<Tag>> {
    MutableVec::new()
}

#[static_ref]
fn new_tag() -> &'static Mutable<Tag> {
    Mutable::new()
}

#[static_ref]
fn tag_id() -> &'static Mutable<i32> {
    Mutable::new(0)
}

fn add_tag() {
    let mut new_tag = new_tag().lock_mut();
    let tag = new_tag.trim();
    if tag.is_empty() {
        return;
    }
    let tag = Tag {
        id: tag_id()+1,
        text: tag,
    };
    tag_id() +=1;
    tags().lock_mut().push_cloned(Arc::new(tag));
    new_tag.clear();
}

fn tags_view() -> impl Element {
    Row::new()
        .items_signal_vec(tags().signal_vec_cloned().map(tag))
        .s(Spacing::new(10))
}

fn tag(tag: Arc<String>) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Column::new()
        .item(Paragraph::new()
            .content(tag.to_string())
            .s(Padding::new().x(10))
            .s(Background::new().color(GRAY_2))
            .s(RoundedCorners::all(10))

        )
        .element_on_right(remove_tag_button(&tag))
        // .element_on_right_signal(hovered_signal.map_true(move || remove_tag_button(&tag)))
}

fn remove_tag(tag: String) {
    tags().lock_mut().retain(|tag| tag.to_string().eq_ignore_ascii_case(tag))
}

fn remove_tag_button(tag: &Arc<String>) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    let id = tag.to_string();
    Button::new()
        .s(Font::new().size(20).center().color_signal(
            hovered_signal.map_bool(|| hsluv!(10.5, 37.7, 48.8), || hsluv!(12.2, 34.7, 68.2)),
        ))
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .on_press(move || remove_tag(id))
        .label("×")
}

struct Tag {
    id: i32,
    text: String,
}




