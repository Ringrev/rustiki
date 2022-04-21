use zoon::*;
use zoon::named_color::*;
use shared::UpMsg;
use crate::{app, connection};
use crate::elements::dialogs;
use crate::router::{Route, router};
use crate::elements::tags;

mod view;

pub fn page() -> impl Element {
    title_text().set("".to_string());
    content_text().set("".to_string());
    tags::tags().lock_mut().clear();
    Column::new()
        .s(Align::center())
        .s(Width::new(800))
        .s(Background::new().color(hsluv!(0,0,0,5)))
        .item(Column::new()
            .s(Align::left(Default::default()))
            .s(Align::center())
            .s(Padding::new().x(100).y(20))
            .item(Paragraph::new().content("Create new article").s(Font::new().size(20)).s(Padding::bottom(Default::default(), 20)))
            .item(title_panel())
            .item(content_text_panel())
            .item(tags::tag_panel())
            .item(tags::tags_view())
        )
        .item(button_panel())
}


//------ Add Article -------
#[static_ref]
fn error_message() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}

// pub fn set_error_msg(msg: String) {
//     error_message().set(msg);
// }

pub fn add_article() {
    Task::start(async {
        let msg = UpMsg::AddArticle {
            title: title_text().get_cloned(),
            // TODO: change content when implemented in frontend with js quill.
            content: content_text().get_cloned(),
            author: app::logged_in_user().get_cloned().unwrap().username,
            tags: tags::tags().lock_mut().to_vec(),
        };
        if let Err(error) = connection::connection().send_up_msg(msg).await {
            dialogs::message_dialog(error.to_string().as_str())
        }
    });
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


// ------ state: content text
#[static_ref]
fn content_text() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}

// ------ title label and input combined

fn content_text_panel() -> impl Element {
    let id = "content_input";
    Column::new()
        .s(Spacing::new(15))
        .item(content_text_label(id))
        .s(Spacing::new(0))
        .item(content_text_input(id))
    // .s(Padding::all(0))
}

// ------ title label

fn content_text_label(id: &str) -> impl Element {
    Label::new()
        .s(Font::new().color(hsluv!(0,0,0,100)))
        .s(Padding::all(0))
        .for_input(id)
        .label("Article text:")
}

fn set_content_text(content: String) {
    content_text().set(content);
}

// ------ title text input


fn content_text_input(id: &str) -> impl Element {
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
        .on_change(set_content_text)
        .placeholder(Placeholder::new("content text of your article"))
        .text_signal(content_text().signal_cloned())
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
        .on_press(add_article)
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
        .on_press(cancel)
}

fn cancel() {
    if dialogs::confirm_dialog("Your article will not be saved. Are you sure you want to leave the page?") {
        router().go(Route::Home);
    } else {
        return;
    }
}