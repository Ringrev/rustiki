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
        UpMsg::AddArticle { title, content, author, tags } => add_article::handler(title, content, author, tags).await,
        UpMsg::EditArticle { id, new_title, new_content, new_contributors, new_tags} => edit_article::handler(id, new_title, new_content, new_contributors, new_tags).await,
        UpMsg::RemoveArticle { id } => delete_article::handler(id).await,
    })
}
