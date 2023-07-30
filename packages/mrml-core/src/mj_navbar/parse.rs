use xmlparser::StrSpan;

use super::{MjNavbar, MjNavbarChild};
use crate::comment::Comment;
use crate::mj_navbar_link::NAME as MJ_NAVBAR_LINK;
use crate::prelude::parser::{
    AttributesParser, ChildrenParser, ElementParser, Error, MrmlParser, MrmlToken,
};

impl<'a> ChildrenParser<'a, Vec<MjNavbarChild>> for MrmlParser<'a> {
    fn parse_children(&mut self) -> Result<Vec<MjNavbarChild>, Error> {
        let mut result = Vec::new();

        loop {
            match self.assert_next()? {
                MrmlToken::Comment(inner) => {
                    result.push(MjNavbarChild::Comment(Comment::from(inner.text.as_str())));
                }
                MrmlToken::ElementStart(inner) => {
                    if inner.local.as_str() == MJ_NAVBAR_LINK {
                        result.push(MjNavbarChild::MjNavbarLink(self.parse(inner.local)?));
                    } else {
                        return Err(Error::UnexpectedElement(inner.span.start()));
                    }
                }
                MrmlToken::ElementClose(inner) => {
                    self.rewind(MrmlToken::ElementClose(inner));
                    return Ok(result);
                }
                other => return Err(Error::unexpected_token(other.range())),
            }
        }
    }
}

impl<'a> ElementParser<'a, MjNavbar> for MrmlParser<'a> {
    fn parse(&mut self, _tag: StrSpan<'a>) -> Result<MjNavbar, Error> {
        let attributes = self.parse_attributes()?;
        let ending = self.assert_element_end()?;

        if ending.empty {
            return Ok(MjNavbar {
                attributes,
                children: Vec::new(),
            });
        }

        let children = self.parse_children()?;
        self.assert_element_close()?;

        Ok(MjNavbar {
            attributes,
            children,
        })
    }
}
