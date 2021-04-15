use super::MJNavbar;
use crate::print_attrs_children;

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
