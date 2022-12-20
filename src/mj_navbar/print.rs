use super::{MJNavbar, MJNavbarChild};
use crate::print_attrs_children;

impl Print for MJNavbarChild {
    fn print(&self, pretty: bool, level: usize, indent_size: usize) -> String {
        match self {
            Self::Comment(elt) => elt.print(pretty, level, indent_size),
            Self::MJNavbarLink(elt) => elt.print(pretty, level, indent_size),
        }
    }
}

print_attrs_children!(MJNavbar, super::NAME);

#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let item = crate::mj_navbar::MJNavbar::default();
        assert_eq!("<mj-navbar></mj-navbar>", item.dense_print());
    }
}
