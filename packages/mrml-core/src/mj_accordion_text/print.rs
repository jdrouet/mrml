#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let item = crate::mj_accordion_text::MjAccordionText::default();
        assert_eq!("<mj-accordion-text />", item.dense_print());
    }
}
