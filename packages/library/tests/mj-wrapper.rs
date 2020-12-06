mod common;

#[test]
fn base() {
    common::compare_render(
        include_str!("../resources/mj-wrapper.mjml"),
        include_str!("../resources/mj-wrapper.html"),
    );
}

#[test]
fn with_background() {
    common::compare_render(
        include_str!("../resources/mj-wrapper-background.mjml"),
        include_str!("../resources/mj-wrapper-background.html"),
    );
}

#[test]
fn with_border() {
    common::compare_render(
        include_str!("../resources/mj-wrapper-border.mjml"),
        include_str!("../resources/mj-wrapper-border.html"),
    );
}

#[test]
fn with_padding() {
    common::compare_render(
        include_str!("../resources/mj-wrapper-padding.mjml"),
        include_str!("../resources/mj-wrapper-padding.html"),
    );
}

#[test]
fn with_other() {
    common::compare_render(
        include_str!("../resources/mj-wrapper-other.mjml"),
        include_str!("../resources/mj-wrapper-other.html"),
    );
}
