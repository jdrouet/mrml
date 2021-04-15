use super::MJNavbarLink;
use crate::print_attrs;

print_attrs!(MJNavbarLink, super::NAME);

#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let mut item = crate::mj_navbar_link::MJNavbarLink::default();
        item.attributes
            .insert("src".to_string(), "http://localhost".into());
        assert_eq!(
            "<mj-navbar-link src=\"http://localhost\" />",
            item.dense_print()
        );
    }
}
