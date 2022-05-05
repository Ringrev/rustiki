//! Defines different reusable layout elements.
use zoon::*;

/// Returns a Paragraph representing the title on a page.
///
/// # Arguments
/// * `title` - A Strings slice that holds the title of the page.
pub fn title_view(title: &str) -> impl Element {
    Paragraph::new()
        .content("Create a user account")
        .s(Font::new().size(20))
        .s(Padding::bottom(Default::default(), 20))
}