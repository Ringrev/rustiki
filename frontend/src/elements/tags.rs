use zoon::{*, named_color::{*}};
use crate::elements::dialogs::message_dialog;

fn add_tag_button() -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Font::new().size(16).color(GRAY_0))
        .s(Background::new()
            .color_signal(hovered_signal.map_bool(|| GRAY_5, || GRAY_9)))
        .s(Padding::new().y(6).x(15))
        .on_hovered_change(move |is_hovered| hovered.set(is_hovered))
        .label("Add")
        .on_press(add_tag)
}

// ------ tag label and input combined

pub fn tag_panel() -> impl Element {
    let id = "tag_input";
    Column::new()
        .s(Spacing::new(15))
        .item(tag_label(id))
        .s(Spacing::new(0))
        .item(Row::new()
            .s(Spacing::new(5))
            .item(tag_input(id))
            .item(add_tag_button()))
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
pub fn tags() -> &'static MutableVec<String> {
    MutableVec::new()
}

#[static_ref]
fn new_tag() -> &'static Mutable<String> {
    Mutable::new(String::new())
}

#[static_ref]
fn tag_id() -> &'static Mutable<u32> {
    Mutable::new(0)
}

fn check_tag_unique(new_tag: String) -> bool {
    let mut unique = true;
    if tags().lock_mut().to_vec().len()>0 {
        for existing_tag in tags().lock_mut().to_vec() {
            if existing_tag.eq(&new_tag) {
                unique = false;
                break;
            }
        }
    }
    unique
}

fn add_tag() {
    let mut new_tag = new_tag().lock_mut();
    let tag = new_tag.trim();
    if tag.is_empty() {
        return;
    }
    if check_tag_unique(tag.clone().to_string()) {
        tag_id().update(|id|id+1);
        tags().lock_mut().push_cloned(tag.clone().to_string());
        new_tag.clear();
    } else {
        message_dialog("Tag already exists");
        new_tag.clear();
    }
}

pub fn tags_view() -> impl Element {
    Row::new()
        .items_signal_vec(tags().signal_vec_cloned().map(tag))
        .s(Spacing::new(10))
}

fn tag(tag: String) -> impl Element {
    // let (hovered, hovered_signal) = Mutable::new_and_signal(false);

    Row::new()
        .item(Label::new()
            // .for_input(tag.clone())
            .label(tag.clone().to_string())
            .element_on_right( remove_tag_button(tag.clone())))
        .s(Padding::new().left(10).right(20))
        .s(Background::new().color(GRAY_2))
        .s(RoundedCorners::all(10))
}

fn remove_tag(text: String) {
    tags().lock_mut().retain(|tag| !tag.eq(text.as_str()))
}

fn remove_tag_button(tag: String) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Font::new().size(20).color_signal(
            hovered_signal.map_bool(|| RED_5, || GRAY_4),
        ))
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .on_press(move || remove_tag(tag.clone().to_string()))
        .label(Paragraph::new().content("x").s(Font::new().size(15)).s(Align::new().center_y()))
        .s(Height::new(20))
        .s(Padding::new().left(5))
}