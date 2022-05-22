//! Defines functions used for getting articles from database.
use crate::models::article::Article;
use aragog::*;
use shared::{DownMsg, LocalArticle};
use std::u32;

/// The handler for getting articles from DB. Returns a DownMsg containing a vector of articles.
pub async fn handler(db_conn: &DatabaseConnection) -> DownMsg {
    let art = get_articles(db_conn).await;
    DownMsg::Articles(art)
}

/// Put the articles received from database into a vector containing the sharable struct type LocalArticle
/// Returns this vector populated with LocalArticle objects.
async fn get_articles(db_conn: &DatabaseConnection) -> Vec<LocalArticle> {
    let result = get_articles_from_db(db_conn).await;
    let mut records: Vec<LocalArticle> = vec![];
    for a in &result {
        let art = LocalArticle::new(
            a.id.to_string().parse::<u32>().unwrap(),
            a.title.clone(),
            a.content.clone(),
            a.contributors.clone(),
            a.author.clone(),
            a.tags.clone(),
            a.created_time.to_string(),
            a.updated_time.to_string(),
        );
        records.push(art);
    }
    records
}

/// Gets articles from ArangoDB database using Aragog crate.
///
/// # Arguments
/// * `conn` - A reference to an Aragog DatabaseConnection.
async fn get_articles_from_db(db_conn: &DatabaseConnection) -> Vec<DatabaseRecord<Article>> {
    let query = Article::query();
    let article_records = Article::get(query, db_conn).await.unwrap();
    let records: Vec<DatabaseRecord<Article>> = article_records.to_vec();
    records
}
