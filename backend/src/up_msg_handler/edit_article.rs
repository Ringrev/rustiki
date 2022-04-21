use shared::DownMsg;
use aragog::Record;
use aragog::query::{Comparison, Filter};
use crate::Article;

pub async fn handler(id: u32, new_title: String, new_content: String, new_contributors: Vec<String>, new_tags: Vec<String>) -> DownMsg {
    update_in_db(id, new_title, new_content, new_contributors, new_tags).await;
    DownMsg::ArticleUpdated
    // if res.eq("Ok") {
    //     DownMsg::LoggedIn(user)
    // } else {
    //     DownMsg::LoginError(res)
    // }
}

async fn update_in_db(id: u32, new_title: String, new_content: String, new_contributors: Vec<String>, new_tags: Vec<String>) {
    let conn = crate::init_db().await;

    let query = Article::query().filter(Filter::new(Comparison::field("id").equals(id)));
    let mut art = Article::get(query, &conn)
        .await
        .unwrap()
        .uniq()
        .unwrap();
    art.title = new_title;
    art.content = new_content;
    art.tags = new_tags;
    art.contributors = new_contributors;
    art.updated_time = super::get_time();
    let result = art.save(&conn).await.unwrap();
    println!("Result from updating db after save: {:?}", result);
}
