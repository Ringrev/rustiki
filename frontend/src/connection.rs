use zoon::*;
use shared::{UpMsg, DownMsg};
use crate::*;
use router::router;
use crate::pages::{home_page, log_in_page, registration_page};
use crate::router::Route;

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
                router().go(Route::Home)},
            DownMsg::RegistrationError(string) => registration_page::set_error_msg(string),
            DownMsg::ArticleUpdated => {
                router().go(Route::Home)
            },
            DownMsg::ArticleRemoved => {
                router().go(Route::Home)
            }
        }
    })
}