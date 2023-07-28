use std::rc::Rc;

use xmlparser::{StrSpan, Tokenizer};

use super::{MjBody, MjBodyChild};
use crate::comment::Comment;
use crate::mj_accordion::{MjAccordion, NAME as MJ_ACCORDION};
use crate::mj_button::{MjButton, NAME as MJ_BUTTON};
use crate::mj_carousel::{MjCarousel, NAME as MJ_CAROUSEL};
use crate::mj_column::{MjColumn, NAME as MJ_COLUMN};
use crate::mj_divider::{MjDivider, NAME as MJ_DIVIDER};
use crate::mj_group::{MjGroup, NAME as MJ_GROUP};
use crate::mj_hero::{MjHero, NAME as MJ_HERO};
use crate::mj_image::{MjImage, NAME as MJ_IMAGE};
use crate::mj_include::body::MjIncludeBody;
use crate::mj_include::NAME as MJ_INCLUDE;
use crate::mj_navbar::{MjNavbar, NAME as MJ_NAVBAR};
use crate::mj_raw::{MjRaw, NAME as MJ_RAW};
use crate::mj_section::{MjSection, NAME as MJ_SECTION};
use crate::mj_social::{MjSocial, NAME as MJ_SOCIAL};
use crate::mj_spacer::{MjSpacer, NAME as MJ_SPACER};
use crate::mj_table::{MjTable, NAME as MJ_TABLE};
use crate::mj_text::{MjText, NAME as MJ_TEXT};
use crate::mj_wrapper::{MjWrapper, NAME as MJ_WRAPPER};
use crate::node::Node;
use crate::prelude::parser::{
    AttributesParser, ChildrenParser, ElementParser, Error, MrmlParser, MrmlToken, Parsable,
    ParserOptions,
};
use crate::text::Text;

impl<'a> ElementParser<'a, Node<MjBodyChild>> for MrmlParser<'a> {
    fn parse(&mut self, tag: StrSpan<'a>) -> Result<Node<MjBodyChild>, Error> {
        let tag = tag.to_string();
        let attributes = self.parse_attributes()?;
        let ending = self.assert_element_end()?;
        if ending.empty {
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

impl<'a> ElementParser<'a, MjBodyChild> for MrmlParser<'a> {
    fn parse(&mut self, tag: StrSpan<'a>) -> Result<MjBodyChild, Error> {
        match tag.as_str() {
            // MJ_ACCORDION => Ok(MjAccordion::parse(tag, tokenizer, opts)?.into()),
            MJ_BUTTON => Ok(MjBodyChild::MjButton(self.parse(tag)?)),
            // MJ_CAROUSEL => Ok(MjCarousel::parse(tag, tokenizer, opts)?.into()),
            MJ_COLUMN => Ok(MjBodyChild::MjColumn(self.parse(tag)?)),
            MJ_DIVIDER => Ok(MjBodyChild::MjDivider(self.parse(tag)?)),
            MJ_GROUP => Ok(MjBodyChild::MjGroup(self.parse(tag)?)),
            MJ_HERO => Ok(MjBodyChild::MjHero(self.parse(tag)?)),
            MJ_IMAGE => Ok(MjBodyChild::MjImage(self.parse(tag)?)),
            // MJ_INCLUDE => Ok(MjIncludeBody::parse(tag, tokenizer, opts)?.into()),
            MJ_NAVBAR => Ok(MjBodyChild::MjNavbar(self.parse(tag)?)),
            MJ_RAW => Ok(MjBodyChild::MjRaw(self.parse(tag)?)),
            MJ_SECTION => Ok(MjBodyChild::MjSection(self.parse(tag)?)),
            // MJ_SOCIAL => Ok(MjSocial::parse(tag, tokenizer, opts)?.into()),
            MJ_SPACER => Ok(MjBodyChild::MjSpacer(self.parse(tag)?)),
            MJ_TABLE => Ok(MjBodyChild::MjTable(self.parse(tag)?)),
            MJ_TEXT => Ok(MjBodyChild::MjText(self.parse(tag)?)),
            MJ_WRAPPER => Ok(MjBodyChild::MjWrapper(self.parse(tag)?)),
            _ => Ok(MjBodyChild::Node(self.parse(tag)?)),
        }
    }
}

impl<'a> ChildrenParser<'a, Vec<MjBodyChild>> for MrmlParser<'a> {
    fn parse_children(&mut self) -> Result<Vec<MjBodyChild>, Error> {
        let mut result = Vec::new();
        loop {
            match self.assert_next()? {
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
                    return Err(Error::unexpected_token(other.range()));
                }
            }
        }
    }
}

impl<'a> ElementParser<'a, MjBody> for MrmlParser<'a> {
    fn parse(&mut self, _tag: StrSpan<'a>) -> Result<MjBody, Error> {
        let attributes = self.parse_attributes()?;
        let ending = self.assert_element_end()?;
        if ending.empty {
            return Ok(MjBody {
                attributes,
                children: Vec::new(),
            });
        }

        let children = self.parse_children()?;

        self.assert_element_close()?;

        Ok(MjBody {
            attributes,
            children,
        })
    }
}

impl Parsable for MjBodyChild {
    fn parse<'a>(
        tag: StrSpan<'a>,
        tokenizer: &mut Tokenizer<'a>,
        opts: Rc<ParserOptions>,
    ) -> Result<Self, Error> {
        match tag.as_str() {
            MJ_ACCORDION => Ok(MjAccordion::parse(tag, tokenizer, opts)?.into()),
            MJ_BUTTON => Ok(MjButton::parse(tag, tokenizer, opts)?.into()),
            MJ_CAROUSEL => Ok(MjCarousel::parse(tag, tokenizer, opts)?.into()),
            MJ_COLUMN => Ok(MjColumn::parse(tag, tokenizer, opts)?.into()),
            MJ_DIVIDER => Ok(MjDivider::parse(tag, tokenizer, opts)?.into()),
            MJ_GROUP => Ok(MjGroup::parse(tag, tokenizer, opts)?.into()),
            MJ_HERO => Ok(MjHero::parse(tag, tokenizer, opts)?.into()),
            MJ_IMAGE => Ok(MjImage::parse(tag, tokenizer, opts)?.into()),
            MJ_INCLUDE => Ok(MjIncludeBody::parse(tag, tokenizer, opts)?.into()),
            MJ_NAVBAR => Ok(MjNavbar::parse(tag, tokenizer, opts)?.into()),
            MJ_RAW => Ok(MjRaw::parse(tag, tokenizer, opts)?.into()),
            MJ_SECTION => Ok(MjSection::parse(tag, tokenizer, opts)?.into()),
            MJ_SOCIAL => Ok(MjSocial::parse(tag, tokenizer, opts)?.into()),
            MJ_SPACER => Ok(MjSpacer::parse(tag, tokenizer, opts)?.into()),
            MJ_TABLE => Ok(MjTable::parse(tag, tokenizer, opts)?.into()),
            MJ_TEXT => Ok(MjText::parse(tag, tokenizer, opts)?.into()),
            MJ_WRAPPER => Ok(MjWrapper::parse(tag, tokenizer, opts)?.into()),
            _ => Ok(Node::<MjBodyChild>::parse(tag, tokenizer, opts)?.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::mjml::Mjml;

    #[test]
    fn parse_complete() {
        let template = r#"
        <mjml>
            <mj-body>
                <!-- Some comment -->
                <mj-button>Hello World</mj-button>
            </mj-body>
        </mjml>
        "#;
        let elt = Mjml::parse(template).unwrap();
        assert!(elt.head().is_none());
        assert!(elt.body().is_some());
        let body = elt.body().unwrap();
        assert_eq!(body.children.len(), 2);
    }
}
