#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let item = crate::mj_accordion_title::MJAccordionTitle::default();
        assert_eq!("<mj-accordion-title />", item.dense_print());
    }
}
