use super::Node;
use crate::elements::body::raw::RawElement;
use crate::elements::body::BodyElement;
use crate::elements::Error;
use crate::parser::{MJMLParser, Node as ParserNode};
use crate::util::attributes::Attributes;
use crate::util::header::Header;

struct NodeParser<'h> {
    header: &'h Header,
    only_raw: bool,
    name: String,
    attributes: Attributes,
    children: Vec<BodyElement>,
}

impl<'h> NodeParser<'h> {
    pub fn new(header: &'h Header, only_raw: bool) -> Self {
        Self {
            header,
            only_raw,
            name: String::new(),
            attributes: Attributes::default(),
            children: Vec::new(),
        }
    }
}

impl<'h> MJMLParser for NodeParser<'h> {
    type Output = Node;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(Node {
            name: self.name,
            attributes: self.attributes,
            context: None,
            children: self.children,
        })
    }

    fn parse<'a>(mut self, node: &ParserNode<'a>) -> Result<Self, Error> {
        self.name = node.name.to_string();
        if self.only_raw && self.name.starts_with("mj-") {
            return Err(Error::UnexpectedElement(self.name));
        }
        self.attributes = Attributes::from(node);
        for child in node.children.iter() {
            if self.only_raw {
                self.children
                    .push(RawElement::conditional_parse(child, self.header, true)?.into())
            } else {
                self.children
                    .push(BodyElement::parse(&child, self.header, None)?);
            }
        }
        Ok(self)
    }
}

impl Node {
    pub fn parse<'a>(element: &ParserNode<'a>, header: &Header) -> Result<Node, Error> {
        Node::conditional_parse(element, header, false)
    }

    pub fn conditional_parse<'a>(
        node: &ParserNode<'a>,
        header: &Header,
        only_raw: bool,
    ) -> Result<Node, Error> {
        NodeParser::new(header, only_raw).parse(node)?.build()
    }
}
