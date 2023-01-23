#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let item = crate::mj_accordion_title::MjAccordionTitle::default();
        assert_eq!("<mj-accordion-title />", item.dense_print());
    }
}
