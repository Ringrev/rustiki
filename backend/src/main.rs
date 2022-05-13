//! Defines main entry point and forwards requests to handlers.
mod firebase;
mod models;
mod up_msg_handler;
use aragog::DatabaseConnection;
use moon::*;
use once_cell::sync::OnceCell;
use shared::UpMsg;

pub static DB: OnceCell<DatabaseConnection> = OnceCell::new();

// ------ ------
//     Start
// ------ ------

/// Starts backend app.
#[moon::main]
async fn main() -> std::io::Result<()> {
    let db = DatabaseConnection::builder().build().await.unwrap();
    DB.set(db).unwrap();
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
