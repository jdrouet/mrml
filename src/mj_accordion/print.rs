#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let item = crate::mj_accordion::MJAccordion::default();
        assert_eq!("<mj-accordion />", item.dense_print());
    }
}
