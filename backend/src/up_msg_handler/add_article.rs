use std::fmt::Debug;
use std::future::Future;
use moon::*;
use moon::*;
use shared::{DownMsg, LocalUser, LocalArticle};
use anyhow::Result;
use aragog::{DatabaseConnection, DatabaseRecord, Record};
use aragog::query::{Comparison, Filter};
use shared::UpMsg::AddArticle;
use rand::Rng;
use std::time::SystemTime;
use crate::Article;

pub async fn handler(title: String, content: String,  author: String, tags: Vec<String>) -> DownMsg {
    let article = create_object(title, content, author, tags).await;
    //if article.eq("Ok") {
    //Creates an article in the Db
    create_article_in_db(article).await;
    DownMsg::ArticleAdded("".to_string())
}

pub async fn create_object(title: String, content: String,  author: String, tags: Vec<String>) -> LocalArticle {
    LocalArticle::new(generate_id().await, title, content, vec![], author, tags, get_time(), get_time())
}

fn get_time() -> String {
    let system_time = SystemTime::now();
    let datetime: DateTime<Local> = system_time.into();
    datetime.format("%d.%m.%Y %T").to_string()
}

async fn check_id_unique(id: u32) -> bool {
    let conn = crate::init_db().await;

    let query = Article::query().filter(Filter::new(Comparison::field("id").equals(id)));
    let art = Article::get(query, &conn)
        .await
        .unwrap();
    if art.is_empty() {
        true
    } else {
        false
    }
}

pub async fn create_article_in_db(art: LocalArticle) {
    let conn = crate::init_db().await;
    let db_article = Article::new(art.id, art.title, art.content, art.contributors, art.author, art.tags, art.created_time, art.updated_time);
    DatabaseRecord::create(db_article, &conn).await.unwrap();
}

async fn generate_id() -> u32 {
    let mut rand = rand::thread_rng();
    let mut id: u32 = 0;
    let mut checking = true;
    while checking {
        id = rand.gen::<u32>();
        if check_id_unique(id.clone()).await {
            checking = false;
        }
    }
    id.clone()
}