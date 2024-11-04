use crate::prelude::print::PrintableElement;
use crate::prelude::AttributeMap;

impl PrintableElement for super::MjAttributesElement {
    type Attrs = AttributeMap;
    type Children = ();

    fn tag(&self) -> &str {
        self.name.as_str()
    }

    fn attributes(&self) -> &Self::Attrs {
        &self.attributes
    }

    fn children(&self) -> &Self::Children {
        &()
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
