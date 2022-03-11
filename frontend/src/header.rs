
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