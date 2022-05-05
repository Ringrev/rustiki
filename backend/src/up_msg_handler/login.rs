//! Defines functions used for logging in user.
use crate::User;
use aragog::query::{Comparison, Filter};
use aragog::Record;
use fireauth::FireAuth;
use shared::{DownMsg, LocalUser};

/// The handler for logging user into Firebase. Returns a DownMsg containing a user if successful.
///
/// # Arguments
/// * `auth` - A FireAuth object holding the connection to Firebase as defined in "firebase" module.
/// * `email` - A String holding the user's email.
/// * `password` - A String holding the user's password.
pub async fn handler(auth: FireAuth, email: String, password: String) -> DownMsg {
    let (res, user) = login(auth, email, password).await;
    if res.eq("Ok") {
        DownMsg::LoggedIn(user)
    } else {
        DownMsg::LoginError(res)
    }
}

/// Logs user into Firebase using FireAuth crate.
///
/// # Arguments
/// * `auth` - A FireAuth object holding the connection to Firebase as defined in "firebase" module.
/// * `email` - A String holding the user's email.
/// * `password` - A String holding the user's password.
pub async fn login(auth: FireAuth, email: String, password: String) -> (String, LocalUser) {
    let mut res: String = "".to_string();
    let mut user = LocalUser::new_empty();
    match auth.sign_in_email(&*email, &*password, true).await {
        Ok(response) => {
            res = String::from("Ok");
            println!("{:?}", response);
            user.id = response.local_id.to_string();
            user.email = response.email.to_string();
            user.username = get_username(user.id.clone()).await;
            user.auth_token = response.id_token.to_string();
        }
        Err(_) => res = String::from("Incorrect input, please try again."),
    }
    (res, user)
}

/// Returns a String holding the user's username from ArangoDB database.
///
/// # Arguments
/// * `id` - A String holding the user's id from Firebase.
async fn get_username(id: String) -> String {
    let conn = crate::init_db().await;
    let query = User::query().filter(Filter::new(Comparison::field("id").equals_str(id.as_str())));
    let user_record = User::get(query, &conn).await.unwrap().uniq().unwrap();
    let res = user_record.username.to_string();
    res
}
