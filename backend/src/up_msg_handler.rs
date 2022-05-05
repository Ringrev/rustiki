//! Handles requests received from frontend crate.
use crate::firebase::init;
use moon::*;
use shared::{DownMsg, UpMsg};
use std::time::SystemTime;

mod add_article;
mod article;
mod delete_article;
mod edit_article;
mod login;
mod registration;

/// Defines which function to execute when an UpMsgRequest is received by matching the UpMsg enum.
/// Returns a Result after handling UpMsg requests.
///
/// # Arguments
/// * `req` - An UpMsgRequest containing the UpMsg from frontend crate.
pub async fn handler(req: UpMsgRequest<UpMsg>) -> Result<DownMsg, Option<DownMsg>> {
    Ok(match req.up_msg {
        UpMsg::GetArticles => article::handler().await,
        UpMsg::Login { email, password } => login::handler(init().await, email, password).await,
        UpMsg::Register {
            email,
            password,
            username,
        } => registration::handler(init().await, email, password, username).await,
        UpMsg::AddArticle {
            title,
            content,
            author,
            tags,
        } => add_article::handler(title, content, author, tags).await,
        UpMsg::EditArticle {
            id,
            new_title,
            new_content,
            new_contributors,
            new_tags,
        } => edit_article::handler(id, new_title, new_content, new_contributors, new_tags).await,
        UpMsg::RemoveArticle { id } => delete_article::handler(id).await,
    })
}

/// Returns current time as a String.
fn get_time() -> String {
    let system_time = SystemTime::now();
    let datetime: DateTime<Local> = system_time.into();
    datetime.format("%d.%m.%Y %T").to_string()
}
