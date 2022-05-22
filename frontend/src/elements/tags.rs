use crate::elements::dialogs::message_dialog;
use crate::elements::{button, panel};
use zoon::{named_color::*, *};

// ------ ------
//     States
// ------ ------

/// State of vector of tags. Initialized with empty vector.
#[static_ref]
pub fn tags() -> &'static MutableVec<String> {
    MutableVec::new()
}

/// State of new tag text. Initialized with empty String.
#[static_ref]
fn new_tag() -> &'static Mutable<String> {
    Mutable::new(String::new())
}

// ------ ------
//     Helpers
// ------ ------

/// Sets the text of new_tag.
///
/// # Arguments
/// * `tag` - A String that holds the text of tag.
fn set_tag_text(tag: String) {
    new_tag().set(tag);
}

// ------ ------
//     Commands
// ------ ------

/// Returns <code>true</code> if tag is unique.
/// Returns <code>false</code> if tag is not unique.
///
/// # Arguments
/// * `new_tag` - A String holding the text of new tag to add.
fn check_tag_unique(new_tag: String) -> bool {
    let mut unique = true;
    if tags().lock_mut().to_vec().len() > 0 {
        for existing_tag in tags().lock_mut().to_vec() {
            if existing_tag.eq(&new_tag) {
                unique = false;
                break;
            }
        }
    }
    unique
}

/// If tag input is empty, nothing happens.
/// If tag is unique, it is added to vector of tags.
/// If tag is not unique, a dialog is shown to user and input field cleared.
fn add_tag() {
    let mut new_tag = new_tag().lock_mut();
    let tag = new_tag.trim();
    if tag.is_empty() {
        return;
    }
    if check_tag_unique(tag.clone().to_string()) {
        tags().lock_mut().push_cloned(tag.clone().to_string());
        new_tag.clear();
    } else {
        message_dialog("Tag already exists");
        new_tag.clear();
    }
}

/// Removes a tag from tags() vector.
///
/// # Arguments
/// * `text` - A String holding the text of  tag to remove from tags() vector.
fn remove_tag(text: String) {
    tags().lock_mut().retain(|tag| !tag.eq(text.as_str()))
}

// ------ ------
//     View
// ------ ------

/// Returns a Column containing tag label, tag input field and button to add tag.
///
/// # Arguments
/// * `id` - A String slice that holds the element's unique HTML id.
pub fn tag_panel(id: &str) -> impl Element {
    Column::new()
        .s(Spacing::new(15))
        .item(panel::input_label(id.clone(), "Add a tag:"))
        .s(Spacing::new(0))
        .item(
            Row::new()
                .s(Spacing::new(10))
                .item(panel::text_input(
                    id,
                    set_tag_text,
                    "Write a tag...",
                    InputType::text(),
                    new_tag().signal_cloned(),
                    Some(add_tag),
                ))
                .item(add_tag_button()),
        )
}

/// Returns a Row displaying tags() vector.
pub fn tags_view() -> impl Element {
    Row::new()
        .items_signal_vec(tags().signal_vec_cloned().map(tag))
        .s(Spacing::new(10))
}

/// Returns a Row representing one single tag.
///
/// # Arguments
/// * `tag` - A String that holds the text the tag should display.
fn tag(tag: String) -> impl Element {
    Row::new()
        .item(
            Label::new()
                .label(tag.clone().to_string())
                .element_on_right(remove_tag_button(tag)),
        )
        .s(Padding::new().left(10).right(20))
        .s(Background::new().color(GRAY_2))
        .s(RoundedCorners::all(10))
}

/// Returns a Button which is displayed as the letter 'x', reacting to hover, click and focus.
///
/// # Arguments
/// * `tag` - A String holding the text of the tag that should be removed when button is clicked.
fn remove_tag_button(tag: String) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    let extra_tag = tag.clone();
    Button::new()
        .id("remove_tag_button")
        .s(Font::new()
            .size(20)
            .color_signal(hovered_signal.map_bool(|| GRAY_9, || RED_5)))
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .on_click(move || remove_tag(tag.clone().to_string()))
        .label(
            Paragraph::new()
                .content("x")
                .s(Font::new().size(15))
                .s(Align::new().center_y()),
        )
        .s(Height::new(20))
        .s(Padding::new().left(5))
        .focus(true)
        .on_key_down_event(move |event| event.if_key(Key::Enter, || remove_tag(extra_tag)))
}

/// Returns a Button element as defined in "button" module.
fn add_tag_button() -> impl Element {
    button::button("add_tag", "Add tag", add_tag)
}
