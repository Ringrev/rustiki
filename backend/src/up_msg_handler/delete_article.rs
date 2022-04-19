use std::fmt::Debug;
use std::time::SystemTime;
use moon::*;
use shared::{DownMsg, LocalUser};
use anyhow::Result;
use aragog::{DatabaseConnection, Record};
use aragog::query::{Comparison, Filter, QueryResult};
use crate::Article;

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
    let conn = crate::init_db().await;

    let query = Article::query().filter(Filter::new(Comparison::field("id").equals(id)));
    let mut art = Article::get(query, &conn)
        .await
        .unwrap()
        .uniq()
        .unwrap();
    let result = art.delete(&conn).await.unwrap();
    println!("Result from updating db after save: {:?}", result);
}