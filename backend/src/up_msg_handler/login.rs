use std::fmt::Debug;
use moon::*;
use shared::{DownMsg, User};
use anyhow::Result;
use fireauth::FireAuth;
use moon::futures::future::err;
use moon::futures::TryFutureExt;
use shared::DownMsg::LoginError;

pub async fn handler(auth: FireAuth, email: String, password: String) -> DownMsg {
    let (res, user) = login(auth, email, password)
        .await;
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
        auth_token: "".to_string()
    };
    match auth.sign_in_email(&*email, &*password, true).await {
        Ok(response) => {
            res = String::from("Ok");
            println!("{:?}", response);
            user = User {
                id: response.expires_in.unwrap().to_string(),
                email: response.email.to_string(),
                auth_token: response.id_token.to_string()
            }
        }
        Err(error) => {  println!("Error from firebase: {:?}", error.clone());
        res = error.clone().to_string(); }
    }

    (res, user)
}