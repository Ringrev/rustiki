
use crate::{app, router::Route};
use zoon::{named_color::*, *};

// ------ ------
//     View
// ------ ------

pub fn header() -> impl Element {
    Row::new()
        .s(Background::new().color(GRAY_4))

        .s(Font::new().size(100).color(hsluv!(18,100,48,100)))
        .s(Spacing::new(10))
        .s(Padding::new())
        .s(Borders::new())
        .item(logo())
        .item(back_button())
       // .item(link("Home", Route::Root))
        .item(search_bar())
        .item(send_button())
        .item(log_inn())
}

fn logo() -> impl Element {
    Image::new()
        .url("https://www.catastrophicreations.com/wp-content/uploads/2021/02/IMG_7465-2glance.jpg%22")
        .description("A cat")
        .s(Width::new(200))
       // .s(Padding::top(Default::default(), 50))
}


fn back_button() -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Background::new().color_signal(hovered_signal.map_bool(|| GRAY_8, || GRAY_4)))
        .s(Padding::new().x(2).y(2))
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

fn search_bar() -> impl Element {
    TextInput::new()
        .s(Align::new().center_x())
        .s(Padding::all(10))
        .s(RoundedCorners::new().left(5))
        .s(Width::fill())
        .s(Font::new().size(17))
        .focus(true)
       // .on_change(super::set_new_message_text)
        .label_hidden("New message text")
        .placeholder(Placeholder::new("Search here you clown"))

}

fn send_button() -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Padding::all(25))
        .s(RoundedCorners::new().right(5))
        .s(Background::new().color_signal(hovered_signal.map_bool(|| GREEN_7, || GREEN_8)))
        .s(Font::new().color(GRAY_0).size(17))
        .on_hovered_change(move |is_hovered| hovered.set(is_hovered))
        //.on_press(super::send_message)
        .label("Send")
}

fn log_inn() -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Font::new().size(25))
        .s(Align::new().right().bottom())
        .s(Spacing::new(20))
        .s(RoundedCorners::new().right(25).left(25))
        .s(Background::new()
            .color_signal(hovered_signal.map_bool(|| GRAY_1, || GRAY_9)))
        .s(Padding::all(17))
        .on_hovered_change(move |is_hovered| hovered.set(is_hovered))
        .label("Log in")
       // .on_press(log_in)
}