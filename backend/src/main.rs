//! Defines main entry point, forwards requests to handlers, and defines structs.
mod firebase;
mod up_msg_handler;
use aragog::{DatabaseConnection, Record};
use moon::*;
use shared::UpMsg;

/// Starts backend app.
#[moon::main]
async fn main() -> std::io::Result<()> {
    start(frontend, up_msg_handler, |_| {}).await?;
    Ok(())
}

/// Returns a Frontend element.
async fn frontend() -> Frontend {
    Frontend::new()
        .lang(Lang::English)
        .title("Rustiki")
        .append_to_head(
        "
        <style>
            html {
                background-color: white;
                color: black;
            }

        </style>
   ",
    )
}

/// Forwards UpMsgRequests received from frontend to "up_msg_handler" module.
///
/// # Arguments
/// * `req` - An UpMsgRequest containing the UpMsg from frontend crate.
async fn up_msg_handler(req: UpMsgRequest<UpMsg>) {
    let (session_id, cor_id) = (req.session_id, req.cor_id);

    let down_msg = match up_msg_handler::handler(req).await {
        Ok(down_msg) | Err(Some(down_msg)) => down_msg,
        _ => return,
    };

    if let Some(session) = sessions::by_session_id().wait_for(session_id).await {
        return session.send_down_msg(&down_msg, cor_id).await;
    }

    println!("Cannot find the session with id `{}`", session_id);
}

/// Returns a connection to ArangoDB.
async fn init_db() -> DatabaseConnection {
    DatabaseConnection::builder().build().await.unwrap()
}

/// This struct must be used to send and receive objects to and from database
/// instead of LocalArticle struct in shared folder. This is
/// because of an issue implementing Record for structs in shared folder.
/// Name of struct has to match name of collection in DB. Case sensitive.
#[derive(Debug, Serialize, Deserialize, Clone, Record)]
#[serde(crate = "serde")]
pub struct Article {
    pub id: u32,
    pub title: String,
    pub content: String,
    pub contributors: Vec<String>,
    pub author: String,
    pub tags: Vec<String>,
    pub created_time: String,
    pub updated_time: String,
}

impl Article {
    /// Creates a new Article object using the Article struct.
    pub fn new(
        id: u32,
        title: String,
        content: String,
        contributors: Vec<String>,
        author: String,
        tags: Vec<String>,
        created_time: String,
        updated_time: String,
    ) -> Self {
        Self {
            id,
            title,
            content,
            contributors,
            author,
            tags,
            created_time,
            updated_time,
        }
    }
}

/// This struct must be used to send and receive objects to and from database
/// instead of LocalUser struct in shared folder. This is
/// because of an issue implementing Record for structs in shared folder.
/// Name of struct has to match name of collection in DB. Case sensitive.
#[derive(Debug, Serialize, Deserialize, Clone, Record)]
#[serde(crate = "serde")]
pub struct User {
    pub id: String,
    pub email: String,
    pub username: String,
}

impl User {
    /// Creates a new User object using the User struct.
    pub fn new(id: String, email: String, username: String) -> Self {
        Self {
            id,
            email,
            username,
        }
    }
}
