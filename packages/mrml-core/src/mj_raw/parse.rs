use std::rc::Rc;

use xmlparser::{StrSpan, Tokenizer};

use super::{MjRaw, MjRawChild};
use crate::comment::Comment;
use crate::node::Node;
use crate::prelude::parser::{
    AttributesParser, ChildrenParser, ElementParser, Error, MrmlParser, MrmlToken, Parsable,
    Parser, ParserOptions,
};
use crate::text::Text;
use crate::{parse_child, parse_comment, parse_text};

impl<'a> ElementParser<'a, Node<MjRawChild>> for MrmlParser<'a> {
    fn parse(&mut self, tag: StrSpan<'a>) -> Result<Node<MjRawChild>, Error> {
        let attributes = self.parse_attributes()?;
        let ending = self.assert_element_end()?;
        if ending.empty {
            return Ok(Node {
                tag: tag.to_string(),
                attributes,
                children: Vec::new(),
            });
        }

        let children = self.parse_children()?;
        self.assert_element_close()?;

        Ok(Node {
            tag: tag.to_string(),
            attributes,
            children,
        })
    }
}

impl<'a> ChildrenParser<'a, Vec<MjRawChild>> for MrmlParser<'a> {
    fn parse_children(&mut self) -> Result<Vec<MjRawChild>, Error> {
        let mut children = Vec::new();
        loop {
            let token = self.assert_next()?;
            match token {
                MrmlToken::Comment(inner) => {
                    children.push(MjRawChild::Comment(Comment::from(inner.text.as_str())));
                }
                MrmlToken::ElementStart(elt) => {
                    children.push(MjRawChild::Node(self.parse(elt.local)?));
                }
                MrmlToken::Text(inner) => {
                    children.push(MjRawChild::Text(Text::from(inner.text.as_str())));
                }
                MrmlToken::ElementClose(_) => return Ok(children),
                other => return Err(Error::unexpected_token(other.range())),
            }
        }
    }
}

impl<'a> ElementParser<'a, MjRaw> for MrmlParser<'a> {
    fn parse(&mut self, _tag: StrSpan<'a>) -> Result<MjRaw, Error> {
        let ending = self.next_element_end()?.ok_or(Error::EndOfStream)?;
        let children = if !ending.empty {
            self.parse_children()?
        } else {
            Default::default()
        };

        Ok(MjRaw { children })
    }
}

impl Parsable for MjRawChild {
    fn parse<'a>(
        tag: StrSpan<'a>,
        tokenizer: &mut Tokenizer<'a>,
        opts: Rc<ParserOptions>,
    ) -> Result<Self, Error> {
        Ok(Node::<MjRawChild>::parse(tag, tokenizer, opts)?.into())
    }
}

#[derive(Debug, Default)]
struct MjRawParser {
    opts: Rc<ParserOptions>,
    children: Vec<MjRawChild>,
}

impl MjRawParser {
    fn new(opts: Rc<ParserOptions>) -> Self {
        Self {
            opts,
            children: Vec::new(),
        }
    }
}

impl Parser for MjRawParser {
    type Output = MjRaw;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(MjRaw {
            children: self.children,
        })
    }

    parse_child!(MjRawChild);
    parse_comment!();
    parse_text!();
}

impl Parsable for MjRaw {
    fn parse(
        _tag: StrSpan,
        tokenizer: &mut Tokenizer,
        opts: Rc<ParserOptions>,
    ) -> Result<Self, Error> {
        MjRawParser::new(opts).parse(tokenizer)?.build()
    }
}
