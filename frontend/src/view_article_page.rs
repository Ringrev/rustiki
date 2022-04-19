use zoon::{*, println};
use zoon::events::Input;
use zoon::named_color::*;
use zoon::text_input::InputTypeText;
use zoon::web_sys::HtmlTextAreaElement;
use shared::{LocalArticle, UpMsg, LocalUser};
use shared::UpMsg::AddArticle;
use crate::{app, connection, edit_article_page};
use crate::app::dialog;
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
                  .item(Row::new()
                      .item(Paragraph::new().content("Author: ").s(Font::new().size(12)))
                      .item(author_view()))
                  .item(Row::new()
                            .item(Paragraph::new().content("Contributors: ").s(Font::new().size(12)))
                      .item(contributors_view()))
            .item(Paragraph::new().content(view_article().get_cloned().title).s(Font::new().size(20))
                .s(Padding::new().top(20)))
            .item(Paragraph::new().content(view_article().get_cloned().content)
                .s(Padding::new().bottom(20).top(10)))
                  .item(Row::new()
                      .item(Paragraph::new().content("Tags: ").s(Font::new().size(12)))
                      .item(tags_view()))
                  .item(time_view())
            // .item(content_text_panel())
            // .item(tag_panel())
            // .item(tags_view())

        )

        .item(button_panel())
}

fn edit_button() -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Font::new().size(16).color(GRAY_0))
        .s(Background::new()
            .color_signal(hovered_signal.map_bool(|| GRAY_5, || GRAY_9)))
        .s(Padding::new().y(10).x(15))
        .on_hovered_change(move |is_hovered| hovered.set(is_hovered))
        .label("Edit")
        .on_press(move || edit_article(view_article().get_cloned()))
}

fn delete_button() -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Font::new().size(16).color(GRAY_0))
        .s(Background::new()
            .color_signal(hovered_signal.map_bool(|| GRAY_5, || GRAY_9)))
        .s(Padding::new().y(10).x(15))
        .on_hovered_change(move |is_hovered| hovered.set(is_hovered))
        .label("Delete")
        .on_press(delete_article)
}

pub fn edit_article(article: LocalArticle) {
    edit_article_page::set_edit_article(article.clone());
    router().go(Route::EditArticle);
}

//------ View Article -------
#[static_ref]
fn view_article() -> &'static Mutable<LocalArticle> {
    Mutable::new(
        LocalArticle {
            id: 5,
            title: "Hei".to_string(),
            content: "hallo".to_string(),
            author: "".to_string(),
            contributors: vec![],
            tags: vec![],
            created_time: "".to_string(),
            updated_time: "".to_string()
        }
    )
}

#[static_ref]
fn article_id() -> &'static Mutable<u32> {
    Mutable::new(0)
}

pub fn set_view_article(art: LocalArticle) {
    tags().lock_mut().replace_cloned(art.tags.to_vec());
    view_article().set(art.clone().to_owned());
    article_id().set(art.clone().id);
    contributors().lock_mut().replace_cloned(art.contributors.clone())
}

pub fn delete_article() {
    if app::logged_user_name().get_cloned().eq(view_article().get_cloned().author.as_str()) {
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

fn button_panel() -> impl Element {
    Row::new()
        .item_signal(app::is_user_logged_signal().map_true(edit_button))
        .item_signal(app::is_user_logged_signal().map_true(delete_button))
        .s(Spacing::new(10))
        .s(Align::right(Default::default()))
}

fn delete_dialog() -> bool {
    let res = window().confirm_with_message("This will permanently delete the article. Are you sure you want to delete it?");
    res.unwrap()
}

#[static_ref]
fn tags() -> &'static MutableVec<String> {
    MutableVec::new()
}

fn tags_view() -> impl Element {
    Row::new()
        .items_signal_vec(tags().signal_vec_cloned().map(tag))
        .s(Spacing::new(10))
        .s(Padding::new().y(5))
}

fn tag(tag: String) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);

    Row::new()
        .item(Label::new()
            .label(tag.clone().to_string())
        .s(Padding::new().x(10))
            .s(Font::new().size(12))
        .s(Background::new().color(GRAY_2))
        .s(RoundedCorners::all(10)))
}

#[static_ref]
fn contributors() -> &'static MutableVec<String> {
    MutableVec::new()
}

fn contributors_view() -> impl Element {
    Row::new()
        .multiline()
        // .s(Background::new().color(GRAY_0))
        .items_signal_vec(contributors().signal_vec_cloned().map(contributor))
        .s(Spacing::new(10))
}

fn contributor(cont: String) -> impl Element {
    Row::new()
        .item(Label::new()
            .label(cont.clone().to_string())
            .s(Font::new().size(12))
            .s(Padding::new().x(10))
            .s(Background::new().color(GRAY_2))
            .s(RoundedCorners::all(10)))
}

fn author_view() -> impl Element {
    Row::new()
        .item(Label::new()
            .label(view_article().get_cloned().author)
            .s(Font::new().size(12))
            .s(Padding::new().x(10))
            .s(Background::new().color(GRAY_2))
            .s(RoundedCorners::all(10))
        )
        .s(Padding::new().bottom(5))
}

fn time_view() -> impl Element {
    Column::new()
        .item(Paragraph::new()
            .content("Last updated: ".to_string() + view_article().get_cloned().updated_time.as_str())
            .s(Font::new().size(12)))
        .s(Padding::new().bottom(5))
        .item(Paragraph::new().content("Created: ".to_string() + view_article().get_cloned().created_time.as_str()).s(Font::new().size(12)))
        .s(Align::left(Default::default()))
}
