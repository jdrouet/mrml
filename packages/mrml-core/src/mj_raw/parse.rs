use xmlparser::StrSpan;

use super::{MjRaw, MjRawChild};
use crate::comment::Comment;
use crate::node::Node;
use crate::prelude::parser::{
    Error, MrmlCursor, MrmlParser, MrmlToken, ParseAttributes, ParseChildren, ParseElement,
};
use crate::text::Text;

impl ParseElement<Node<MjRawChild>> for MrmlParser {
    fn parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        tag: StrSpan<'a>,
    ) -> Result<Node<MjRawChild>, Error> {
        let attributes = self.parse_attributes(cursor)?;
        let ending = cursor.assert_element_end()?;
        if ending.empty {
            return Ok(Node {
                tag: tag.to_string(),
                attributes,
                children: Vec::new(),
            });
        }

        let children = self.parse_children(cursor)?;
        cursor.assert_element_close()?;

        Ok(Node {
            tag: tag.to_string(),
            attributes,
            children,
        })
    }
}

impl ParseChildren<Vec<MjRawChild>> for MrmlParser {
    fn parse_children(&self, cursor: &mut MrmlCursor<'_>) -> Result<Vec<MjRawChild>, Error> {
        let mut children = Vec::new();
        loop {
            let token = cursor.assert_next()?;
            match token {
                MrmlToken::Comment(inner) => {
                    children.push(MjRawChild::Comment(Comment::from(inner.text.as_str())));
                }
                MrmlToken::ElementStart(elt) => {
                    children.push(MjRawChild::Node(self.parse(cursor, elt.local)?));
                }
                MrmlToken::Text(inner) => {
                    children.push(MjRawChild::Text(Text::from(inner.text.as_str())));
                }
                MrmlToken::ElementClose(close) => {
                    cursor.rewind(MrmlToken::ElementClose(close));
                    return Ok(children);
                }
                other => return Err(Error::UnexpectedToken(other.span())),
            }
        }
    }
}

impl ParseElement<MjRaw> for MrmlParser {
    fn parse<'a>(&self, cursor: &mut MrmlCursor<'a>, _: StrSpan<'a>) -> Result<MjRaw, Error> {
        let ending = cursor.assert_element_end()?;
        if ending.empty {
            return Ok(MjRaw {
                children: Default::default(),
            });
        }
        let children = self.parse_children(cursor)?;
        cursor.assert_element_close()?;

        Ok(MjRaw { children })
    }
}
