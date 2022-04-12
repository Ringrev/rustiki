mod up_msg_handler;
mod firebase;
use shared::{Article, DownMsg, UpMsg};
use up_msg_handler::add_article;

use moon::*;
use aragog::query::{Comparison, Filter, QueryResult};
use moon::actix_web::web::Data;
use serde::de::Unexpected::Str;
use std::borrow::Borrow;
use aragog::DatabaseConnection;
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


// ------### ArangoDb-Aragog ##### --------
// Collections in Db: user, article




//ArangoDb connection
async fn aragog_connect() -> DatabaseConnection {
    let db_connection = DatabaseConnection::builder()
        .with_credentials("http://174.138.11.103:8529", "_system", "root", "ringrev")
        .with_schema_path("backend/config/db/schema.yaml")
        .apply_schema()
        .build()
        .await
        .unwrap();
    db_connection
}
// Returns all as query result from collection: user
//async fn aragog_get_all_query_result(conn: &DatabaseConnection) -> QueryResult<user> {
//    let query = user::query();
//    let user_records = user::get(query, conn).await.unwrap();
//    user_records
//}


#[moon::main]
async fn main() -> std::io::Result<()> {
    let connection = aragog_connect().await;

    //Gets all entries as query result
   // let records = aragog_get_all_query_result(&connection).await;
   // println!("{:?}", records);



    start(frontend, up_msg_handler, |_| {}).await?;
    Ok(())
}


