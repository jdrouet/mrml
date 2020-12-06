mod common;

#[test]
fn base() {
    common::compare_render(
        include_str!("../resources/mj-button.mjml"),
        include_str!("../resources/mj-button.html"),
    );
}

#[test]
fn example() {
    common::compare_render(
        include_str!("../resources/mj-button-example.mjml"),
        include_str!("../resources/mj-button-example.html"),
    );
}

#[test]
fn with_align() {
    common::compare_render(
        include_str!("../resources/mj-button-align.mjml"),
        include_str!("../resources/mj-button-align.html"),
    );
}

#[test]
fn with_background() {
    common::compare_render(
        include_str!("../resources/mj-button-background.mjml"),
        include_str!("../resources/mj-button-background.html"),
    );
}

#[test]
fn with_border() {
    common::compare_render(
        include_str!("../resources/mj-button-border.mjml"),
        include_str!("../resources/mj-button-border.html"),
    );
}

#[test]
fn with_border_radius() {
    common::compare_render(
        include_str!("../resources/mj-button-border-radius.mjml"),
        include_str!("../resources/mj-button-border-radius.html"),
    );
}

#[test]
fn with_color() {
    common::compare_render(
        include_str!("../resources/mj-button-color.mjml"),
        include_str!("../resources/mj-button-color.html"),
    );
}

#[test]
fn with_container_background_color() {
    common::compare_render(
        include_str!("../resources/mj-button-container-background-color.mjml"),
        include_str!("../resources/mj-button-container-background-color.html"),
    );
}

#[test]
fn with_class() {
    common::compare_render(
        include_str!("../resources/mj-button-class.mjml"),
        include_str!("../resources/mj-button-class.html"),
    );
}

#[test]
fn with_font_family() {
    common::compare_render(
        include_str!("../resources/mj-button-font-family.mjml"),
        include_str!("../resources/mj-button-font-family.html"),
    );
}

#[test]
fn with_font_size() {
    common::compare_render(
        include_str!("../resources/mj-button-font-size.mjml"),
        include_str!("../resources/mj-button-font-size.html"),
    );
}

#[test]
fn with_font_style() {
    common::compare_render(
        include_str!("../resources/mj-button-font-style.mjml"),
        include_str!("../resources/mj-button-font-style.html"),
    );
}

#[test]
fn with_font_weight() {
    common::compare_render(
        include_str!("../resources/mj-button-font-weight.mjml"),
        include_str!("../resources/mj-button-font-weight.html"),
    );
}

#[test]
fn with_height() {
    common::compare_render(
        include_str!("../resources/mj-button-height.mjml"),
        include_str!("../resources/mj-button-height.html"),
    );
}

#[test]
fn with_href() {
    common::compare_render(
        include_str!("../resources/mj-button-href.mjml"),
        include_str!("../resources/mj-button-href.html"),
    );
}

#[test]
fn with_inner_padding() {
    common::compare_render(
        include_str!("../resources/mj-button-inner-padding.mjml"),
        include_str!("../resources/mj-button-inner-padding.html"),
    );
}

#[test]
fn with_line_height() {
    common::compare_render(
        include_str!("../resources/mj-button-line-height.mjml"),
        include_str!("../resources/mj-button-line-height.html"),
    );
}

#[test]
fn with_padding() {
    common::compare_render(
        include_str!("../resources/mj-button-padding.mjml"),
        include_str!("../resources/mj-button-padding.html"),
    );
}

#[test]
fn with_text_decoration() {
    common::compare_render(
        include_str!("../resources/mj-button-text-decoration.mjml"),
        include_str!("../resources/mj-button-text-decoration.html"),
    );
}

#[test]
fn with_text_transform() {
    common::compare_render(
        include_str!("../resources/mj-button-text-transform.mjml"),
        include_str!("../resources/mj-button-text-transform.html"),
    );
}

#[test]
fn with_vertical_align() {
    common::compare_render(
        include_str!("../resources/mj-button-vertical-align.mjml"),
        include_str!("../resources/mj-button-vertical-align.html"),
    );
}

#[test]
fn with_width() {
    common::compare_render(
        include_str!("../resources/mj-button-width.mjml"),
        include_str!("../resources/mj-button-width.html"),
    );
}
