use super::MJHead;
use crate::elements::head::prelude::HeadComponent;
use crate::elements::head::HeadElement;
use crate::elements::Error;
use crate::parser::{MJMLParser, Node};
use crate::util::header::Header;
use crate::Options;

struct MJHeadParser {
    options: Options,
    head: Option<MJHead>,
}

impl MJHeadParser {
    pub fn new(options: Options) -> Self {
        Self {
            options,
            head: None,
        }
    }
}

impl MJMLParser for MJHeadParser {
    type Output = MJHead;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(self.head.unwrap())
    }

    fn parse<'a>(mut self, node: &Node<'a>) -> Result<Self, Error> {
        let children = HeadElement::parse_all(&node.children)?;
        let mut header = Header::from(&self.options);
        for child in children.iter() {
            child.update_header(&mut header);
        }
        self.head = Some(MJHead {
            attributes: node
                .attributes
                .iter()
                .map(|(key, value)| (key.to_string(), value.to_string()))
                .collect(),
            context: None,
            children,
            header,
        });
        Ok(self)
    }
}

impl<'a> MJHead {
    pub fn parse(node: &Node<'a>, opts: Options) -> Result<MJHead, Error> {
        MJHeadParser::new(opts).parse(node)?.build()
    }
}
