use std::fmt::Debug;
use std::future::Future;
use moon::*;
use moon::*;
use shared::{DownMsg, Article};
use anyhow::Result;
use aragog::{DatabaseConnection, DatabaseRecord, Record};
use shared::UpMsg::AddArticle;


#[derive(Debug, Serialize, Deserialize, Clone, Record)]
#[serde(crate = "serde")]
pub struct article {
    //pub id: String,
    //pub user: String,
    //pub tags: String
    pub title: String,
    pub content: String,
}

pub async fn handler(title: String, content: String) -> DownMsg {
    let article = create(title.clone(), content.clone()).await;
    //if article.eq("Ok") {
        //Creates an article in the Db
        create_article_in_db(title.clone(), content.clone()).await;


    //}
    DownMsg::ArticleAdded("".to_string())

}



    pub async fn create(title: String, content: String) -> Article {
        let mut article = Article {
            //id: "".to_string(),
            //user: "".to_string(),
            //tags: "".to_string(),
            title: "".to_string(),
            content: "".to_string(),
        };
        article
    }




    pub async fn create_article_in_db(title: String, content: String) {
        let conn = DatabaseConnection::builder()
            .with_credentials("http://174.138.11.103:8529", "_system", "root", "ringrev")
            .with_schema_path("backend/config/db/schema.yaml")
            .apply_schema()
            .build()
            .await
            .unwrap();
        let db_article = article { title, content};
        DatabaseRecord::create(db_article, &conn).await.unwrap();
    }




























// ----Old not working ----
//pub async fn handler(
//    //db: &DatabaseConnection,
//    title: String,
//    content: String,
//) -> Result<DownMsg, Option<DownMsg>> {
 //   async fn aragog_create_article (conn: &DatabaseConnection) {
 //       let mut article = Article {
 //           //TODO:change from when object is defined in frontend!
 //           title: String::from("bacon ipsum"),
 //           content: String::from("lorem ipsum delor"),
 //       };
 //      // let mut article_record = DatabaseRecord::create(article, conn).await.unwrap();
//}
 //   Ok(DownMsg::ArticleAdded)
//}