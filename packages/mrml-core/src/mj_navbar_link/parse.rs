#[cfg(test)]
mod tests {
    use crate::mj_navbar_link::MjNavbarLink;

    macro_rules! assert_success {
        ($title:ident, $template:expr) => {
            crate::should_sync_parse!($title, MjNavbarLink, $template);
        };
    }

    assert_success!(should_handle_empty_children, "<mj-navbar-link />");

    assert_success!(
        should_handle_comments,
        "<mj-navbar-link><!-- comment --></mj-navbar-link>"
    );

    assert_success!(
        should_work_with_text,
        "<mj-navbar-link>Hello</mj-navbar-link>"
    );

    assert_success!(
        should_work_with_other_element,
        "<mj-navbar-link><span /></mj-navbar-link>"
    );
}
