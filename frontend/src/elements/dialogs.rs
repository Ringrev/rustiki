use zoon::*;

pub fn message_dialog(text: &str) {
    if let Err(_) = window().alert_with_message(text.clone()) {
        message_dialog(text.clone())
    };
}


pub fn confirm_dialog(text: &str) -> bool {
    let res = window().confirm_with_message(text);
    res.unwrap()
}