use zoon::{named_color::*, *};

// ------ ------
//     View
// ------ -----

pub fn footer() -> impl Element {
    Row::new()
        .s(Background::new().color(GRAY_4))
        .s(Spacing::new(10))
        .s(Padding::all(20))
        .s(Borders::new())
        .s(Align::new().bottom())
        .item(information())
}


fn information() -> impl Element {
    Paragraph::new()
        .s(Font::new().size(14))
        .content("© 2022 Rustiki")
}