mod common;

#[test]
fn base() {
    common::compare_render(
        include_str!("../resources/mj-carousel.mjml"),
        include_str!("../resources/mj-carousel.html"),
    );
}

#[test]
fn with_align_border_radius_css_class() {
    common::compare_render(
        include_str!("../resources/mj-carousel-align-border-radius-class.mjml"),
        include_str!("../resources/mj-carousel-align-border-radius-class.html"),
    );
}

#[test]
fn with_icon() {
    common::compare_render(
        include_str!("../resources/mj-carousel-icon.mjml"),
        include_str!("../resources/mj-carousel-icon.html"),
    );
}

#[test]
fn with_tb() {
    common::compare_render(
        include_str!("../resources/mj-carousel-tb.mjml"),
        include_str!("../resources/mj-carousel-tb.html"),
    );
}

#[test]
fn with_thumbnail() {
    common::compare_render(
        include_str!("../resources/mj-carousel-thumbnails.mjml"),
        include_str!("../resources/mj-carousel-thumbnails.html"),
    );
}
