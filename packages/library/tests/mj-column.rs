mod common;

#[test]
fn base() {
    common::compare_render(
        include_str!("../resources/mj-column.mjml"),
        include_str!("../resources/mj-column.html"),
    );
}

#[test]
fn with_background_color() {
    common::compare_render(
        include_str!("../resources/mj-column-background-color.mjml"),
        include_str!("../resources/mj-column-background-color.html"),
    );
}

#[test]
fn with_border() {
    common::compare_render(
        include_str!("../resources/mj-column-border.mjml"),
        include_str!("../resources/mj-column-border.html"),
    );
}

#[test]
fn with_border_radius() {
    common::compare_render(
        include_str!("../resources/mj-column-border-radius.mjml"),
        include_str!("../resources/mj-column-border-radius.html"),
    );
}

#[test]
fn with_class() {
    common::compare_render(
        include_str!("../resources/mj-column-class.mjml"),
        include_str!("../resources/mj-column-class.html"),
    );
}

#[test]
fn with_padding() {
    common::compare_render(
        include_str!("../resources/mj-column-padding.mjml"),
        include_str!("../resources/mj-column-padding.html"),
    );
}

#[test]
fn with_vertical_align() {
    common::compare_render(
        include_str!("../resources/mj-column-vertical-align.mjml"),
        include_str!("../resources/mj-column-vertical-align.html"),
    );
}

#[test]
fn with_width() {
    common::compare_render(
        include_str!("../resources/mj-column-width.mjml"),
        include_str!("../resources/mj-column-width.html"),
    );
}
