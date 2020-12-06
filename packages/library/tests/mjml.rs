mod common;

#[test]
fn basic() {
    common::compare_render(
        include_str!("../resources/mjml.mjml"),
        include_str!("../resources/mjml.html"),
    );
}
