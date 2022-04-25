use fireauth::FireAuth;

pub async fn init() -> FireAuth {
    let api_key: String = std::env::var("API_KEY").unwrap().to_string();
    fireauth::FireAuth::new(api_key)
}
