use std::time::SystemTime;
use moon::*;
use shared::{LocalArticle, DownMsg, LocalUser};
use anyhow::Result;
use shared::UpMsg::{GetArticles}; // is this supposed to be here?
use aragog::*;
use moon::serde_json::from_str;
use std::u32;
// use shared::DownMsg::LoginError;
use crate::Article;

pub async fn handler() -> DownMsg {
    let art = articles().await;
    DownMsg::Articles(art)
}

pub async fn articles() -> Vec<LocalArticle> {
    let conn = crate::init_db().await;

    let result = aragog_get_all(&conn).await;
    let mut records: Vec<LocalArticle> = vec![];
    for a in &result {
        let id: String = a.id.to_string();
        let art = LocalArticle {
            id: id.parse::<u32>().unwrap(),
            title: a.title.clone(),
            content: a.content.clone(),
            contributors: a.contributors.clone(),
            author: a.author.clone(),
            tags: a.tags.clone(),
            created_time: a.created_time.to_string(),
            updated_time: a.updated_time.to_string(),
        };
        records.push(art);
    }
    records
}

async fn aragog_get_all(conn: &DatabaseConnection) -> Vec<DatabaseRecord<Article>> {
    let query = Article::query();
    let article_records = Article::get(query, conn).await.unwrap();
    let records: Vec<DatabaseRecord<Article>> = article_records.to_vec();
    records
}





