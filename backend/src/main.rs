mod up_msg_handler;
mod firebase;
use shared::{LocalArticle, DownMsg, UpMsg};
use up_msg_handler::add_article;

use moon::*;
use aragog::query::{Comparison, Filter, QueryResult};
use moon::actix_web::web::Data;
use serde::de::Unexpected::Str;
use std::borrow::Borrow;
use aragog::{DatabaseConnection, Record};
use UpMsg::AddArticle;
use crate::add_article::create_article_in_db;
use crate::up_msg_handler::handler;

async fn frontend() -> Frontend {
    Frontend::new()
        .title("Rustiki")
        .append_to_head(
        "
        <style>
            html {
                background-color: white;
                color: black;
            }
        </style>"
        ,
    )
}

async fn up_msg_handler(req: UpMsgRequest<UpMsg>) {
    let (session_id, cor_id) = (req.session_id, req.cor_id);

    let down_msg = match up_msg_handler::handler(req).await {
        Ok(down_msg) | Err(Some(down_msg)) => down_msg,
        _ => return,
    };

    if let Some(session) = sessions::by_session_id().wait_for(session_id).await {
        return session.send_down_msg(&down_msg, cor_id).await;
    }

    println!("Cannot find the session with id `{}`", session_id);
}

#[moon::main]
async fn main() -> std::io::Result<()> {
    start(frontend, up_msg_handler, |_| {}).await?;
    Ok(())
}

//ArangoDb connection
async fn init_db() -> DatabaseConnection {
    DatabaseConnection::builder()
        .build()
        .await
        .unwrap()
}

///! This struct must be used instead of Article struct in shared folder
/// because of an issue implementing Record for structs in shared folder.
/// Name of struct has to match name of collection in DB. Case sensitive.
#[derive(Debug, Serialize, Deserialize, Clone, Record)]
#[serde(crate = "serde")]
pub struct Article {
    pub id: u32,
    pub title: String,
    pub content: String,
    pub contributors: Vec<String>,
    pub author: String,
    pub tags: Vec<String>,
    pub created_time: String,
    pub updated_time: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Record)]
#[serde(crate = "serde")]
pub struct User {
    pub id: String,
    pub email: String,
    pub username: String,
}