use xmlparser::StrSpan;

use super::{MjBody, MjBodyChild};
use crate::comment::Comment;
use crate::mj_accordion::NAME as MJ_ACCORDION;
use crate::mj_button::NAME as MJ_BUTTON;
use crate::mj_carousel::NAME as MJ_CAROUSEL;
use crate::mj_column::NAME as MJ_COLUMN;
use crate::mj_divider::NAME as MJ_DIVIDER;
use crate::mj_group::NAME as MJ_GROUP;
use crate::mj_hero::NAME as MJ_HERO;
use crate::mj_image::NAME as MJ_IMAGE;
use crate::mj_include::NAME as MJ_INCLUDE;
use crate::mj_navbar::NAME as MJ_NAVBAR;
use crate::mj_raw::NAME as MJ_RAW;
use crate::mj_section::NAME as MJ_SECTION;
use crate::mj_social::NAME as MJ_SOCIAL;
use crate::mj_spacer::NAME as MJ_SPACER;
use crate::mj_table::NAME as MJ_TABLE;
use crate::mj_text::NAME as MJ_TEXT;
use crate::mj_wrapper::NAME as MJ_WRAPPER;
use crate::node::Node;
use crate::prelude::parser::{
    AttributesParser, ChildrenParser, ElementParser, Error, MrmlCursor, MrmlToken,
};
use crate::text::Text;

fn should_ignore_children(tag: &str) -> bool {
    matches!(
        tag,
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

impl<'a> ElementParser<'a, Node<MjBodyChild>> for MrmlCursor<'a> {
    fn parse(&mut self, tag: StrSpan<'a>) -> Result<Node<MjBodyChild>, Error> {
        let tag = tag.to_string();
        let attributes = self.parse_attributes()?;
        let ending = self.assert_element_end()?;
        if ending.empty || should_ignore_children(tag.as_str()) {
            return Ok(Node {
                tag,
                attributes,
                children: Vec::new(),
            });
        }
        let children = self.parse_children()?;

        self.assert_element_close()?;

        Ok(Node {
            tag,
            attributes,
            children,
        })
    }
}

impl<'a> ElementParser<'a, MjBodyChild> for MrmlCursor<'a> {
    fn parse(&mut self, tag: StrSpan<'a>) -> Result<MjBodyChild, Error> {
        match tag.as_str() {
            MJ_ACCORDION => Ok(MjBodyChild::MjAccordion(self.parse(tag)?)),
            MJ_BUTTON => Ok(MjBodyChild::MjButton(self.parse(tag)?)),
            MJ_CAROUSEL => Ok(MjBodyChild::MjCarousel(self.parse(tag)?)),
            MJ_COLUMN => Ok(MjBodyChild::MjColumn(self.parse(tag)?)),
            MJ_DIVIDER => Ok(MjBodyChild::MjDivider(self.parse(tag)?)),
            MJ_GROUP => Ok(MjBodyChild::MjGroup(self.parse(tag)?)),
            MJ_HERO => Ok(MjBodyChild::MjHero(self.parse(tag)?)),
            MJ_IMAGE => Ok(MjBodyChild::MjImage(self.parse(tag)?)),
            MJ_INCLUDE => Ok(MjBodyChild::MjInclude(self.parse(tag)?)),
            MJ_NAVBAR => Ok(MjBodyChild::MjNavbar(self.parse(tag)?)),
            MJ_RAW => Ok(MjBodyChild::MjRaw(self.parse(tag)?)),
            MJ_SECTION => Ok(MjBodyChild::MjSection(self.parse(tag)?)),
            MJ_SOCIAL => Ok(MjBodyChild::MjSocial(self.parse(tag)?)),
            MJ_SPACER => Ok(MjBodyChild::MjSpacer(self.parse(tag)?)),
            MJ_TABLE => Ok(MjBodyChild::MjTable(self.parse(tag)?)),
            MJ_TEXT => Ok(MjBodyChild::MjText(self.parse(tag)?)),
            MJ_WRAPPER => Ok(MjBodyChild::MjWrapper(self.parse(tag)?)),
            _ => Ok(MjBodyChild::Node(self.parse(tag)?)),
        }
    }
}

impl<'a> ChildrenParser<'a, Vec<MjBodyChild>> for MrmlCursor<'a> {
    fn parse_children(&mut self) -> Result<Vec<MjBodyChild>, Error> {
        let mut result = Vec::new();
        while let Some(token) = self.next_token() {
            match token? {
                MrmlToken::Comment(inner) => {
                    result.push(MjBodyChild::Comment(Comment::from(inner.text.as_str())));
                }
                MrmlToken::Text(inner) => {
                    result.push(MjBodyChild::Text(Text::from(inner.text.as_str())));
                }
                MrmlToken::ElementStart(inner) => {
                    result.push(self.parse(inner.local)?);
                }
                MrmlToken::ElementClose(close) => {
                    self.rewind(MrmlToken::ElementClose(close));
                    return Ok(result);
                }
                other => {
                    return Err(Error::UnexpectedToken(other.span()));
                }
            }
        }
        Ok(result)
    }
}

impl<'a> ElementParser<'a, MjBody> for MrmlCursor<'a> {
    fn parse(&mut self, _tag: StrSpan<'a>) -> Result<MjBody, Error> {
        let (attributes, children) = self.parse_attributes_and_children()?;

        Ok(MjBody {
            attributes,
            children,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_body::MjBody;
    use crate::prelude::parser::MrmlCursor;

    #[test]
    fn parse_complete() {
        let raw = r#"
<mj-body>
    <!-- Some comment -->
    <mj-button>Hello World</mj-button>
</mj-body>
        "#;
        let body: MjBody = MrmlCursor::new(raw, Default::default())
            .parse_root()
            .unwrap();
        assert_eq!(body.children.len(), 2);
    }
}
