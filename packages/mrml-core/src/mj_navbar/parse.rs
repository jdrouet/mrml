use xmlparser::StrSpan;

use super::{MjNavbar, MjNavbarChild};
use crate::comment::Comment;
use crate::mj_navbar_link::NAME as MJ_NAVBAR_LINK;
use crate::prelude::parser::{
    ChildrenParser, ElementParser, Error, MrmlParser, MrmlToken,
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
                        return Err(Error::UnexpectedElement(inner.span.into()));
                    }
                }
                MrmlToken::ElementClose(inner) => {
                    self.rewind(MrmlToken::ElementClose(inner));
                    return Ok(result);
                }
                other => return Err(Error::UnexpectedToken(other.span())),
            }
        }
    }
}

impl<'a> ElementParser<'a, MjNavbar> for MrmlParser<'a> {
    fn parse(&mut self, _tag: StrSpan<'a>) -> Result<MjNavbar, Error> {
        let (attributes, children) = self.parse_attributes_and_children()?;

        Ok(MjNavbar {
            attributes,
            children,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::MjNavbar;
    use crate::prelude::parser::MrmlParser;

    macro_rules! assert_success {
        ($title:ident, $template:expr) => {
            #[test]
            fn $title() {
                let _: MjNavbar = MrmlParser::new($template, Default::default())
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
                let _: MjNavbar = MrmlParser::new($template, Default::default())
                    .parse_root()
                    .unwrap();
            }
        };
    }

    assert_success!(should_handle_empty_children, "<mj-navbar />");

    assert_success!(
        should_handle_comments,
        "<mj-navbar><!-- comment --></mj-navbar>"
    );

    assert_fail!(
        should_error_with_text,
        "<mj-navbar>Hello</mj-navbar>",
        "UnexpectedToken(Span { start: 11, end: 16 })"
    );

    assert_fail!(
        should_error_with_other_element,
        "<mj-navbar><span /></mj-navbar>",
        "UnexpectedElement(Span { start: 11, end: 16 })"
    );
}
