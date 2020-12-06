mod common;

#[test]
fn base() {
    common::compare_render(
        include_str!("../resources/mj-divider.mjml"),
        include_str!("../resources/mj-divider.html"),
    );
}

#[test]
fn with_border() {
    common::compare_render(
        include_str!("../resources/mj-divider-border.mjml"),
        include_str!("../resources/mj-divider-border.html"),
    );
}

#[test]
fn with_container_background_color() {
    common::compare_render(
        include_str!("../resources/mj-divider-container-background-color.mjml"),
        include_str!("../resources/mj-divider-container-background-color.html"),
    );
}

#[test]
fn with_css_class() {
    common::compare_render(
        include_str!("../resources/mj-divider-class.mjml"),
        include_str!("../resources/mj-divider-class.html"),
    );
}

#[test]
fn with_padding() {
    common::compare_render(
        include_str!("../resources/mj-divider-padding.mjml"),
        include_str!("../resources/mj-divider-padding.html"),
    );
}

#[test]
fn with_width() {
    common::compare_render(
        include_str!("../resources/mj-divider-width.mjml"),
        include_str!("../resources/mj-divider-width.html"),
    );
}
