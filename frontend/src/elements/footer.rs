//! Defines the footer of the website.
use zoon::{named_color::*, *};

/// Returns a Row element user as a footer.
pub fn footer() -> impl Element {
    Row::new()
        .s(Background::new().color(GRAY_4))
        .s(Spacing::new(10))
        .s(Padding::all(20))
        .s(Borders::new())
        .s(Align::new().bottom())
        .item(footer_text())
}

/// Returns a Paragraph that is used as text of footer element.
fn footer_text() -> impl Element {
    Paragraph::new()
        .s(Font::new().size(14))
        .content("© 2022 Rustiki")
}
