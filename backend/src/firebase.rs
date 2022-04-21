use fireauth::FireAuth;

pub async fn init() -> FireAuth {
    let api_key: String = String::from("AIzaSyAOyFlU1ws--PLQOKn4ZalUywZuFI8pqBI");
    fireauth::FireAuth::new(api_key)
}
