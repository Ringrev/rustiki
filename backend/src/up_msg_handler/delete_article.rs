use std::fmt::Debug;
use std::time::SystemTime;
use moon::*;
use shared::{DownMsg, Tag, User};
use anyhow::Result;
use aragog::{DatabaseConnection, Record};
use aragog::query::{Comparison, Filter, QueryResult};


pub async fn handler(id: u32) -> DownMsg {
    remove_from_db(id).await;
    DownMsg::ArticleRemoved
    // if res.eq("Ok") {
    //     DownMsg::LoggedIn(user)
    // } else {
    //     DownMsg::LoginError(res)
    // }
}

async fn remove_from_db(id: u32) {
    let conn = DatabaseConnection::builder()
        .with_credentials("http://174.138.11.103:8529", "_system", "root", "ringrev")
        .with_schema_path("backend/config/db/schema.yaml")
        .apply_schema()
        .build()
        .await
        .unwrap();

    let query = article::query().filter(Filter::new(Comparison::field("id").equals(id)));
    let mut art = article::get(query, &conn)
        .await
        .unwrap()
        .uniq()
        .unwrap();
    let result = art.delete(&conn).await.unwrap();
    println!("Result from updating db after save: {:?}", result);
}


#[derive(Debug, Serialize, Deserialize, Clone, Record)]
#[serde(crate = "serde")]
pub struct article {
    pub id: u32,
    pub title: String,
    pub content: String,
    pub contributors: Vec<User>,
    pub author: User,
    pub tags: Vec<Tag>,
    pub created_time: String,
    pub updated_time: String,
}