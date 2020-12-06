mod common;

#[test]
fn base() {
    common::compare_render(
        include_str!("../resources/mj-attributes.mjml"),
        include_str!("../resources/mj-attributes.html"),
    );
}
