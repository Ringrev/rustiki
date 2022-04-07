use moon::*;
use shared::{DownMsg, User};
use anyhow::Result;
use shared::DownMsg::LoginError;

pub async fn handler(email: String, password: String) -> DownMsg {
    println!("login handler received: {}, {}", email, password);
    login().map_or_else(|err| LoginError(format!("error: {:?}", err)), DownMsg::LoggedIn)
    // let user = login();
    // DownMsg::LoggedIn(user.unwrap())
}

pub fn login() -> Result<User> {
    let user = User {
        id: "Linda".to_string(),
        email: "linda@linda.com".to_string(),
        auth_token: "sefhwiufhriwy2498y9hg".to_string()
    };

    Ok(user)
}