use crate::prelude::print::{Printable, PrintableAttributes, PrintableChildren};

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

impl Printable for super::MjAccordionElement {
    fn print<P: crate::prelude::print::Printer>(&self, printer: &mut P) -> std::fmt::Result {
        printer.open_tag(super::NAME)?;
        self.attributes.print(printer)?;
        if self.children.has_children() {
            printer.close_tag();
            self.children.print(printer)?;
            printer.end_tag(super::NAME)?;
        } else {
            printer.closed_tag();
        }
        Ok(())
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
