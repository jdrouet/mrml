use crate::prelude::print::{PrintableChildren, PrintableElement};

impl PrintableElement for super::MjAccordionTitle {
    fn tag(&self) -> &str {
        super::NAME
    }

    fn children(&self) -> &impl PrintableChildren {
        &self.children
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::print::Printable;

    #[test]
    fn empty() {
        let item = crate::mj_accordion_title::MjAccordionTitle::default();
        assert_eq!("<mj-accordion-title />", item.print_dense().unwrap());
    }
}
