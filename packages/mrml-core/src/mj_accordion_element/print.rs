use super::MJAccordionElement;
use crate::prelude::print::{self, Print};
use std::fmt;

impl Print for MJAccordionElement {
    fn print(&self, pretty: bool, level: usize, indent_size: usize) -> String {
        print::open(
            super::NAME,
            Some(&self.attributes),
            false,
            pretty,
            level,
            indent_size,
        ) + &self
            .title
            .as_ref()
            .map(|title| title.print(pretty, level + 1, indent_size))
            .unwrap_or_default()
            + &self
                .text
                .as_ref()
                .map(|text| text.print(pretty, level + 1, indent_size))
                .unwrap_or_default()
            + &print::close(super::NAME, pretty, level, indent_size)
    }
}

impl fmt::Display for MJAccordionElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.dense_print().as_str())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let item = crate::mj_accordion_element::MJAccordionElement::default();
        assert_eq!(
            "<mj-accordion-element></mj-accordion-element>",
            item.dense_print()
        );
    }
}
