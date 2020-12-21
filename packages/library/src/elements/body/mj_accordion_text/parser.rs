use super::MJAccordionText;
use crate::elements::body::raw::RawElement;
use crate::elements::error::Error;
use crate::parser::Node;
use crate::util::attributes::*;
use crate::util::header::Header;

lazy_static! {
    static ref DEFAULT_ATTRIBUTES: Attributes = Attributes::default()
        .add("line-height", "1")
        .add("font-size", "13px")
        .add("padding", "16px");
}

impl MJAccordionText {
    fn default_attributes<'a>(node: &Node<'a>, header: &Header) -> Attributes {
        header
            .default_attributes
            .get_attributes(node, DEFAULT_ATTRIBUTES.clone())
    }

    pub fn parse<'a>(
        node: &Node<'a>,
        header: &Header,
        attrs: &Attributes,
    ) -> Result<MJAccordionText, Error> {
        if node.name.as_str() != "mj-accordion-text" {
            return Err(Error::UnexpectedElement(node.name.as_str().into()));
        }
        let attributes = Self::default_attributes(node, header)
            .concat(attrs)
            .concat(node);
        let mut element = MJAccordionText::new(attributes);
        for child in node.children.iter() {
            element
                .children
                .push(RawElement::conditional_parse(&child, header, true)?);
        }
        Ok(element)
    }
}
