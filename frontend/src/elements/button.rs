//! Defines different reusable buttons.
use crate::router::router;
use crate::router::Route;
use zoon::named_color::*;
use zoon::*;

/// Returns a Button element
///
/// # Arguments
/// * `id` - A string slice that decides the (unique) HTML id of the element.
/// * `label_text` - A string slice that holds the text to be shown on the button label.
/// * `function` - A function to be called on key down. Function takes no arguments.
pub fn button(id: &str, label_text: &str, function: fn()) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .id(id)
        .s(Font::new().size(16).color(GRAY_0))
        .s(Background::new().color_signal(hovered_signal.map_bool(|| GRAY_5, || GRAY_9)))
        .s(Padding::new().y(10).x(15))
        .on_hovered_change(move |is_hovered| hovered.set(is_hovered))
        .label(label_text)
        .on_click(function)
        .focus(true)
        .on_key_down_event(move |event| event.if_key(Key::Enter, function))
}

/// Returns a Link element that looks like a Button
///
/// # Arguments
/// * `id` - A string slice that decides the (unique) HTML id of the element
/// * `label` - A string slice that holds the text to be shown on the button label
/// * `route` - The Route the user should be directed to
/// * `function` - An Option containing a function to be called on key down. Function takes no arguments.
pub fn header_button(id: &str, label: &str, route: Route, function: Option<fn()>) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Link::new()
        .id(id)
        .s(Font::new().size(18).color(GRAY_0))
        .s(Align::new().right().bottom())
        .s(Spacing::new(20))
        .s(RoundedCorners::new().right(15).left(15))
        .s(Background::new().color_signal(hovered_signal.map_bool(|| GRAY_5, || GRAY_9)))
        .s(Padding::new().x(16).y(9))
        .on_hovered_change(move |is_hovered| hovered.set(is_hovered))
        .label(label)
        .on_click(function.unwrap_or_else(|| on_click_do_nothing))
        // .on_click(function.unwrap_or_else(|| { return; }))
        .to(route.clone())
        .on_key_down_event(move |event| event.if_key(Key::Enter, || router().go(route)))
}

/// Function that does nothing.
/// Helper for making nothing happen when you unwrap
///
pub fn on_click_do_nothing() {
    return;
}
