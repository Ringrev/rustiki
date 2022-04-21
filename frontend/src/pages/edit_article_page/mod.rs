use zoon::*;
use zoon::named_color::*;
use shared::{LocalArticle, UpMsg};
use crate::{app, connection};
use crate::app::{logged_user_name};
use crate::router::{Route, router};
use crate::elements::dialogs::*;
use crate::elements::tags;

mod view;

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
            .item(tags::tag_panel())
            .item(tags::tags_view())
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
    tags::tags().lock_mut().replace_cloned(art.tags.clone());
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
            new_tags: tags::tags().lock_mut().to_vec(),
        };
        if let Err(error) = connection::connection().send_up_msg(msg).await {
            message_dialog(error.to_string().as_str());
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
            if confirm_dialog("Are you sure you want to delete the article?") {
                let msg = UpMsg::RemoveArticle {
                    id: article_id().get_cloned(),
                };
                if let Err(error) = connection::connection().send_up_msg(msg).await {
                    message_dialog(error.to_string().as_str());
                }
            } else {
                return;
            }
        });
    } else {
        message_dialog("Only the author can delete an article.")
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
    if confirm_dialog("Your changes will not be saved. Are you sure you want to leave the page?") {
        router().go(Route::Home);
    } else {
        return;
    }
}


