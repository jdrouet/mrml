use super::MJBody;
use crate::elements::body::BodyElement;
use crate::elements::Error;
use crate::parser::Node;
use crate::util::attributes::*;
use crate::util::header::Header;

lazy_static! {
    static ref DEFAULT_ATTRIBUTES: Attributes = Attributes::default().add("width", "600px");
}

impl MJBody {
    fn default_attributes<'a>(node: &Node<'a>, header: &Header) -> Attributes {
        header
            .default_attributes
            .get_attributes(node, DEFAULT_ATTRIBUTES.clone())
    }

    pub fn parse<'a>(node: &Node<'a>, header: &Header) -> Result<MJBody, Error> {
        let mut children = vec![];
        for child in node.children.iter() {
            children.push(BodyElement::parse(child, header, None::<&Attributes>)?);
        }
        Ok(MJBody {
            attributes: Self::default_attributes(node, header).concat(node),
            children,
            context: None,
            exists: true,
        })
    }
}
