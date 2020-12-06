mod common;

#[test]
fn with_body_width() {
    common::compare_render(
        include_str!("../resources/mj-section-body-width.mjml"),
        include_str!("../resources/mj-section-body-width.html"),
    );
}

#[test]
fn base() {
    common::compare_render(
        include_str!("../resources/mj-section.mjml"),
        include_str!("../resources/mj-section.html"),
    );
}

#[test]
fn with_background_color() {
    common::compare_render(
        include_str!("../resources/mj-section-background-color.mjml"),
        include_str!("../resources/mj-section-background-color.html"),
    );
}

#[test]
fn with_background_url() {
    common::compare_render(
        include_str!("../resources/mj-section-background-url.mjml"),
        include_str!("../resources/mj-section-background-url.html"),
    );
}

#[test]
fn with_background_url_full() {
    common::compare_render(
        include_str!("../resources/mj-section-background-url-full.mjml"),
        include_str!("../resources/mj-section-background-url-full.html"),
    );
}

#[test]
fn with_border() {
    common::compare_render(
        include_str!("../resources/mj-section-border.mjml"),
        include_str!("../resources/mj-section-border.html"),
    );
}

#[test]
fn with_border_radius() {
    common::compare_render(
        include_str!("../resources/mj-section-border-radius.mjml"),
        include_str!("../resources/mj-section-border-radius.html"),
    );
}

#[test]
fn with_css_class() {
    common::compare_render(
        include_str!("../resources/mj-section-class.mjml"),
        include_str!("../resources/mj-section-class.html"),
    );
}

#[test]
fn with_direction() {
    common::compare_render(
        include_str!("../resources/mj-section-direction.mjml"),
        include_str!("../resources/mj-section-direction.html"),
    );
}

#[test]
fn with_full_width() {
    common::compare_render(
        include_str!("../resources/mj-section-full-width.mjml"),
        include_str!("../resources/mj-section-full-width.html"),
    );
}

#[test]
fn with_padding() {
    common::compare_render(
        include_str!("../resources/mj-section-padding.mjml"),
        include_str!("../resources/mj-section-padding.html"),
    );
}

#[test]
fn with_text_align() {
    common::compare_render(
        include_str!("../resources/mj-section-text-align.mjml"),
        include_str!("../resources/mj-section-text-align.html"),
    );
}
