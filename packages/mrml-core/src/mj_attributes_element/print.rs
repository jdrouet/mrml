use crate::prelude::print::{PrintableAttributes, PrintableElement};

impl PrintableElement for super::MjAttributesElement {
    fn tag(&self) -> &str {
        self.name.as_str()
    }

    fn attributes(&self) -> &impl PrintableAttributes {
        &self.attributes
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::print::Printable;

    #[test]
    fn empty() {
        let item = crate::mj_attributes_element::MjAttributesElement::new("span".to_string());
        assert_eq!("<span />", item.print_dense().unwrap());
    }
}
