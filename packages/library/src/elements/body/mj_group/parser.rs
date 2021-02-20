use super::MJGroup;
use crate::elements::body::BodyElement;
use crate::elements::error::Error;
use crate::parser::{MJMLParser, Node};
use crate::util::attributes::*;
use crate::util::header::Header;

lazy_static! {
    static ref DEFAULT_ATTRIBUTES: Attributes = Attributes::default().add("direction", "ltr");
}

struct MJGroupParser<'h> {
    header: &'h Header,
    attributes: Attributes,
    children: Vec<BodyElement>,
}

impl<'h> MJGroupParser<'h> {
    pub fn new(header: &'h Header) -> Self {
        Self {
            header,
            attributes: Attributes::default(),
            children: Vec::new(),
        }
    }

    fn default_attributes<'a>(node: &Node<'a>, header: &Header) -> Attributes {
        header
            .default_attributes
            .get_attributes(node, DEFAULT_ATTRIBUTES.clone())
    }
}

impl<'h> MJMLParser for MJGroupParser<'h> {
    type Output = MJGroup;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(MJGroup {
            attributes: self.attributes,
            context: None,
            children: self.children,
        })
    }

    fn parse<'a>(mut self, node: &Node<'a>) -> Result<Self, Error> {
        self.attributes = Self::default_attributes(node, self.header).concat(node);
        let child_attrs = Attributes::default().add("mobile-width", "mobile-width");
        for child in node.children.iter() {
            self.children
                .push(BodyElement::parse(&child, self.header, Some(&child_attrs))?);
        }
        Ok(self)
    }
}

impl MJGroup {
    pub fn parse<'a>(node: &Node<'a>, header: &Header) -> Result<MJGroup, Error> {
        MJGroupParser::new(header).parse(node)?.build()
    }
}
