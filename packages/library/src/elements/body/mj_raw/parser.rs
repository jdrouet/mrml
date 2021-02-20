use super::MJRaw;
use crate::elements::body::raw::RawElement;
use crate::elements::body::BodyElement;
use crate::elements::error::Error;
use crate::parser::{MJMLParser, Node};
use crate::util::header::Header;

struct MJRawParser<'h> {
    header: &'h Header,
    children: Vec<BodyElement>,
}

impl<'h> MJRawParser<'h> {
    pub fn new(header: &'h Header) -> Self {
        Self {
            header,
            children: vec![],
        }
    }
}

impl<'h> MJMLParser for MJRawParser<'h> {
    type Output = MJRaw;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(MJRaw {
            context: None,
            children: self.children,
        })
    }

    fn parse<'a>(mut self, node: &Node<'a>) -> Result<Self, Error> {
        for child in node.children.iter() {
            self.children
                .push(RawElement::conditional_parse(&child, self.header, true)?.into());
        }
        Ok(self)
    }
}

impl MJRaw {
    pub fn parse<'a>(node: &Node<'a>, header: &Header) -> Result<MJRaw, Error> {
        MJRawParser::new(header).parse(node)?.build()
    }
}
