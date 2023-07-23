#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let item = crate::mj_accordion_element::MjAccordionElement::default();
        assert_eq!("<mj-accordion-element />", item.dense_print());
    }

    #[test]
    fn with_children() {
        let mut item = crate::mj_accordion_element::MjAccordionElement::default();
        item.children.text = Some(crate::mj_accordion_text::MjAccordionText::default());
        assert_eq!(
            "<mj-accordion-element><mj-accordion-text /></mj-accordion-element>",
            item.dense_print()
        );
    }
}
