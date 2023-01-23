#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let item = crate::mj_navbar::MjNavbar::default();
        assert_eq!("<mj-navbar />", item.dense_print());
    }
}
