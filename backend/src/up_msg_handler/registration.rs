use std::fmt::Debug;
use std::future::Future;
use moon::*;
use shared::{DownMsg, User};
use anyhow::Result;
use aragog::{DatabaseConnection, DatabaseRecord, Record};
use fireauth::{Error, FireAuth};
use moon::futures::future::err;
use shared::DownMsg::LoginError;
use crate::up_msg_handler::login;
use crate::up_msg_handler::login::login;
use crate::firebase;

#[derive(Debug, Serialize, Deserialize, Clone, Record)]
#[serde(crate = "serde")]
pub struct user {
    pub id: String,
    pub email: String,
    pub username: String,
}

pub async fn handler(auth: FireAuth, email: String, password: String, username: String) -> DownMsg {
    let (res, user) = register(auth, email.clone(), password.clone()).await;
    if res.eq("Ok") {
        // Creates a User object in database
        create_user_in_db(user.id, email.clone(), username).await;
        println!("User created: {:?}", res);
        let (result, user) = login(firebase::init().await, email.clone(), password).await;
        if result.eq("Ok") {
            DownMsg::LoggedIn(user.clone())
        } else {
            DownMsg::LoginError(result)
        }
    } else {
        DownMsg::LoginError(res)
    }
}

pub async fn register(auth: FireAuth, email: String, password: String) -> (String, User) {
    let mut user = User {
        id: "".to_string(),
        email: "".to_string(),
        username: "".to_string(),
        auth_token: "".to_string()
    };
    let mut res: String = "".to_string();
    match auth.sign_up_email(&*email, &*password, true).await {
        Ok(response) => {
            res = String::from("Ok");
            println!("{:?}", response);
            user = User {
                id: response.local_id.to_string().clone(),
                email: response.email.to_string(),
                username: "".to_string(),
                auth_token: response.id_token.to_string()
            }
        }
        Err(error) => { res = error.to_string() }
    }
    (res, user)
}

async fn create_user_in_db(id: String, email: String, username: String) {
    let conn = DatabaseConnection::builder()
        .with_credentials("http://174.138.11.103:8529", "_system", "root", "ringrev")
        .with_schema_path("backend/config/db/schema.yaml")
        .apply_schema()
        .build()
        .await
        .unwrap();
    let db_user = user { id, email, username };
    DatabaseRecord::create(db_user, &conn).await.unwrap();
}