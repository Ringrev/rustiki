use std::any::Any;
use zoon::*;
use zoon::named_color::*;
use crate::router::router;
use crate::router::Route;

pub fn button(label_text: &str, function: fn()) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Font::new().size(16).color(GRAY_0))
        .s(Background::new()
            .color_signal(hovered_signal.map_bool(|| GRAY_5, || GRAY_9)))
        .s(Padding::new().y(10).x(15))
        .on_hovered_change(move |is_hovered| hovered.set(is_hovered))
        .label(label_text)
        .on_click(function)
        .on_key_down_event(move |event| event.if_key(Key::Enter, function))
}

pub fn header_button(label: &str, route: Route, function: Option<fn()>) -> impl Element {
        let (hovered, hovered_signal) = Mutable::new_and_signal(false);
        Link::new()
            .s(Font::new().size(20).color(GRAY_0))
            .s(Align::new().right().bottom())
            .s(Spacing::new(20))
            .s(RoundedCorners::new().right(25).left(25))
            .s(Background::new()
                .color_signal(hovered_signal.map_bool(|| GRAY_5, || GRAY_9)))
            .s(Padding::all(17))
            .on_hovered_change(move |is_hovered| hovered.set(is_hovered))
            .label(label)
            .on_click(function.unwrap_or_else(|| {
                on_click_do_nothing
            }))
            .to(route.clone())
            .on_key_down_event(move |event| event.if_key(Key::Enter, || router().go(route)))
}

pub fn on_click_do_nothing() {
    return;
}

