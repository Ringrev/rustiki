use std::fmt::Debug;
use moon::*;
use shared::{DownMsg, User};
use anyhow::Result;
use aragog::{DatabaseConnection, Record};
use aragog::query::{Comparison, Filter, QueryResult};


pub async fn handler(org_title: String, new_title: String, new_content: String) -> DownMsg {
    get_from_db(org_title, new_title, new_content).await;
    DownMsg::ArticleUpdated
    // if res.eq("Ok") {
    //     DownMsg::LoggedIn(user)
    // } else {
    //     DownMsg::LoginError(res)
    // }
}

async fn get_from_db(title: String, new_title: String, new_content: String) {
    let conn = DatabaseConnection::builder()
        .with_credentials("http://174.138.11.103:8529", "_system", "root", "ringrev")
        .with_schema_path("backend/config/db/schema.yaml")
        .apply_schema()
        .build()
        .await
        .unwrap();

    let query = article::query().filter(Filter::new(Comparison::field("title").equals_str(title)));
    let mut art = article::get(query, &conn)
        .await
        .unwrap()
        .uniq()
        .unwrap();
    art.title = new_title;
    art.content = new_content;
    let result = art.save(&conn).await.unwrap();
    println!("Result from updating db after save: {:?}", result);
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