//! Defines different reusable layout elements.
use crate::elements::tags;
use zoon::named_color::GRAY_0;
use zoon::*;

/// Returns a Paragraph representing the title on a page.
///
/// # Arguments
/// * `title` - A Strings slice that holds the title of the page.
pub fn page_title_view(title: &str) -> impl Element {
    Paragraph::new()
        .content(title)
        .s(Font::new().size(20))
        .s(Padding::bottom(Default::default(), 20))
}

pub fn edit_layout(
    title: &str,
    title_panel: impl Element,
    content_panel: impl Element,
    tag_panel_id: &str,
) -> impl Element {
    Column::new()
        .s(Align::center())
        .s(Padding::new().x(100).y(20))
        .item(page_title_view(title))
        .item(title_panel)
        .item(content_panel)
        .item(tags::tag_panel(tag_panel_id))
        .item(tags::tags_view())
}

pub fn common_layout(first_item: impl Element, second_item: impl Element) -> impl Element {
    Column::new()
        .s(Align::center())
        .s(Width::new(800))
        .s(Background::new().color(GRAY_0))
        .item(first_item)
        .item(second_item)
}
