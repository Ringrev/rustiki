use zoon::{named_color::*, *};

// ------ ------
//     View
// ------ -----

pub fn footer() -> impl Element {
    Row::new()
        .s(Background::new().color(GRAY_4))

        //.s(Font::new().size(100).color(hsluv!(18,100,48,100)))
        .s(Spacing::new(10))
        .s(Padding::all(20))
        .s(Borders::new())
        .s(Align::new().bottom())
        .item(information())
}


fn information() -> impl Element {

    Paragraph::new()
        .s(Font::new().size(25).weight(FontWeight::Bold))
        .content("Made by the foxy science department")
}