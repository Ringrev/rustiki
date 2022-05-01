use crate::app::logged_user_name;
use crate::elements::dialogs::*;
use crate::elements::{button, panel, tags};
use crate::router::{router, Route};
use crate::connection;
use shared::{LocalArticle, UpMsg};
use zoon::named_color::GRAY_0;
use zoon::*;

mod view;

// ------ ------
//     States
// ------ ------

// ------ state of title

#[static_ref]
fn title_text() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}

// ------ state: content text
#[static_ref]
fn content_text() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}

//------ Article to edit
#[static_ref]
fn edit_article() -> &'static Mutable<LocalArticle> {
    Mutable::new(LocalArticle::new_empty())
}

#[static_ref]
fn article_id() -> &'static Mutable<u32> {
    Mutable::new(0)
}

#[static_ref]
fn contributors() -> &'static MutableVec<String> {
    MutableVec::new()
}

// ------ ------
//     View
// ------ ------

pub fn page() -> impl Element {
    Column::new()
        .s(Align::center())
        .s(Width::new(800))
        .s(Background::new().color(GRAY_0))
        .item(
            Column::new()
                .s(Align::left(Default::default()))
                .s(Align::center())
                .s(Padding::new().x(100).y(20))
                .item(
                    Paragraph::new()
                        .content("Edit article")
                        .s(Font::new().size(20))
                        .s(Padding::bottom(Default::default(), 20)),
                )
                .item(title_panel())
                .item(content_text_panel())
                .item(tags::tag_panel("tag_input_edit_article"))
                .item(tags::tags_view()),
        )
        .item(button_panel())
}

// ------ title label and input

fn title_panel() -> impl Element {
    let id = "title_input";
    panel::input_panel(
        id,
        "Title:",
        set_title,
        "Title of your article",
        InputType::text(),
        title_text().signal_cloned(),
        None,
    )
}

// ------ title label and input combined

fn content_text_panel() -> impl Element {
    panel::textarea_panel(
        "textarea_edit_article",
        set_content_text,
        content_text().signal_cloned(),
    )
}

fn button_panel() -> impl Element {
    Row::new()
        .item(delete_button())
        .item(cancel_button())
        .item(publish_button())
        .s(Spacing::new(10))
        .s(Align::center())
}

fn delete_button() -> impl Element {
    button::button("delete_article", "Delete article", delete_article)
}

fn publish_button() -> impl Element {
    button::button("publish_changes", "Publish changes", update_article)
}

fn cancel_button() -> impl Element {
    button::button("cancel", "Cancel", cancel)
}

// ------ ------
//    Commands
// ------ ------

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

pub fn delete_article() {
    super::delete_article(
        edit_article().get_cloned().author.as_str(),
        article_id().get_cloned(),
    );
}

pub fn set_edit_article(art: LocalArticle) {
    edit_article().set(art.clone().to_owned());
    title_text().set(art.title.clone());
    content_text().set(art.content.clone());
    article_id().set(art.id.to_owned());
    tags::tags().lock_mut().replace_cloned(art.tags.clone());
    contributors()
        .lock_mut()
        .replace_cloned(art.contributors.clone());
}

fn add_contributor() {
    let logged_in_user = logged_user_name().get_cloned().to_string();
    if !edit_article()
        .get_cloned()
        .author
        .to_string()
        .eq(logged_in_user.clone().as_str())
    {
        if !contributors().lock_mut().contains(&logged_in_user.clone()) {
            contributors()
                .lock_mut()
                .push_cloned(logged_in_user.clone());
        }
    }
}

fn cancel() {
    if confirm_dialog("Your changes will not be saved. Are you sure you want to leave the page?") {
        router().go(Route::Home);
    } else {
        return;
    }
}

// ------ ------
//     Helpers
// ------ ------

fn set_title(title: String) {
    title_text().set(title);
}

fn set_content_text(content: String) {
    content_text().set(content);
}
