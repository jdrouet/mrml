use crate::prelude::print::{Printable, PrintableAttributes, PrintableChildren};

impl Printable for super::MjAccordion {
    fn print<P: crate::prelude::print::Printer>(&self, printer: &mut P) -> std::fmt::Result {
        printer.open_tag(super::NAME)?;
        self.attributes.print(printer)?;
        if self.children.is_empty() {
            printer.closed_tag();
        } else {
            printer.close_tag();
            self.children.print(printer)?;
            printer.end_tag(super::NAME)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::comment::Comment;
    use crate::mj_accordion::{MjAccordion, MjAccordionChild};
    use crate::mj_accordion_element::{MjAccordionElement, MjAccordionElementChildren};
    use crate::mj_accordion_text::MjAccordionText;
    use crate::mj_accordion_title::MjAccordionTitle;
    use crate::prelude::print::Printable;
    use crate::text::Text;

    #[test]
    fn empty() {
        let item = MjAccordion::default();
        assert_eq!("<mj-accordion />", item.print_dense().unwrap());
    }

    #[test]
    fn with_children() {
        let item = MjAccordion {
            attributes: Default::default(),
            children: vec![
                MjAccordionChild::Comment(Comment {
                    children: "Hello World!".into(),
                }),
                MjAccordionChild::MjAccordionElement(MjAccordionElement {
                    attributes: Default::default(),
                    children: MjAccordionElementChildren {
                        title: Some(MjAccordionTitle {
                            attributes: Default::default(),
                            children: vec![Text::from("Foo".to_string())],
                        }),
                        text: Some(MjAccordionText {
                            attributes: Default::default(),
                            children: vec![Text::from("Bar".to_string()).into()],
                        }),
                    },
                }),
            ],
        };
        assert_eq!("<mj-accordion><!--Hello World!--><mj-accordion-element><mj-accordion-title>Foo</mj-accordion-title><mj-accordion-text>Bar</mj-accordion-text></mj-accordion-element></mj-accordion>", item.print_dense().unwrap());
    }
}
