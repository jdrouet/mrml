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
#[cfg(test)]
mod tests {
    use super::MjSocial;
    use crate::prelude::parser::MrmlParser;

    macro_rules! assert_success {
        ($title:ident, $template:expr) => {
            #[test]
            fn $title() {
                let _: MjSocial = MrmlParser::new($template, Default::default())
                    .parse_root()
                    .unwrap();
            }
        };
    }

    macro_rules! assert_fail {
        ($title:ident, $template:expr, $error:expr) => {
            #[test]
            #[should_panic(expected = $error)]
            fn $title() {
                let _: MjSocial = MrmlParser::new($template, Default::default())
                    .parse_root()
                    .unwrap();
            }
        };
    }

    assert_success!(should_handle_empty_children, "<mj-social />");

    assert_success!(
        should_handle_comments,
        "<mj-social><!-- comment --></mj-social>"
    );

    assert_fail!(
        should_error_with_text,
        "<mj-social>Hello</mj-social>",
        "UnexpectedToken(11, 16)"
    );

    assert_fail!(
        should_error_with_other_element,
        "<mj-social><span /></mj-social>",
        "UnexpectedElement(11)"
    );
}
