use std::fmt::Debug;
use moon::*;
use shared::{DownMsg, User};
use anyhow::Result;
use aragog::{DatabaseConnection, Record};
use aragog::query::{Comparison, Filter};
use fireauth::FireAuth;
use moon::futures::future::err;
use shared::DownMsg::LoginError;
use crate::up_msg_handler::registration::user;


pub async fn handler(auth: FireAuth, email: String, password: String) -> DownMsg {
    let (res, user) = login(auth, email, password).await;
    if res.eq("Ok") {
        DownMsg::LoggedIn(user)
    } else {
        DownMsg::LoginError(res)
    }
}

pub async fn login(auth: FireAuth, email: String, password: String) -> (String, User) {
    let mut res: String = "".to_string();
    let mut user = User {
        id: "".to_string(),
        email: "".to_string(),
        username: "".to_string(),
        auth_token: "".to_string()
    };
    match auth.sign_in_email(&*email, &*password, true).await {
        Ok(response) => {
            res = String::from("Ok");
            println!("{:?}", response);
            user = User {
                id: response.local_id.to_string().clone(),
                email: response.email.to_string(),
                username: get_username(response.local_id.to_string()).await,
                auth_token: response.id_token.to_string()
            }
        }
        Err(error) => {  println!("Error from firebase: {:?}", error.clone());
        // res = error.clone().to_string();
        res = String::from("Incorrect input, please try again.")}
    }

    (res, user)
}

async fn get_username(id: String) -> String {
    let conn = DatabaseConnection::builder()
        .with_credentials("http://174.138.11.103:8529", "_system", "root", "ringrev")
        .with_schema_path("backend/config/db/schema.yaml")
        .apply_schema()
        .build()
        .await
        .unwrap();
    let query = user::query().filter(Filter::new(Comparison::field("id").equals_str(id.as_str())));
    let user_record = user::get(query, &conn).await.unwrap().uniq().unwrap();
    let res = user_record.username.to_string();
    res
}