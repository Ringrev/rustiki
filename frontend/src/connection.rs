use zoon::{*, println, eprintln};
use shared::{UpMsg, DownMsg};
use crate::app::set_user;

#[static_ref]
pub fn connection() -> &'static Connection<UpMsg, DownMsg> {
    Connection::new(|down_msg, _cor_id| {
        println!("DownMsg received: {:?}", down_msg);

        match down_msg {
            // ------ Auth ------
            DownMsg::LoggedIn(user) => set_user(user),
        }
    })
}