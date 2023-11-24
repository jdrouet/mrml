use xmlparser::StrSpan;

use super::{MjRaw, MjRawChild};
use crate::comment::Comment;
use crate::node::Node;
use crate::prelude::parser::{
    AttributesParser, ChildrenParser, ElementParser, Error, MrmlCursor, MrmlToken,
};
use crate::text::Text;

impl<'a> ElementParser<'a, Node<MjRawChild>> for MrmlCursor<'a> {
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

impl<'a> ChildrenParser<'a, Vec<MjRawChild>> for MrmlCursor<'a> {
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
                MrmlToken::ElementClose(close) => {
                    self.rewind(MrmlToken::ElementClose(close));
                    return Ok(children);
                }
                other => return Err(Error::UnexpectedToken(other.span())),
            }
        }
    }
}

impl<'a> ElementParser<'a, MjRaw> for MrmlCursor<'a> {
    fn parse(&mut self, _tag: StrSpan<'a>) -> Result<MjRaw, Error> {
        let ending = self.assert_element_end()?;
        if ending.empty {
            return Ok(MjRaw {
                children: Default::default(),
            });
        }
        let children = self.parse_children()?;
        self.assert_element_close()?;

        Ok(MjRaw { children })
    }
}
