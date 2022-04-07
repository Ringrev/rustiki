use moon::*;
use shared::{UpMsg, DownMsg};
use crate::login;

pub async fn handler(req: UpMsgRequest<UpMsg>) -> Result<DownMsg, Option<DownMsg>> {
    Ok(match req.up_msg {
        UpMsg::Login { email, password } => login::handler(email, password).await,
    })
}
