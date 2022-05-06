//! Defines functions used for logging in user.
use crate::User;
use aragog::query::{Comparison, Filter};
use aragog::{DatabaseConnection, Record};
use fireauth::FireAuth;
use shared::{DownMsg, LocalUser};

/// The handler for logging user into Firebase. Returns a DownMsg containing a user if successful.
///
/// # Arguments
/// * `auth` - A FireAuth object holding the connection to Firebase as defined in "firebase" module.
/// * `email` - A String holding the user's email.
/// * `password` - A String holding the user's password.
pub async fn handler(
    auth: FireAuth,
    email: String,
    password: String,
    db_conn: &DatabaseConnection,
) -> DownMsg {
    let (res, user) = login(auth, email, password, db_conn).await;
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
pub async fn login(
    auth: FireAuth,
    email: String,
    password: String,
    db_conn: &DatabaseConnection,
) -> (String, LocalUser) {
    let mut res: String = "".to_string();
    let mut user = LocalUser::new_empty();
    match auth.sign_in_email(&*email, &*password, true).await {
        Ok(response) => {
            res = String::from("Ok");
            println!("{:?}", response);
            user.id = response.local_id.to_string();
            user.email = response.email.to_string();
            user.username = get_username(user.id.clone(), db_conn).await;
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
async fn get_username(id: String, db_conn: &DatabaseConnection) -> String {
    let query = User::query().filter(Filter::new(Comparison::field("id").equals_str(id.as_str())));
    let user_record = User::get(query, db_conn).await.unwrap().uniq().unwrap();
    let res = user_record.username.to_string();
    res
}

// ------ ------
//     Tests
// ------ ------

#[cfg(test)]
mod login_test {
    use crate::firebase;
    use crate::up_msg_handler::login;
    use aragog::DatabaseConnection;
    use fireauth::FireAuth;
    use moon::tokio::task::futures::TaskLocalFuture;
    use std::thread;
    use std::time::Duration;

    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    #[test]
    fn test_get_username() {
        let conn = aw!(async {
            DatabaseConnection::builder()
                .with_schema_path("config/db/schema.yaml")
                .build()
                .await
                .unwrap()
        });
        let username = aw!(login::get_username(
            "ILyhoFJJRHUoivxKyyCZvL04aj63".to_string(),
            &conn
        ));

        assert_eq!(username, "testtest".to_string());
    }

    #[test]
    fn test_login() {
        let conn = aw!(async {
            DatabaseConnection::builder()
                .with_schema_path("config/db/schema.yaml")
                .build()
                .await
                .unwrap()
        });
        let firebase = aw!(firebase::init());
        thread::sleep(Duration::from_millis(1000));
        let (result, user) = aw!(login::login(
            firebase,
            "test@testing.com".to_string(),
            "password".to_string(),
            &conn
        ));
        assert_eq!(user.id, "ILyhoFJJRHUoivxKyyCZvL04aj63".to_string());
    }
}
