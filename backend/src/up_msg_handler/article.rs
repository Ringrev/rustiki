use crate::Article;
use aragog::*;
use shared::{DownMsg, LocalArticle};
use std::u32;

pub async fn handler() -> DownMsg {
    let art = articles().await;
    DownMsg::Articles(art)
}

pub async fn articles() -> Vec<LocalArticle> {
    let conn = crate::init_db().await;

    let result = get_articles_from_db(&conn).await;
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

async fn get_articles_from_db(conn: &DatabaseConnection) -> Vec<DatabaseRecord<Article>> {
    let query = Article::query();
    let article_records = Article::get(query, conn).await.unwrap();
    let records: Vec<DatabaseRecord<Article>> = article_records.to_vec();
    records
}
