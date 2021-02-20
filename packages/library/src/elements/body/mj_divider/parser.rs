use super::MJDivider;
use crate::elements::error::Error;
use crate::parser::{MJMLParser, Node};
use crate::util::attributes::*;
use crate::util::header::Header;

lazy_static! {
    static ref DEFAULT_ATTRIBUTES: Attributes = Attributes::default()
        .add("border-color", "#000000")
        .add("border-style", "solid")
        .add("border-width", "4px")
        .add("padding", "10px 25px")
        .add("width", "100%");
}

struct MJDividerParser<'h> {
    header: &'h Header,
    attributes: Attributes,
}

impl<'h> MJDividerParser<'h> {
    fn default_attributes<'a>(node: &Node<'a>, header: &Header) -> Attributes {
        header
            .default_attributes
            .get_attributes(node, DEFAULT_ATTRIBUTES.clone())
    }

    pub fn new(header: &'h Header) -> Self {
        Self {
            header,
            attributes: Attributes::new(),
        }
    }
}

impl<'h> MJMLParser for MJDividerParser<'h> {
    type Output = MJDivider;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(MJDivider {
            attributes: self.attributes,
            context: None,
        })
    }

    fn parse<'a>(mut self, node: &Node<'a>) -> Result<Self, Error> {
        self.attributes = Self::default_attributes(node, self.header).concat(node);
        Ok(self)
    }
}

impl MJDivider {
    pub fn parse<'a>(node: &Node<'a>, header: &Header) -> Result<MJDivider, Error> {
        MJDividerParser::new(header).parse(node)?.build()
    }
}
