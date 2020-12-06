mod common;

#[test]
fn base() {
    common::compare_render(
        include_str!("../resources/mj-text.mjml"),
        include_str!("../resources/mj-text.html"),
    );
}

#[test]
fn doc_example() {
    common::compare_render(
        include_str!("../resources/mj-text-example.mjml"),
        include_str!("../resources/mj-text-example.html"),
    );
}

#[test]
fn with_color() {
    common::compare_render(
        include_str!("../resources/mj-text-color.mjml"),
        include_str!("../resources/mj-text-color.html"),
    );
}

#[test]
fn with_font_family() {
    common::compare_render(
        include_str!("../resources/mj-text-font-family.mjml"),
        include_str!("../resources/mj-text-font-family.html"),
    );
}

#[test]
fn with_font_size() {
    common::compare_render(
        include_str!("../resources/mj-text-font-size.mjml"),
        include_str!("../resources/mj-text-font-size.html"),
    );
}

#[test]
fn with_font_style() {
    common::compare_render(
        include_str!("../resources/mj-text-font-style.mjml"),
        include_str!("../resources/mj-text-font-style.html"),
    );
}

#[test]
fn with_line_height() {
    common::compare_render(
        include_str!("../resources/mj-text-line-height.mjml"),
        include_str!("../resources/mj-text-line-height.html"),
    );
}

#[test]
fn with_letter_spacing() {
    common::compare_render(
        include_str!("../resources/mj-text-letter-spacing.mjml"),
        include_str!("../resources/mj-text-letter-spacing.html"),
    );
}

#[test]
fn with_height() {
    common::compare_render(
        include_str!("../resources/mj-text-height.mjml"),
        include_str!("../resources/mj-text-height.html"),
    );
}

#[test]
fn with_decoration() {
    common::compare_render(
        include_str!("../resources/mj-text-decoration.mjml"),
        include_str!("../resources/mj-text-decoration.html"),
    );
}

#[test]
fn with_transform() {
    common::compare_render(
        include_str!("../resources/mj-text-transform.mjml"),
        include_str!("../resources/mj-text-transform.html"),
    );
}

#[test]
fn with_align() {
    common::compare_render(
        include_str!("../resources/mj-text-align.mjml"),
        include_str!("../resources/mj-text-align.html"),
    );
}

#[test]
fn with_container_background_color() {
    common::compare_render(
        include_str!("../resources/mj-text-container-background-color.mjml"),
        include_str!("../resources/mj-text-container-background-color.html"),
    );
}

#[test]
fn with_padding() {
    common::compare_render(
        include_str!("../resources/mj-text-padding.mjml"),
        include_str!("../resources/mj-text-padding.html"),
    );
}

#[test]
fn with_css_class() {
    common::compare_render(
        include_str!("../resources/mj-text-class.mjml"),
        include_str!("../resources/mj-text-class.html"),
    );
}
