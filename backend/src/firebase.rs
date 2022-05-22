//! Defines Firebase connection.
use fireauth::FireAuth;

/// Returns a Firebase connection by using crate FireAuth.
/// API_KEY is defined in an environment variable on the server.
pub async fn init() -> FireAuth {
    let api_key: String = std::env::var("API_KEY").unwrap().to_string();
    fireauth::FireAuth::new(api_key)
}
