mod common;

#[test]
fn base() {
    common::compare_render(
        include_str!("../resources/mj-style.mjml"),
        include_str!("../resources/mj-style.html"),
    );
}
