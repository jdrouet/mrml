use super::Node;
use crate::mj_body::MJBodyChild;
use crate::prelude::parse::{Error, Parser};
use crate::{parse_attribute, parse_child, parse_comment, parse_text};
use xmlparser::Tokenizer;

#[derive(Debug)]
struct NodeParser(Node);

impl NodeParser {
    pub fn new(tag: String) -> Self {
        Self(Node::new(tag))
    }
}

impl Parser for NodeParser {
    type Output = Node;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(self.0)
    }

    parse_attribute!();
    parse_child!(MJBodyChild);
    parse_comment!();
    parse_text!();
}

impl Node {
    pub fn parse(tag: String, tokenizer: &mut Tokenizer) -> Result<Self, Error> {
        NodeParser::new(tag).parse(tokenizer)?.build()
    }
}
