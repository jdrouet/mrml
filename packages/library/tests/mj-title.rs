mod common;

#[test]
fn base() {
    common::compare_render(
        include_str!("../resources/mj-title.mjml"),
        include_str!("../resources/mj-title.html"),
    );
}

#[test]
fn to_title() {
    common::compare_title(include_str!("../resources/mj-title.mjml"), "Hello MJML");
}
