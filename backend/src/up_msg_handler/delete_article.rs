//! Defines functions used for deleting articles from database.
use crate::Article;
use aragog::query::{Comparison, Filter};
use aragog::{DatabaseConnection, Record};
use shared::DownMsg;

/// The handler for deleting articles from DB. Returns a DownMsg indicating the article was removed.
///
/// # Arguments
/// * `id` - An u32 value holding the id of the article.
pub async fn handler(id: u32, db_conn: &DatabaseConnection) -> DownMsg {
    remove_from_db(id, db_conn).await;
    DownMsg::ArticleRemoved
}

/// Deletes an article from the ArangoDB database using Aragog crate.
///
/// # Arguments
/// * `id` - An u32 value holding the id of the article to delete.
async fn remove_from_db(id: u32, db_conn: &DatabaseConnection) {
    let query = Article::query().filter(Filter::new(Comparison::field("id").equals(id)));
    let mut art = Article::get(query, db_conn).await.unwrap().uniq().unwrap();
    let result = art.delete(db_conn).await.unwrap();
    result
}
