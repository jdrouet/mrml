use super::MJImage;
use crate::elements::error::Error;
use crate::parser::{MJMLParser, Node};
use crate::util::attributes::*;
use crate::util::header::Header;

lazy_static! {
    static ref DEFAULT_ATTRIBUTES: Attributes = Attributes::default()
        .add("align", "center")
        .add("border", "0")
        .add("height", "auto")
        .add("padding", "10px 25px")
        .add("target", "_blank")
        .add("font-size", "13px");
}

struct MJImageParser<'h> {
    header: &'h Header,
    attributes: Attributes,
}

impl<'h> MJImageParser<'h> {
    pub fn new(header: &'h Header) -> Self {
        Self {
            header,
            attributes: Attributes::default(),
        }
    }

    fn default_attributes<'a>(node: &Node<'a>, header: &Header) -> Attributes {
        header
            .default_attributes
            .get_attributes(node, DEFAULT_ATTRIBUTES.clone())
    }
}

impl<'h> MJMLParser for MJImageParser<'h> {
    type Output = MJImage;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(MJImage {
            attributes: self.attributes,
            context: None,
        })
    }

    fn parse<'a>(mut self, node: &Node<'a>) -> Result<Self, Error> {
        self.attributes = Self::default_attributes(node, self.header).concat(node);
        Ok(self)
    }
}

impl MJImage {
    pub fn parse<'a>(node: &Node<'a>, header: &Header) -> Result<MJImage, Error> {
        MJImageParser::new(header).parse(node)?.build()
    }
}
