use xmlparser::StrSpan;

use super::{MjSocial, MjSocialChild};
use crate::comment::Comment;
use crate::mj_social_element::NAME as MJ_SOCIAL_ELEMENT;
use crate::prelude::parser::{
    AttributesParser, ChildrenParser, ElementParser, Error, MrmlParser, MrmlToken,
};

impl<'a> ChildrenParser<'a, Vec<MjSocialChild>> for MrmlParser<'a> {
    fn parse_children(&mut self) -> Result<Vec<MjSocialChild>, Error> {
        let mut result = Vec::new();

        loop {
            match self.assert_next()? {
                MrmlToken::Comment(inner) => {
                    result.push(MjSocialChild::Comment(Comment::from(inner.text.as_str())));
                }
                MrmlToken::ElementStart(inner) => {
                    if inner.local.as_str() == MJ_SOCIAL_ELEMENT {
                        result.push(MjSocialChild::MjSocialElement(self.parse(inner.local)?));
                    } else {
                        return Err(Error::UnexpectedElement(inner.span.start()));
                    }
                }
                MrmlToken::ElementClose(inner) => {
                    self.rewind(MrmlToken::ElementClose(inner));
                    return Ok(result);
                }
                MrmlToken::Text(inner) if inner.text.trim().is_empty() => {}
                other => return Err(Error::unexpected_token(other.range())),
            }
        }
    }
}

impl<'a> ElementParser<'a, MjSocial> for MrmlParser<'a> {
    fn parse(&mut self, _tag: StrSpan<'a>) -> Result<MjSocial, Error> {
        let attributes = self.parse_attributes()?;
        let ending = self.assert_element_end()?;
        if ending.empty {
            return Ok(MjSocial {
                attributes,
                children: Vec::new(),
            });
        }

        let children = self.parse_children()?;
        self.assert_element_close()?;

        Ok(MjSocial {
            attributes,
            children,
        })
    }
}
