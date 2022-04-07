use moon::*;
use shared::{DownMsg, Article};
use anyhow::Result;
use aragog::{DatabaseConnection, DatabaseRecord};


pub async fn handler(
    db: &DatabaseConnection,
    title: String,
    content: String,
) -> Result<DownMsg, Option<DownMsg>> {
    let mut article =  Article {
        //TODO:change from when object is defined in frontend!
        title: String::from("bacon ipsum"),
        content: String::from("lorem ipsum delor"),
    };
    let mut article_record = DatabaseRecord::create(article, conn).await.unwrap();
}