use std::thread;
use moon::*;
use fireauth::FireAuth;

pub async fn init() -> FireAuth {
    let api_key: String = String::from("AIzaSyAOyFlU1ws--PLQOKn4ZalUywZuFI8pqBI");
    fireauth::FireAuth::new(api_key)
}

// pub async fn sign_up(email: &str, password: &str) {
//     let auth = init().await;
//     match auth.sign_up_email(email, password, true).await {
//         Ok(response) =>  { println!("{:?}", response) ; }
//         Err(error) => { println!("{:?}", error); }
//     }
// }
