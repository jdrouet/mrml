mod common;

#[test]
fn base() {
    common::compare_render(
        include_str!("../resources/mj-image.mjml"),
        include_str!("../resources/mj-image.html"),
    );
}

#[test]
fn with_align() {
    common::compare_render(
        include_str!("../resources/mj-image-align.mjml"),
        include_str!("../resources/mj-image-align.html"),
    );
}

#[test]
fn with_border() {
    common::compare_render(
        include_str!("../resources/mj-image-border.mjml"),
        include_str!("../resources/mj-image-border.html"),
    );
}

#[test]
fn with_border_radius() {
    common::compare_render(
        include_str!("../resources/mj-image-border-radius.mjml"),
        include_str!("../resources/mj-image-border-radius.html"),
    );
}

#[test]
fn with_container_background_color() {
    common::compare_render(
        include_str!("../resources/mj-image-container-background-color.mjml"),
        include_str!("../resources/mj-image-container-background-color.html"),
    );
}

#[test]
fn with_css_class() {
    common::compare_render(
        include_str!("../resources/mj-image-class.mjml"),
        include_str!("../resources/mj-image-class.html"),
    );
}

#[test]
fn with_fluid_on_mobile() {
    common::compare_render(
        include_str!("../resources/mj-image-fluid-on-mobile.mjml"),
        include_str!("../resources/mj-image-fluid-on-mobile.html"),
    );
}

#[test]
fn with_height() {
    common::compare_render(
        include_str!("../resources/mj-image-height.mjml"),
        include_str!("../resources/mj-image-height.html"),
    );
}

#[test]
fn with_href() {
    common::compare_render(
        include_str!("../resources/mj-image-href.mjml"),
        include_str!("../resources/mj-image-href.html"),
    );
}

#[test]
fn with_padding() {
    common::compare_render(
        include_str!("../resources/mj-image-padding.mjml"),
        include_str!("../resources/mj-image-padding.html"),
    );
}

#[test]
fn with_rel() {
    common::compare_render(
        include_str!("../resources/mj-image-rel.mjml"),
        include_str!("../resources/mj-image-rel.html"),
    );
}

#[test]
fn with_title() {
    common::compare_render(
        include_str!("../resources/mj-image-title.mjml"),
        include_str!("../resources/mj-image-title.html"),
    );
}
