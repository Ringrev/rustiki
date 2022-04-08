use zoon::{*, println, eprintln};
use shared::{UpMsg, DownMsg};
use crate::*;

#[static_ref]
pub fn connection() -> &'static Connection<UpMsg, DownMsg> {
    Connection::new(|down_msg, _cor_id| {
        println!("DownMsg received: {:?}", down_msg);

        match down_msg {
            // ------ Auth ------
            DownMsg::LoggedIn(user) => header::set_logged_in_user_and_token(user),
            DownMsg::LoginError(string) => log_in_page::set_login_error(string),
            DownMsg::RegistrationError(string) => registration_page::set_error_msg(string),
        }
    })
}