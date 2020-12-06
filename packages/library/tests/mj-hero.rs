mod common;

#[test]
fn base() {
    common::compare_render(
        include_str!("../resources/mj-hero.mjml"),
        include_str!("../resources/mj-hero.html"),
    );
}

#[test]
fn with_background_color() {
    common::compare_render(
        include_str!("../resources/mj-hero-background-color.mjml"),
        include_str!("../resources/mj-hero-background-color.html"),
    );
}

#[test]
fn with_background_height() {
    common::compare_render(
        include_str!("../resources/mj-hero-background-height.mjml"),
        include_str!("../resources/mj-hero-background-height.html"),
    );
}

#[test]
fn with_background_position() {
    common::compare_render(
        include_str!("../resources/mj-hero-background-position.mjml"),
        include_str!("../resources/mj-hero-background-position.html"),
    );
}

#[test]
fn with_background_url() {
    common::compare_render(
        include_str!("../resources/mj-hero-background-url.mjml"),
        include_str!("../resources/mj-hero-background-url.html"),
    );
}

#[test]
fn with_background_width() {
    common::compare_render(
        include_str!("../resources/mj-hero-background-width.mjml"),
        include_str!("../resources/mj-hero-background-width.html"),
    );
}

#[test]
fn with_class() {
    common::compare_render(
        include_str!("../resources/mj-hero-class.mjml"),
        include_str!("../resources/mj-hero-class.html"),
    );
}

#[test]
fn with_height() {
    common::compare_render(
        include_str!("../resources/mj-hero-height.mjml"),
        include_str!("../resources/mj-hero-height.html"),
    );
}

#[test]
fn with_mode() {
    common::compare_render(
        include_str!("../resources/mj-hero-mode.mjml"),
        include_str!("../resources/mj-hero-mode.html"),
    );
}

#[test]
fn with_vertical_align() {
    common::compare_render(
        include_str!("../resources/mj-hero-vertical-align.mjml"),
        include_str!("../resources/mj-hero-vertical-align.html"),
    );
}

#[test]
fn with_width() {
    common::compare_render(
        include_str!("../resources/mj-hero-width.mjml"),
        include_str!("../resources/mj-hero-width.html"),
    );
}
