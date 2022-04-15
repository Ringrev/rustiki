use zoon::{*, println};
use zoon::events::Input;
use zoon::named_color::*;
use zoon::text_input::InputTypeText;
use zoon::web_sys::HtmlTextAreaElement;
use shared::{Article, UpMsg, User, Tag};
use shared::UpMsg::AddArticle;
use crate::{app, connection};
use crate::router::{Route, router};

pub fn page() -> impl Element {
    Column::new()
        .s(Align::center())
        .s(Width::new(800))
        .s(Background::new().color(hsluv!(0,0,0,5)))
        .item(Column::new()
            .s(Align::left(Default::default()))
            .s(Align::center())
            .s(Padding::new().x(100).y(20))
            .item(Paragraph::new().content("Edit article"))
            .item(title_panel())
            .item(content_text_panel())
            .item(tag_panel())
            .item(tags_view())
        )
        .item(button_panel())
}


//------ Editing Article -------
#[static_ref]
fn edit_article() -> &'static Mutable<Article> {
    let mut user = User { id: "".to_string(), username: "".to_string(), email: "".to_string(), auth_token: "".to_string() };
    let mut cont = vec![user.clone(), user.clone()];
    let tag = Tag {
        id: 5,
        text: "Hei".to_string(),
    };
    let tags = vec![tag.clone(), tag.clone()];
    Mutable::new(
        Article {
            id: 5,
            title: "Hei".to_string(),
            content: "hallo".to_string(),
            author: user.clone(),
            contributors: cont,
            tags: tags,
            created_time: "".to_string(),
            updated_time: "".to_string()
        }
    )
}

// Should be replaced with article ID later
#[static_ref]
fn article_id() -> &'static Mutable<u32> {
    Mutable::new(0)
}

pub fn set_edit_article(art: Article) {
    edit_article().set(art.clone().to_owned());
    title_text().set(art.title.clone());
    content_text().set(art.content.clone());
    tags().lock_mut().replace_cloned(art.tags.clone());
}

//TODO Add error handlers and response to user on article added Ok.
pub fn update_article() {
    Task::start(async {
        let msg = UpMsg::EditArticle {
            // org_title must be replace with ID when that gets implemented for Article object
            id: edit_article().lock_mut().id,
            new_title: title_text().lock_mut().to_string(),
            new_content: content_text().lock_mut().to_string(),
            new_contributors: vec![],
            new_tags: tags().lock_mut().to_vec(),
        };
        if let Err(error) = connection::connection().send_up_msg(msg).await {
            let error = error.to_string();
            //set_error.msg(error.clone());
        }
    });
}

pub fn delete_article() {
    Task::start(async {
        if delete_dialog() {
            let msg = UpMsg::RemoveArticle {
                // Must be replaced with ID when that gets implemented for Article object
                id: article_id().get_cloned(),
            };
            if let Err(error) = connection::connection().send_up_msg(msg).await {
                let error = error.to_string();
                //set_error.msg(error.clone());
            }
        } else {
            return;
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
        .item(delete_button())
        .item(cancel_button())
        .item(publish_button())
        .s(Spacing::new(10))
        .s(Align::right(Default::default()))
}

fn delete_dialog() -> bool {
    let res = window().confirm_with_message("This will permanently delete the article. Are you sure you want to delete it?");
    res.unwrap()
}

fn cancel_dialog() -> bool {
    let res = window().confirm_with_message("Your changes will not be saved. Are you sure you want to leave the page?");
    res.unwrap()
}

fn delete_button() -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Font::new().size(16).color(GRAY_0))
        .s(Background::new()
            .color_signal(hovered_signal.map_bool(|| GRAY_5, || GRAY_9)))
        .s(Padding::new().y(10).x(15))
        .on_hovered_change(move |is_hovered| hovered.set(is_hovered))
        .label("Delete article")
        .on_press(delete_article)
}

fn publish_button() -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Font::new().size(16).color(GRAY_0))
        .s(Background::new()
            .color_signal(hovered_signal.map_bool(|| GRAY_5, || GRAY_9)))
        .s(Padding::new().y(10).x(15))
        .on_hovered_change(move |is_hovered| hovered.set(is_hovered))
        .label("Publish changes")
        .on_press(update_article)
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
    if cancel_dialog() {
        router().go(Route::Root);
    } else {
        return;
    }
}

// ------ tag label and input combined

fn tag_panel() -> impl Element {
    let id = "tag_input";
    Column::new()
        .s(Spacing::new(15))
        .item(tag_label(id))
        .s(Spacing::new(0))
        .item(tag_input(id))
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
fn tags() -> &'static MutableVec<Tag> {
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

fn add_tag() {
    let mut new_tag = new_tag().lock_mut();
    let tag = new_tag.trim();
    if tag.is_empty() {
        return;
    }
    let tag = Tag {
        id: tag_id().to_owned().get_cloned()+1,
        text: tag.to_string(),
    };
    tag_id().update(|id|id+1);
    tags().lock_mut().push_cloned(tag);
    new_tag.clear();
}

fn tags_view() -> impl Element {
    Row::new()
        .items_signal_vec(tags().signal_vec_cloned().map(tag))
        .s(Spacing::new(10))
}

fn tag(tag: Tag) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);

    Row::new()
        .item(Label::new()
            .for_input(tag.id)
            .label(tag.text.to_string())
            .element_on_right( remove_tag_button(&tag)))
        .s(Padding::new().x(10))
        .s(Background::new().color(GRAY_2))
        .s(RoundedCorners::all(10))
}

fn remove_tag(id: u32) {
    tags().lock_mut().retain(|tag| tag.id != id)
}

fn remove_tag_button(tag: &Tag) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    let id = tag.id;
    Button::new()
        .s(Font::new().size(20).color_signal(
            hovered_signal.map_bool(|| GRAY_9, || GRAY_4),
        ))
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .on_press(move || remove_tag(id))
        .label("×")
}