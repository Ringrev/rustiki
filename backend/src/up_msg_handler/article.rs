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
        let art = LocalArticle::new(
                                    a.id.to_string().parse::<u32>().unwrap(),
                                    a.title.clone(),
                                    a.content.clone(),
                                    a.contributors.clone(),
                                    a.author.clone(),
                                    a.tags.clone(),
                                    a.created_time.to_string(),
                                    a.updated_time.to_string());
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





