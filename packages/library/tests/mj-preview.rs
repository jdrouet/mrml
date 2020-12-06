mod common;

#[test]
fn base() {
    common::compare_render(
        include_str!("../resources/mj-preview.mjml"),
        include_str!("../resources/mj-preview.html"),
    );
}

#[test]
fn to_preview() {
    common::compare_preview(include_str!("../resources/mj-preview.mjml"), "Hello MJML");
}
