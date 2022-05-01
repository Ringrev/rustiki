use crate::Article;
use aragog::query::{Comparison, Filter};
use aragog::Record;
use shared::DownMsg;

pub async fn handler(id: u32) -> DownMsg {
    remove_from_db(id).await;
    DownMsg::ArticleRemoved
}

async fn remove_from_db(id: u32) {
    let conn = crate::init_db().await;

    let query = Article::query().filter(Filter::new(Comparison::field("id").equals(id)));
    let mut art = Article::get(query, &conn).await.unwrap().uniq().unwrap();
    let result = art.delete(&conn).await.unwrap();
    result
}
