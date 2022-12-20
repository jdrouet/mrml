use super::{MJAccordionText, NAME};
use crate::print_attrs_children;

print_attrs_children!(MJAccordionText, NAME);

#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let item = crate::mj_accordion_text::MJAccordionText::default();
        assert_eq!(
            "<mj-accordion-text></mj-accordion-text>",
            item.dense_print()
        );
    }
}
