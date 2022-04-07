use moon::*;
use shared::{DownMsg, Articles};
use anyhow::Result;
use shared::UpMsg::Article; // is this supposed to be here?
// use shared::DownMsg::LoginError;

pub async fn handler(id: String) -> DownMsg {
    println!("article handler received: {}", id);
    // login().map_or_else(|err| LoginError(format!("error: {:?}", err)), DownMsg::LoggedIn)
    let articles = article(); // usikker om rett
    DownMsg::Articles(articles.unwrap())
}

/*pub fn article() -> Vec<Article> {
    let article = Article {
        id: "test id artikkel".to_string(),
    };

    Ok(article)
}*/

let result = aragog_get_all(&connection).await;
let mut records: Vec<test_collection> = vec![];
for user in &result {
println!("{:?}", &user.record);
records.push(user.record.clone());
}

