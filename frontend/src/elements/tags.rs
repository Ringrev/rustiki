use zoon::{*, named_color::{*}};
use crate::elements::dialogs::message_dialog;
use crate::elements::{panel, button};

fn add_tag_button() -> impl Element {
    button::button("Add", add_tag)
}

// ------ tag label and input combined

pub fn tag_panel() -> impl Element {
    let id = "tag_input";
    Column::new()
        .s(Spacing::new(15))
        .item(panel::input_label(id.clone(), "Add a tag:"))
        .s(Spacing::new(0))
        .item(Row::new()
            .s(Spacing::new(10))
            .item(panel::text_input(id, set_tag_text, "Write a tag...", InputType::text(), new_tag().signal_cloned(), Some(add_tag)))
            .item(add_tag_button()))
}

fn set_tag_text(tag: String) {
    new_tag().set(tag);
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