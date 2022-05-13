//! Defines functions used for creating a new user.
use crate::firebase;
use crate::models::User;
use crate::up_msg_handler::login::login;
use aragog::query::{Comparison, Filter};
use aragog::{DatabaseConnection, DatabaseRecord, Record};
use fireauth::FireAuth;
use shared::{DownMsg, LocalUser};

/// The handler for creating a new user. Returns a DownMsg containing a user if successful.
/// If unsuccessful it returns a DownMsg containing an error message as a String.
///
/// # Arguments
/// * `auth` - A FireAuth object holding the connection to Firebase as defined in "firebase" module.
/// * `email` - A String holding the user's email.
/// * `password` - A String holding the user's password.
/// * `username` - A String holding the user's username.
pub async fn handler(
    auth: FireAuth,
    email: String,
    password: String,
    username: String,
    db_conn: &DatabaseConnection,
) -> DownMsg {
    if !check_username_unique(username.clone(), db_conn).await {
        return DownMsg::RegistrationError("Invalid username".to_string());
    }
    let (res, user) = register(auth, email.clone(), password.clone()).await;
    if res.eq("Ok") {
        create_user_in_db(user.id, email.clone(), username, db_conn).await;
        let (result, user) = login(firebase::init().await, email.clone(), password, db_conn).await;
        if result.eq("Ok") {
            DownMsg::LoggedIn(user.clone())
        } else {
            DownMsg::RegistrationError(result)
        }
    } else {
        DownMsg::RegistrationError(res)
    }
}

/// Creates new user in Firebase using FireAuth crate.
/// Returns a tuple consisting of a String message and a LocalUser object if successful,
/// and a message if unsuccessful.
///
/// # Arguments
/// * `auth` - A FireAuth object holding the connection to Firebase as defined in "firebase" module.
/// * `email` - A String holding the user's email.
/// * `password` - A String holding the user's password.
pub async fn register(auth: FireAuth, email: String, password: String) -> (String, LocalUser) {
    let mut user = LocalUser::new_empty();
    let mut res: String = "".to_string();
    match auth.sign_up_email(&*email, &*password, true).await {
        Ok(response) => {
            res = String::from("Ok");
            user.id = response.local_id.to_string().clone();
            user.email = response.email.to_string();
            user.auth_token = response.id_token.to_string();
        }
        Err(_error) => res = "Invalid email".to_string(),
    }
    (res, user)
}

/// Creates the user in ArangoDB database using Aragog crate.
///
/// # Arguments
/// * `id` - A String holding the user's id that was generated in Firebase.
/// * `email` - A String holding the user's email.
/// * `username` - A String holding the user's username.
async fn create_user_in_db(
    id: String,
    email: String,
    username: String,
    db_conn: &DatabaseConnection,
) {
    let db_user = User::new(id, email, username);
    DatabaseRecord::create(db_user, db_conn).await.unwrap();
}

/// Returns <code>true</code> if the username is unique.
/// Returns <code>false</code> if the username is taken.
///
/// # Arguments
/// * `username` - A String holding the username the user wants.
async fn check_username_unique(username: String, db_conn: &DatabaseConnection) -> bool {
    let query = User::query().filter(Filter::new(
        Comparison::field("username").equals_str(username.as_str()),
    ));
    let user_record = User::get(query, db_conn).await.unwrap();
    if user_record.is_empty() {
        true
    } else {
        false
    }
}

// ------ ------
//     Tests
// ------ ------

#[cfg(test)]
mod registration_test {
    use super::*;
    use crate::firebase;
    use aragog::DatabaseConnection;
    use fireauth::FireAuth;

    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    #[test]
    fn test_username_unique() {
        let conn = aw!(async {
            DatabaseConnection::builder()
                .with_schema_path("config/db/schema.yaml")
                .build()
                .await
                .unwrap()
        });
        let res = aw!(check_username_unique("test".to_string(), &conn));
        assert_eq!(res.clone(), false);
        println!("Expected 'false', got {}", res);
    }

    #[test]
    fn test_register() {
        let firebase = aw!(firebase::init());
        let (res, user) = aw!(register(firebase, "test@test.com".to_string(), "password".to_string()));
        assert_eq!(res.clone(), "Invalid email".to_string());
        println!("Expected 'Invalid email', got {}", res);
    }
}
