use moon::*;
use shared::{DownMsg, User};
use anyhow::Result;

pub async fn handler(email: String, password: String) -> DownMsg {
    // login().map_or_else(|err| DownMsg::LoggedIn(user), DownMsg::LoggedIn(user))
    let user = login();
    DownMsg::LoggedIn(user.unwrap())
}

pub fn login() -> Result<User> {
    let user = User {
        id: "Linda".to_string(),
        email: "linda@linda.com".to_string(),
        auth_token: "sefhwiufhriwy2498y9hg".to_string()
    };

    Ok(user)
}