use super::MJHead;
use crate::elements::head::prelude::HeadComponent;
use crate::elements::head::HeadElement;
use crate::elements::Error;
use crate::parser::Node;
use crate::util::header::Header;
use crate::Options;

impl<'a> MJHead {
    pub fn parse(node: &Node<'a>, opts: Options) -> Result<MJHead, Error> {
        let children = HeadElement::parse_all(&node.children)?;
        let mut header = Header::from(opts);
        for child in children.iter() {
            child.update_header(&mut header);
        }
        Ok(MJHead {
            attributes: node
                .attributes
                .iter()
                .map(|(key, value)| (key.to_string(), value.to_string()))
                .collect(),
            context: None,
            children,
            header,
        })
    }
}
