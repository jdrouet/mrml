mod common;

#[test]
fn base() {
    common::compare_render(
        include_str!("../resources/mj-raw.mjml"),
        include_str!("../resources/mj-raw.html"),
    );
}
