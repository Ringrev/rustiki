use zoon::*;

pub fn page() -> impl Element {
    Column::new()
        .item(Paragraph::new().content("Register as a user"))
}