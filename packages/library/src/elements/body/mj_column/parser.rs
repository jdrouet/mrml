use super::MJColumn;
use crate::elements::body::BodyElement;
use crate::elements::error::Error;
use crate::parser::Node;
use crate::util::attributes::*;
use crate::util::header::Header;

lazy_static! {
    static ref DEFAULT_ATTRIBUTES: Attributes = Attributes::default()
        .add("direction", "ltr")
        .add("vertical-align", "top");
}

impl MJColumn {
    fn default_attributes<'a>(node: &Node<'a>, header: &Header) -> Attributes {
        header
            .default_attributes
            .get_attributes(node, DEFAULT_ATTRIBUTES.clone())
    }

    pub fn parse<'a>(
        node: &Node<'a>,
        header: &Header,
        extra: Option<&Attributes>,
    ) -> Result<MJColumn, Error> {
        let mut children = vec![];
        for child in node.children.iter() {
            children.push(BodyElement::parse(&child, header, None::<&Attributes>)?);
        }
        let mut attributes = Self::default_attributes(node, header);
        if let Some(extra) = extra {
            attributes.merge(extra);
        }
        Ok(MJColumn {
            attributes: attributes.concat(node),
            context: None,
            children,
        })
    }
}
