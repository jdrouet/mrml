mod common;

#[test]
fn base() {
    common::compare_render(
        include_str!("../resources/mj-accordion.mjml"),
        include_str!("../resources/mj-accordion.html"),
    );
}

#[test]
fn with_others() {
    common::compare_render(
        include_str!("../resources/mj-accordion-other.mjml"),
        include_str!("../resources/mj-accordion-other.html"),
    );
}

#[test]
fn with_icon() {
    common::compare_render(
        include_str!("../resources/mj-accordion-icon.mjml"),
        include_str!("../resources/mj-accordion-icon.html"),
    );
}

#[test]
fn with_font_padding() {
    common::compare_render(
        include_str!("../resources/mj-accordion-font-padding.mjml"),
        include_str!("../resources/mj-accordion-font-padding.html"),
    );
}
