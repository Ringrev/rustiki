
use crate::{app, router::Route};
use zoon::{named_color::*, *};

// ------ ------
//     View
// ------ ------

pub fn header() -> impl Element {
    Row::new()
        .s(Background::new().color(GRAY_4))

        //.s(Font::new().size(100).color(hsluv!(18,100,48,100)))
        .s(Spacing::new(10))
        .s(Padding::all(20))
        .s(Borders::new())
        .item(logo())
        .item(back_button())
       // .item(link("Home", Route::Root))
        .item(search_box())
        .item(buttons_row())
}

fn logo() -> impl Element {
    Image::new()
        .url("data:image/png;base64,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")
        .description("A cat")
        .s(Width::new(200))
       // .s(Padding::top(Default::default(), 50))
}


fn back_button() -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Background::new().color_signal(hovered_signal.map_bool(|| GRAY_8, || GRAY_4)))
        .s(Padding::new().x(2).y(2))
        .s(Align::new().bottom().left())
        .on_hovered_change(move |is_hovered| hovered.set(is_hovered))
        .label("< Back")
        .on_press(routing::back)

}

fn link(label: &str, route: Route) -> impl Element {
    Link::new()
        .s(Font::new().color(BLUE_4).line(FontLine::new().underline()))
        .label(label)
        .to(route)
}

fn search_box() -> impl Element {
    Row::new()
        .s(Align::new().center_x().center_y())
        .item(search_bar())
        .item(search_button())

}

fn search_bar() -> impl Element {
    TextInput::new()
        .s(Align::new().center_x())
        .s(Padding::all(10))
        .s(RoundedCorners::new().left(5))
        .s(Width::fill().min(350).max(400))
        .s(Font::new().size(20))
        .s(RoundedCorners::new().right(25).left(25))
        .focus(true)
       // .on_change(super::set_new_message_text)
        .label_hidden("New message text")
        .placeholder(Placeholder::new("Search here you clown"))

}

fn search_button() -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Padding::all(25))
        .s(RoundedCorners::new().right(5))
        .s(Background::new().color_signal(hovered_signal.map_bool(|| GREEN_7, || GREEN_8)))
        .s(Font::new().color(GRAY_0).size(17))
        .s(Align::new().left())
        .s(RoundedCorners::new().right(25).left(25))
        .on_hovered_change(move |is_hovered| hovered.set(is_hovered))
        //.on_press(super::send_message)
        .label("Search")
}


fn buttons_row() -> impl Element {
    Row::new()
        .s(Align::new().bottom().right())
        .s(Spacing::new(6))
        .item(log_inn())
        .item(log_inn())
        .item(log_inn())
}


fn log_inn() -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Font::new().size(20).color(GRAY_0))
        .s(Align::new().right().bottom())
        .s(Spacing::new(20))
        .s(RoundedCorners::new().right(25).left(25))
        .s(Background::new()
            .color_signal(hovered_signal.map_bool(|| GRAY_5, || GRAY_9)))
        .s(Padding::all(17))
        .on_hovered_change(move |is_hovered| hovered.set(is_hovered))
        .label("Log in")
       // .on_press(log_in)
}