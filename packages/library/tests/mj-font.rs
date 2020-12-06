mod common;

#[test]
fn base() {
    common::compare_render(
        include_str!("../resources/mj-font.mjml"),
        include_str!("../resources/mj-font.html"),
    );
}
