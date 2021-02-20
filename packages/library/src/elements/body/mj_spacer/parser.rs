use super::MJSpacer;
use crate::elements::error::Error;
use crate::parser::{MJMLParser, Node};
use crate::util::attributes::*;
use crate::util::header::Header;

lazy_static! {
    static ref DEFAULT_ATTRIBUTES: Attributes = Attributes::default().add("height", "20px");
}

struct MJSpacerParser<'h> {
    header: &'h Header,
    attributes: Attributes,
}

impl<'h> MJSpacerParser<'h> {
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

impl<'h> MJMLParser for MJSpacerParser<'h> {
    type Output = MJSpacer;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(MJSpacer {
            attributes: self.attributes,
            context: None,
        })
    }

    fn parse<'a>(mut self, node: &Node<'a>) -> Result<Self, Error> {
        self.attributes = Self::default_attributes(node, self.header).concat(node);
        Ok(self)
    }
}

impl MJSpacer {
    pub fn parse<'a>(node: &Node<'a>, header: &Header) -> Result<MJSpacer, Error> {
        MJSpacerParser::new(header).parse(node)?.build()
    }
}
