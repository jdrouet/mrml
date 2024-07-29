#[cfg(test)]
mod tests {
    use crate::prelude::print::Printable;

    #[test]
    fn empty() {
        let item = crate::mj_navbar::MjNavbar::default();
        assert_eq!("<mj-navbar />", item.print_dense().unwrap());
    }
}
