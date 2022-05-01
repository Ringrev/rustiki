use zoon::*;

pub fn title_view(title: &str) -> impl Element {
    Paragraph::new()
        .content("Create a user account")
        .s(Font::new().size(20))
        .s(Padding::bottom(Default::default(), 20))
}