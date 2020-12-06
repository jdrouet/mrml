mod common;

#[test]
fn base() {
    common::compare_render(
        include_str!("../resources/mj-group.mjml"),
        include_str!("../resources/mj-group.html"),
    );
}

#[test]
fn with_background_color() {
    common::compare_render(
        include_str!("../resources/mj-group-background-color.mjml"),
        include_str!("../resources/mj-group-background-color.html"),
    );
}

#[test]
fn with_css_class() {
    common::compare_render(
        include_str!("../resources/mj-group-class.mjml"),
        include_str!("../resources/mj-group-class.html"),
    );
}

#[test]
fn with_direction() {
    common::compare_render(
        include_str!("../resources/mj-group-direction.mjml"),
        include_str!("../resources/mj-group-direction.html"),
    );
}

#[test]
fn with_vertical_align() {
    common::compare_render(
        include_str!("../resources/mj-group-vertical-align.mjml"),
        include_str!("../resources/mj-group-vertical-align.html"),
    );
}

#[test]
fn with_width() {
    common::compare_render(
        include_str!("../resources/mj-group-width.mjml"),
        include_str!("../resources/mj-group-width.html"),
    );
}
