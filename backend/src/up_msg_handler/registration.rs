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
    pub auth_token: String,
}

pub async fn handler(auth: FireAuth, email: String, password: String, username: String) -> DownMsg {
    let res = register(auth, email.clone(), password.clone()).await;
    if res.eq("Ok") {
        println!("User created: {:?}", res);
        let (result, user) = login(firebase::init().await, email.clone(), password).await;
        if result.eq("Ok") {
            // TODO: create a User object in database
            let conn = DatabaseConnection::builder()
                .with_credentials("http://174.138.11.103:8529", "_system", "root", "ringrev")
                .with_schema_path("backend/config/db/schema.yaml")
                .apply_schema()
                .build()
                .await
                .unwrap();
            let db_user = user {
                id: user.clone().id,
                email: email.clone(),
                username,
                auth_token: user.clone().auth_token,
            };
            DatabaseRecord::create(db_user, &conn).await.unwrap();

            DownMsg::LoggedIn(user.clone())
        } else {
            DownMsg::LoginError(result)
        }
    } else {
        DownMsg::LoginError(res)
    }
}

pub async fn register(auth: FireAuth, email: String, password: String) -> String {
    let mut res: String = "".to_string();
    match auth.sign_up_email(&*email, &*password, true).await {
        Ok(..) => { res = "Ok".to_string() }
        Err(error) => { res = error.to_string() }
    }
    res
}