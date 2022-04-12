use moon::*;
use shared::{Article, DownMsg};
use anyhow::Result;
use shared::UpMsg::{GetArticles}; // is this supposed to be here?
use aragog::*;
// use shared::DownMsg::LoginError;

pub async fn handler() -> DownMsg {
    let art = articles().await;
    DownMsg::Articles(art)
}

#[derive(Debug, Serialize, Deserialize, Clone, Record)]
#[serde(crate = "serde")]
pub struct article {
    //pub id: String,
    //pub user: String,
    //pub tags: String
    pub title: String,
    pub content: String,
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
        let art = Article {
            title: a.title.clone(),
            content: a.content.clone(),
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



