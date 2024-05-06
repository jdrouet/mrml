use crate::prelude::print::{Printable, PrintableAttributes, PrintableChildren, PrintableElement};

impl PrintableChildren for super::MjAccordionElementChildren {
    fn has_children(&self) -> bool {
        self.title.is_some() || self.text.is_some()
    }

    fn print<P: crate::prelude::print::Printer>(&self, printer: &mut P) -> std::fmt::Result {
        if let Some(ref elt) = self.title {
            elt.print(printer)?;
        }
        if let Some(ref elt) = self.text {
            elt.print(printer)?;
        }
        Ok(())
    }
}

impl PrintableElement for super::MjAccordionElement {
    fn tag(&self) -> &str {
        super::NAME
    }

    fn attributes(&self) -> &impl PrintableAttributes {
        &self.attributes
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
        let item = crate::mj_accordion_element::MjAccordionElement::default();
        assert_eq!("<mj-accordion-element />", item.print_dense().unwrap());
    }

    #[test]
    fn with_children() {
        let mut item = crate::mj_accordion_element::MjAccordionElement::default();
        item.children.text = Some(crate::mj_accordion_text::MjAccordionText::default());
        assert_eq!(
            "<mj-accordion-element><mj-accordion-text /></mj-accordion-element>",
            item.print_dense().unwrap()
        );
    }
}
