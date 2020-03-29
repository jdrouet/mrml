use super::{MJAccordionTitle, NAME as MJ_ACCORDION_TITLE};
use crate::elements::error::Error;
use crate::parser::Node;
use crate::util::attributes::*;
use crate::util::header::Header;

lazy_static! {
    static ref DEFAULT_ATTRIBUTES: Attributes = Attributes::default()
        .add("font-size", "13px")
        .add("padding", "16px");
}

impl MJAccordionTitle {
    fn default_attributes<'a>(node: &Node<'a>, header: &Header) -> Attributes {
        header
            .default_attributes
            .get_attributes(node, DEFAULT_ATTRIBUTES.clone())
    }

    pub fn parse<'a>(
        node: &Node<'a>,
        header: &Header,
        attrs: &Attributes,
    ) -> Result<MJAccordionTitle, Error> {
        if node.name.as_str() != MJ_ACCORDION_TITLE {
            return Err(Error::UnexpectedElement(node.name.as_str().into()));
        }
        let content: String = node
            .children
            .iter()
            .filter_map(|child| child.as_text())
            .map(|value| value.as_str())
            .collect::<String>();
        let attributes = Self::default_attributes(node, header)
            .concat(attrs)
            .concat(node);
        Ok(MJAccordionTitle {
            attributes,
            context: None,
            content,
        })
    }
}
