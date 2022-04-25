use crate::{app, connection};
use zoon::*;
use shared::UpMsg;
use crate::elements::dialogs::{confirm_dialog, message_dialog};

pub mod home_page;
pub mod log_in_page;
pub mod registration_page;
pub mod view_article_page;
pub mod create_article_page;
pub mod edit_article_page;


pub fn delete_article(author: &str, article_id: u32) {
    if app::logged_user_name().get_cloned().eq(author) {
        Task::start(async move {
            if confirm_dialog("Are you sure you want to delete the article?") {
                let msg = UpMsg::RemoveArticle {
                    id: article_id.clone(),
                };
                if let Err(error) = connection::connection().send_up_msg(msg).await {
                    message_dialog(error.to_string().as_str());
                }
            } else {
                return;
            }
        });
    } else {
        message_dialog("Only the author can delete an article.")
    }
}