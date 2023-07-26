use xmlparser::{StrSpan, Tokenizer};

use super::Node;
use crate::comment::Comment;
use crate::prelude::hash::Map;
use crate::prelude::parse::{Error, Parsable, Parser};
use crate::text::Text;
use crate::{parse_attribute, parse_comment, parse_text};

#[derive(Debug)]
struct NodeParser<T> {
    opts: std::rc::Rc<crate::prelude::parse::ParserOptions>,
    tag: String,
    attributes: Map<String, String>,
    children: Vec<T>,
}

impl<T> NodeParser<T> {
    pub fn new(tag: String, opts: std::rc::Rc<crate::prelude::parse::ParserOptions>) -> Self {
        Self {
            opts,
            tag,
            attributes: Default::default(),
            children: Vec::new(),
        }
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
        Ok(Node {
            tag: self.tag,
            attributes: self.attributes,
            children: self.children,
        })
    }

    fn should_ignore_children(&self) -> bool {
        matches!(
            self.tag.as_str(),
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
        self.children
            .push(T::parse(tag, tokenizer, self.opts.clone())?);
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
    fn parse<'a>(
        tag: StrSpan<'a>,
        tokenizer: &mut Tokenizer<'a>,
        opts: std::rc::Rc<crate::prelude::parse::ParserOptions>,
    ) -> Result<Self, Error> {
        NodeParser::<T>::new(tag.to_string(), opts)
            .parse(tokenizer)?
            .build()
    }
}
