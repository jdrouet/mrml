use super::{MJNavbarLink, MJNavbarLinkChild};
use crate::print_attrs;

impl Print for MJNavbarLinkChild {
    fn print(&self, pretty: bool, level: usize, indent_size: usize) -> String {
        match self {
            Self::Comment(elt) => elt.print(pretty, level, indent_size),
            Self::Text(elt) => elt.print(pretty, level, indent_size),
        }
    }
}

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
