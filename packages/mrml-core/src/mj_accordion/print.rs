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
        let item = MjAccordion::new(
            Default::default(),
            vec![
                MjAccordionChild::Comment(Comment::new((), "Hello World!".into())),
                MjAccordionChild::MjAccordionElement(MjAccordionElement::new(
                    Default::default(),
                    MjAccordionElementChildren {
                        title: Some(MjAccordionTitle::new(
                            Default::default(),
                            vec![Text::from("Foo".to_string())],
                        )),
                        text: Some(MjAccordionText::new(
                            Default::default(),
                            vec![Text::from("Bar".to_string()).into()],
                        )),
                    },
                )),
            ],
        );
        assert_eq!("<mj-accordion><!--Hello World!--><mj-accordion-element><mj-accordion-title>Foo</mj-accordion-title><mj-accordion-text>Bar</mj-accordion-text></mj-accordion-element></mj-accordion>", item.print_dense().unwrap());
    }
}
