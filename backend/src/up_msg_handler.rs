use moon::*;
use shared::{UpMsg, DownMsg};
mod login;
mod article;
use fireauth;
use crate::firebase::init;
pub(crate) mod add_article;
mod registration;
mod edit_article;
mod delete_article;

// What you receive from frontend and what you do with it
pub async fn handler(req: UpMsgRequest<UpMsg>) -> Result<DownMsg, Option<DownMsg>> {
    Ok(match req.up_msg {
        UpMsg::GetArticles => article::handler().await,
        UpMsg::Login { email, password } => login::handler(init().await, email, password).await,
        UpMsg::Register { email, password, username } => registration::handler(init().await, email, password, username).await,
        UpMsg::AddArticle { title, content} => add_article::handler(
            //db,
            title,
            content,
        ).await,
        UpMsg::EditArticle { org_title, new_title, new_content} => edit_article::handler(org_title, new_title, new_content).await,
        UpMsg::RemoveArticle { title } => delete_article::handler(title).await,
    })
}
