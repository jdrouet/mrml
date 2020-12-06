mod common;

#[test]
fn base() {
    common::compare_render(
        include_str!("../resources/mj-breakpoint.mjml"),
        include_str!("../resources/mj-breakpoint.html"),
    );
}

#[test]
fn default_options() {
    common::compare_render(
        include_str!("../resources/mj-breakpoint-default.mjml"),
        include_str!("../resources/mj-breakpoint-default.html"),
    );
}

#[test]
fn with_options() {
    let mut opts = mrml::Options::default();
    opts.breakpoint = mrml::util::size::Size::Pixel(800.0);
    common::compare_render_with_options(
        include_str!("../resources/mj-breakpoint-options.mjml"),
        include_str!("../resources/mj-breakpoint-options.html"),
        opts,
    );
}
