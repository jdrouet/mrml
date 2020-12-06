mod common;

#[test]
fn base() {
    common::compare_render(
        include_str!("../resources/mj-spacer.mjml"),
        include_str!("../resources/mj-spacer.html"),
    );
}
