use super::Node;
use crate::comment::Comment;
use crate::prelude::parse::{Error, Parsable, Parser};
use crate::text::Text;
use crate::{parse_attribute, parse_comment, parse_text};
use xmlparser::{StrSpan, Tokenizer};

#[derive(Debug)]
struct NodeParser<T>(Node<T>);

impl<T> NodeParser<T> {
    pub fn new(tag: String) -> Self {
        Self(Node::new(tag))
    }
}

impl<T> Parser for NodeParser<T>
where
    T: Parsable,
    T: From<Comment>,
    T: From<Text>,
{
    type Output = Node<T>;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(self.0)
    }

    fn should_ignore_children(&self) -> bool {
        matches!(
            self.0.tag.as_str(),
            "area"
                | "base"
                | "br"
                | "col"
                | "embed"
                | "hr"
                | "img"
                | "input"
                | "link"
                | "meta"
                | "param"
                | "source"
                | "track"
                | "wbr"
        )
    }

    parse_attribute!();

    fn parse_child_element<'a>(
        &mut self,
        tag: xmlparser::StrSpan<'a>,
        tokenizer: &mut xmlparser::Tokenizer<'a>,
    ) -> Result<(), Error> {
        self.0.children.push(T::parse(tag, tokenizer)?);
        Ok(())
    }

    parse_comment!();
    parse_text!();
}

impl<T> Parsable for Node<T>
where
    T: Parsable,
    T: From<Comment>,
    T: From<Text>,
{
    fn parse<'a>(tag: StrSpan<'a>, tokenizer: &mut Tokenizer<'a>) -> Result<Self, Error> {
        NodeParser::<T>::new(tag.to_string())
            .parse(tokenizer)?
            .build()
    }
}
