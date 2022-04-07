use moon::*;
use shared::{UpMsg, DownMsg};
use fireauth;
use crate::firebase::init;

mod login;
mod registration;

// What you receive from frontend and what you do with it
pub async fn handler(req: UpMsgRequest<UpMsg>) -> Result<DownMsg, Option<DownMsg>> {
    Ok(match req.up_msg {
        UpMsg::Login { email, password } => login::handler(init().await, email, password).await,
        UpMsg::Register { email, password, username } => registration::handler(init().await, email, password, username).await,
    })
}
