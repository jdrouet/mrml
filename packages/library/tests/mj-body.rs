mod common;

#[test]
fn basic() {
    common::compare_render(
        include_str!("../resources/mj-body.mjml"),
        include_str!("../resources/mj-body.html"),
    );
}

#[test]
fn with_options() {
    let mut opts = mrml::Options::default();
    opts.keep_comments = false;
    common::compare_render_with_options(
        include_str!("../resources/mj-body.mjml"),
        include_str!("../resources/mj-body-without-comments.html"),
        opts,
    );
}

#[test]
fn with_background_color() {
    common::compare_render(
        include_str!("../resources/mj-body-background-color.mjml"),
        include_str!("../resources/mj-body-background-color.html"),
    );
}

#[test]
fn with_class() {
    common::compare_render(
        include_str!("../resources/mj-body-class.mjml"),
        include_str!("../resources/mj-body-class.html"),
    );
}

#[test]
fn with_width() {
    common::compare_render(
        include_str!("../resources/mj-body-width.mjml"),
        include_str!("../resources/mj-body-width.html"),
    );
}
