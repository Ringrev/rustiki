use zoon::{*, println};
use zoon::events::Input;
use zoon::named_color::*;
use zoon::text_input::InputTypeText;
use zoon::web_sys::HtmlTextAreaElement;
use shared::{LocalArticle, UpMsg, LocalUser};
use shared::UpMsg::AddArticle;
use crate::{app, connection};
use crate::app::{dialog, logged_user_name, view_article};
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
fn edit_article() -> &'static Mutable<LocalArticle> {
    Mutable::new(
        LocalArticle::new_empty()
    )
}

#[static_ref]
fn article_id() -> &'static Mutable<u32> {
    Mutable::new(0)
}

#[static_ref]
fn contributors() -> &'static MutableVec<String> {
    MutableVec::new()
}

pub fn set_edit_article(art: LocalArticle) {
    edit_article().set(art.clone().to_owned());
    title_text().set(art.title.clone());
    content_text().set(art.content.clone());
    article_id().set(art.id.to_owned());
    tags().lock_mut().replace_cloned(art.tags.clone());
    contributors().lock_mut().replace_cloned(art.contributors.clone());
}

pub fn update_article() {
    add_contributor();
    Task::start(async {
        let msg = UpMsg::EditArticle {
            // org_title must be replace with ID when that gets implemented for Article object
            id: article_id().get_cloned(),
            new_title: title_text().lock_mut().to_string(),
            new_content: content_text().lock_mut().to_string(),
            new_contributors: contributors().lock_mut().to_vec(),
            new_tags: tags().lock_mut().to_vec(),
        };
        if let Err(error) = connection::connection().send_up_msg(msg).await {
            let error = error.to_string();
            //set_error.msg(error.clone());
        }
    });
}

fn add_contributor() {
    let logged_in_user = logged_user_name().get_cloned().to_string();
    if !edit_article().get_cloned().author.to_string().eq(logged_in_user.clone().as_str()) {
        if !contributors().lock_mut().contains(&logged_in_user.clone()) {
            contributors().lock_mut().push_cloned(logged_in_user.clone());
        }
    }
}

pub fn delete_article() {
    if app::logged_user_name().get_cloned().eq(edit_article().get_cloned().author.as_str()) {
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
    } else {
        dialog("Only the author can delete an article".to_string());
    }
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

fn tag_panel() -> impl Element {
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
fn tags() -> &'static MutableVec<String> {
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
        window().alert_with_message("Tag already exists");
        new_tag.clear();
    }
}

fn tags_view() -> impl Element {
    Row::new()
        .items_signal_vec(tags().signal_vec_cloned().map(tag))
        .s(Spacing::new(10))
}

fn tag(tag: String) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);

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