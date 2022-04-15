use std::time::SystemTime;
use moon::*;
use shared::{Article, DownMsg, User};
use anyhow::Result;
use shared::UpMsg::{GetArticles}; // is this supposed to be here?
use aragog::*;
use moon::serde_json::from_str;
use std::u32;
// use shared::DownMsg::LoginError;

pub async fn handler() -> DownMsg {
    let art = articles().await;
    DownMsg::Articles(art)
}

#[derive(Debug, Serialize, Deserialize, Clone, Record)]
#[serde(crate = "serde")]
pub struct article {
    pub id: u32,
    pub title: String,
    pub content: String,
    pub contributors: Vec<String>,
    pub author: String,
    pub tags: Vec<String>,
    pub created_time: String,
    pub updated_time: String,
}

pub async fn articles() -> Vec<Article> {
    let conn = DatabaseConnection::builder()
        .with_credentials("http://174.138.11.103:8529/", "_system", "root", "ringrev")
        .with_schema_path("backend/config/db/schema.yaml")
        .apply_schema()
        .build()
        .await
        .unwrap();

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



