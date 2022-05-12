//! Represents the connection between frontend and backend.
//! Defines what happens when frontend receives messages from backend.
use crate::pages::{home_page, log_in_page, registration_page};
use crate::router::Route;
use crate::*;
use router::router;
use shared::{DownMsg, UpMsg};

/// State of Connection to backend.
/// Defines what to do when messages are received from backend.
#[static_ref]
pub fn connection() -> &'static Connection<UpMsg, DownMsg> {
    Connection::new(|down_msg, _cor_id| {
        match down_msg {
            // ------ Auth ------
            DownMsg::LoggedIn(user) => app::set_logged_in_user_and_token(user),
            DownMsg::LoginError(string) => log_in_page::set_login_error(string),

            // ----- Article -------
            DownMsg::Articles(vec) => home_page::set_articles(vec),
            DownMsg::ArticleAdded(_) => {
                home_page::get_articles();
                router().go(Route::Home);
            }
            DownMsg::RegistrationError(string) => registration_page::set_error_msg(string),
            DownMsg::ArticleUpdated => {
                home_page::get_articles();
                router().go(Route::Home);
            }
            DownMsg::ArticleRemoved => {
                home_page::get_articles();
                router().go(Route::Home);
            }
        }
    }).auth_token_getter(app::auth_token)
}
