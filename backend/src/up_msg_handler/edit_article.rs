//! Defines functions used for editing articles in database.
use crate::Article;
use aragog::query::{Comparison, Filter};
use aragog::Record;
use shared::DownMsg;

/// The handler for updating an article in DB. Returns a DownMsg indicating the article was updated.
///
/// # Arguments
/// * `id` - An u32 value holding the id of the article.
/// * `new_title` - A String holding the title of the article.
/// * `new_content` - A String holding the content of the article.
/// * `new_contributors` - A vector of Strings holding the article's contributors.
/// * `new_tags` - A vector of Strings holding the article's tags.
pub async fn handler(
    id: u32,
    new_title: String,
    new_content: String,
    new_contributors: Vec<String>,
    new_tags: Vec<String>,
) -> DownMsg {
    update_in_db(id, new_title, new_content, new_contributors, new_tags).await;
    DownMsg::ArticleUpdated
}

/// Updated the article in the ArangoDB database using Aragog crate.
///
/// # Arguments
/// * `id` - An u32 value holding the id of the article.
/// * `new_title` - A String holding the title of the article.
/// * `new_content` - A String holding the content of the article.
/// * `new_contributors` - A vector of Strings holding the article's contributors.
/// * `new_tags` - A vector of Strings holding the article's tags.
async fn update_in_db(
    id: u32,
    new_title: String,
    new_content: String,
    new_contributors: Vec<String>,
    new_tags: Vec<String>,
) {
    let conn = crate::init_db().await;

    let query = Article::query().filter(Filter::new(Comparison::field("id").equals(id)));
    let mut art = Article::get(query, &conn).await.unwrap().uniq().unwrap();
    art.title = new_title;
    art.content = new_content;
    art.tags = new_tags;
    art.contributors = new_contributors;
    art.updated_time = super::get_time();
    let result = art.save(&conn).await.unwrap();
}
