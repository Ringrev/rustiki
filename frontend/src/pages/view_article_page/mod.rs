//! Defines the non-visual content and operations for view edit article page.
use crate::pages::edit_article_page;
use crate::router::{router, Route};
use shared::LocalArticle;
use zoon::*;

mod view;

// ------ ------
//     States
// ------ ------

/// Current article. Initialized with an empty LocalArticle.
#[static_ref]
pub fn view_article() -> &'static Mutable<LocalArticle> {
    Mutable::new(LocalArticle::new_empty())
}

/// Current article's id. Initialized with 0.
#[static_ref]
fn article_id() -> &'static Mutable<u32> {
    Mutable::new(0)
}

/// Current article's tags.
#[static_ref]
fn tags() -> &'static MutableVec<String> {
    MutableVec::new()
}

/// Current article's contributors.
#[static_ref]
fn contributors() -> &'static MutableVec<String> {
    MutableVec::new()
}

// ------ ------
//     Commands
// ------ ------

/// Sets the article to be edited to the one being viewed, and directs user to edit page.
pub fn edit_article() {
    edit_article_page::set_edit_article(view_article().get_cloned());
    router().go(Route::EditArticle);
}

/// Sets which LocalArticle should be viewed.
///
/// # Arguments
/// * `art` - The LocalArticle to be viewed.
pub fn set_view_article(art: LocalArticle) {
    tags().lock_mut().replace_cloned(art.tags.to_vec());
    view_article().set(art.clone().to_owned());
    article_id().set(art.id.to_owned());
    contributors()
        .lock_mut()
        .replace_cloned(art.contributors.clone())
}

/// Deletes an article from database as defined in pages::mod module.
pub fn delete_article() {
    super::delete_article(
        view_article().get_cloned().author.as_str(),
        article_id().get_cloned(),
    );
}

// ------ ------
//     View
// ------ ------

pub fn view() -> RawElement {
    view::page().into_raw_element()
}
