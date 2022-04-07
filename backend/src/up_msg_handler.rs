use moon::*;
use shared::{UpMsg, DownMsg};
mod login;
use fireauth;
use crate::firebase::init;

// What you receive from frontend and what you do with it
pub async fn handler(req: UpMsgRequest<UpMsg>) -> Result<DownMsg, Option<DownMsg>> {
    Ok(match req.up_msg {
        UpMsg::Login { email, password } => login::handler(init().await, email, password).await,
    })
}
