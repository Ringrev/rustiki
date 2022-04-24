use shared::{DownMsg, LocalUser};
use aragog::Record;
use aragog::query::{Comparison, Filter};
use fireauth::FireAuth;
use crate::User;


pub async fn handler(auth: FireAuth, email: String, password: String) -> DownMsg {
    let (res, user) = login(auth, email, password).await;
    if res.eq("Ok") {
        DownMsg::LoggedIn(user)
    } else {
        DownMsg::LoginError(res)
    }
}

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
        Err(_) => { res = String::from("Incorrect input, please try again.")}
    }
    (res, user)
}

async fn get_username(id: String) -> String {
    let conn = crate::init_db().await;
    let query = User::query().filter(Filter::new(Comparison::field("id").equals_str(id.as_str())));
    let user_record = User::get(query, &conn).await.unwrap().uniq().unwrap();
    let res = user_record.username.to_string();
    res
}