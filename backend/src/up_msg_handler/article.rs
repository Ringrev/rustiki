use std::time::SystemTime;
use moon::*;
use shared::{Article, DownMsg, User};
use anyhow::Result;
use shared::UpMsg::{GetArticles}; // is this supposed to be here?
use aragog::*;
use moon::serde_json::from_str;
use std::u32;
// use shared::DownMsg::LoginError;
use crate::article;

pub async fn handler() -> DownMsg {
    let art = articles().await;
    DownMsg::Articles(art)
}

pub async fn articles() -> Vec<Article> {
    let conn = crate::init_db().await;

    let result = aragog_get_all(&conn).await;
    let mut records: Vec<Article> = vec![];
    for a in &result {
        let id: String = a.id.to_string();
        let art = Article {
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

async fn aragog_get_all(conn: &DatabaseConnection) -> Vec<DatabaseRecord<article>> {
    let query = article::query();
    let article_records = article::get(query, conn).await.unwrap();
    let records: Vec<DatabaseRecord<article>> = article_records.to_vec();
    records
}





