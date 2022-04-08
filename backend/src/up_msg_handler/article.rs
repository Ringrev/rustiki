use moon::*;
use shared::{DownMsg};
use anyhow::Result;
use shared::UpMsg::{GetArticles}; // is this supposed to be here?
use aragog::*;
// use shared::DownMsg::LoginError;

pub async fn handler(id: String) -> DownMsg {
    println!("article handler received: {}", id);
    // login().map_or_else(|err| LoginError(format!("error: {:?}", err)), DownMsg::LoggedIn)
    let articles = article().await;
    DownMsg::GetArticles(articles)
}

#[derive(Debug, Serialize, Deserialize, Record, Clone)]
#[serde(crate = "serde")]
pub struct article {
    pub id: String,
}

pub async fn article() -> Vec<std::string::String> {
    /*let articles = GetArticles {
        id: "test id artikkel".to_string(),
    };

    Ok(response) => {
        res = String::from("Ok");
        println!("{:?}", response);
        user = User {
            id: response.id.to_string()
        }*/

    let conn = DatabaseConnection::builder()
        .with_credentials("http://174.138.11.103:8529/", "_system", "root", "ringrev")
        .with_schema_path("backend/config/db/schema.yaml")
        .apply_schema()
        .build()
        .await
        .unwrap();

    let result = aragog_get_all(&conn).await;
    let mut records: Vec<String> = vec![];
    for a in &result {
        println!("{:?}", &a.record);
        records.push(a.record.id.to_string());
    }
    records
}

async fn aragog_get_all(conn: &DatabaseConnection) -> Vec<DatabaseRecord<article>> {
    let query = article::query();
    let article_records = article::get(query, conn).await.unwrap();
    let records: Vec<DatabaseRecord<article>> = article_records.to_vec();
    records
}



