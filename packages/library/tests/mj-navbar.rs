mod common;

#[test]
fn base() {
    common::compare_render(
        include_str!("../resources/mj-navbar.mjml"),
        include_str!("../resources/mj-navbar.html"),
    );
}

#[test]
fn with_align_and_class() {
    common::compare_render(
        include_str!("../resources/mj-navbar-align-class.mjml"),
        include_str!("../resources/mj-navbar-align-class.html"),
    );
}

#[test]
fn with_ico_and_link() {
    common::compare_render(
        include_str!("../resources/mj-navbar-ico.mjml"),
        include_str!("../resources/mj-navbar-ico.html"),
    );
}
