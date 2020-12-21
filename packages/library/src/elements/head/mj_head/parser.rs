use super::MJHead;
use crate::elements::head::prelude::HeadComponent;
use crate::elements::head::HeadElement;
use crate::elements::Error;
use crate::parser::Node;
use crate::util::header::Header;
use crate::Options;
use std::collections::HashMap;

impl<'a> MJHead<'a> {
    pub fn parse(node: &Node<'a>, opts: Options) -> Result<MJHead<'a>, Error> {
        let children = HeadElement::parse_all(&node.children)?;
        let mut header = Header::from(opts);
        for child in children.iter() {
            child.update_header(&mut header);
        }
        Ok(MJHead {
            attributes: node
                .attributes
                .iter()
                .fold(HashMap::new(), |mut res, (key, value)| {
                    res.insert(key.as_str(), value.as_str());
                    res
                }),
            context: None,
            children,
            header,
        })
    }
}
