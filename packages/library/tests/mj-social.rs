mod common;

#[test]
fn base() {
    common::compare_render(
        include_str!("../resources/mj-social.mjml"),
        include_str!("../resources/mj-social.html"),
    );
}

#[test]
fn different_origin() {
    let result = include_str!("../resources/mj-social.html").replace(
        "https://www.mailjet.com/images/theme/v1/icons/ico-social/",
        "http://my.origin.rust/",
    );
    let mut opts = mrml::Options::default();
    opts.social_icon_origin = String::from("http://my.origin.rust/");
    common::compare_render_with_options(
        include_str!("../resources/mj-social.mjml"),
        result.as_str(),
        opts,
    );
}

#[test]
fn link() {
    common::compare_render(
        include_str!("../resources/mj-social-link.mjml"),
        include_str!("../resources/mj-social-link.html"),
    );
}

#[test]
fn with_align() {
    common::compare_render(
        include_str!("../resources/mj-social-align.mjml"),
        include_str!("../resources/mj-social-align.html"),
    );
}

#[test]
fn with_border_radius() {
    common::compare_render(
        include_str!("../resources/mj-social-border-radius.mjml"),
        include_str!("../resources/mj-social-border-radius.html"),
    );
}

#[test]
fn with_color() {
    common::compare_render(
        include_str!("../resources/mj-social-color.mjml"),
        include_str!("../resources/mj-social-color.html"),
    );
}

#[test]
fn with_class() {
    common::compare_render(
        include_str!("../resources/mj-social-class.mjml"),
        include_str!("../resources/mj-social-class.html"),
    );
}

#[test]
fn with_container_background_color() {
    common::compare_render(
        include_str!("../resources/mj-social-container-background-color.mjml"),
        include_str!("../resources/mj-social-container-background-color.html"),
    );
}

#[test]
fn with_font_family() {
    common::compare_render(
        include_str!("../resources/mj-social-font-family.mjml"),
        include_str!("../resources/mj-social-font-family.html"),
    );
}

#[test]
fn with_font() {
    common::compare_render(
        include_str!("../resources/mj-social-font.mjml"),
        include_str!("../resources/mj-social-font.html"),
    );
}

#[test]
fn with_icon() {
    common::compare_render(
        include_str!("../resources/mj-social-icon.mjml"),
        include_str!("../resources/mj-social-icon.html"),
    );
}

#[test]
fn with_text() {
    common::compare_render(
        include_str!("../resources/mj-social-text.mjml"),
        include_str!("../resources/mj-social-text.html"),
    );
}

#[test]
fn with_padding() {
    common::compare_render(
        include_str!("../resources/mj-social-padding.mjml"),
        include_str!("../resources/mj-social-padding.html"),
    );
}

#[test]
fn with_mode() {
    common::compare_render(
        include_str!("../resources/mj-social-mode.mjml"),
        include_str!("../resources/mj-social-mode.html"),
    );
}
