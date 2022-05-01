use crate::up_msg_handler::login::login;
use crate::{firebase, init_db, User};
use aragog::query::{Comparison, Filter};
use aragog::{DatabaseRecord, Record};
use fireauth::FireAuth;
use shared::{DownMsg, LocalUser};

pub async fn handler(auth: FireAuth, email: String, password: String, username: String) -> DownMsg {
    if !check_username_unique(username.clone()).await {
        return DownMsg::RegistrationError("Invalid username".to_string());
    }
    let (res, user) = register(auth, email.clone(), password.clone()).await;
    if res.eq("Ok") {
        create_user_in_db(user.id, email.clone(), username).await;
        let (result, user) = login(firebase::init().await, email.clone(), password).await;
        if result.eq("Ok") {
            DownMsg::LoggedIn(user.clone())
        } else {
            DownMsg::RegistrationError(result)
        }
    } else {
        DownMsg::RegistrationError(res)
    }
}

pub async fn register(auth: FireAuth, email: String, password: String) -> (String, LocalUser) {
    let mut user = LocalUser::new_empty();
    let mut res: String = "".to_string();
    match auth.sign_up_email(&*email, &*password, true).await {
        Ok(response) => {
            res = String::from("Ok");
            println!("{:?}", response);

            user.id = response.local_id.to_string().clone();
            user.email = response.email.to_string();
            user.auth_token = response.id_token.to_string();
        }
        Err(_error) => res = "Invalid email".to_string(),
    }
    (res, user)
}

async fn create_user_in_db(id: String, email: String, username: String) {
    let conn = crate::init_db().await;
    let db_user = User::new(id, email, username);
    DatabaseRecord::create(db_user, &conn).await.unwrap();
}

async fn check_username_unique(username: String) -> bool {
    let conn = init_db().await;
    let query = User::query().filter(Filter::new(
        Comparison::field("username").equals_str(username.as_str()),
    ));
    let user_record = User::get(query, &conn).await.unwrap();
    if user_record.is_empty() {
        true
    } else {
        false
    }
}
