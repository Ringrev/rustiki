use crate::{app, router::Route};
use zoon::{named_color::*, *};

// ------ ------
//     View
// ------ ------

pub fn header() -> impl Element {
    Row::new()
        .s(Background::new().color(GRAY_4))
        .s(Font::new().size(80).color(hsluv!(18,100,48,100)))
        .s(Spacing::new(20))
        .item(logo())
        .item(back_button())
        .item(link("Home", Route::Root))

}

fn back_button() -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Background::new().color_signal(hovered_signal.map_bool(|| GREEN_7, || GREEN_8)))
        .s(Padding::new().x(7).y(4))
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

fn logo() -> impl Element {

}

