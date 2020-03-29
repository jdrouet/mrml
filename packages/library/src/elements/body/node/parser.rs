use super::Node;
use crate::elements::body::raw::RawElement;
use crate::elements::body::BodyElement;
use crate::elements::Error;
use crate::parser::Node as ParserNode;
use crate::util::attributes::Attributes;
use crate::util::header::Header;

impl Node {
    pub fn parse<'a>(element: &ParserNode<'a>, header: &Header) -> Result<Node, Error> {
        Node::conditional_parse(element, header, false)
    }

    pub fn conditional_parse<'a>(
        node: &ParserNode<'a>,
        header: &Header,
        only_raw: bool,
    ) -> Result<Node, Error> {
        let tag = node.name.as_str();
        if only_raw && tag.starts_with("mj-") {
            return Err(Error::UnexpectedElement(tag.into()));
        }
        let mut children = vec![];
        for child in node.children.iter() {
            if only_raw {
                children.push(RawElement::conditional_parse(child, header, true)?.into())
            } else {
                children.push(BodyElement::parse(&child, header, None)?);
            }
        }
        Ok(Node {
            name: node.name.as_str().to_string(),
            attributes: Attributes::from(node),
            context: None,
            children,
        })
    }
}
