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
use crate::prelude::is_void_element;
use crate::prelude::parser::{
    parse_attributes_map, Error, MrmlCursor, MrmlParser, MrmlToken, ParseChildren, ParseElement,
};
#[cfg(feature = "async")]
use crate::prelude::parser::{AsyncMrmlParser, AsyncParseChildren, AsyncParseElement};
use crate::text::Text;

impl<'opts> ParseElement<Node<MjBodyChild>> for MrmlParser<'opts> {
    fn parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        tag: StrSpan<'a>,
    ) -> Result<Node<MjBodyChild>, Error> {
        let tag = tag.to_string();
        let attributes = parse_attributes_map(cursor)?;
        let ending = cursor.assert_element_end()?;
        if ending.empty || is_void_element(tag.as_str()) {
            return Ok(Node {
                tag,
                attributes,
                children: Vec::new(),
            });
        }
        let children = self.parse_children(cursor)?;

        cursor.assert_element_close()?;

        Ok(Node {
            tag,
            attributes,
            children,
        })
    }
}

#[cfg(feature = "async")]
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl AsyncParseElement<Node<MjBodyChild>> for AsyncMrmlParser {
    async fn async_parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        tag: StrSpan<'a>,
    ) -> Result<Node<MjBodyChild>, Error> {
        let tag = tag.to_string();
        let attributes = parse_attributes_map(cursor)?;
        let ending = cursor.assert_element_end()?;
        if ending.empty || is_void_element(tag.as_str()) {
            return Ok(Node {
                tag,
                attributes,
                children: Vec::new(),
            });
        }
        let children = self.async_parse_children(cursor).await?;

        cursor.assert_element_close()?;

        Ok(Node {
            tag,
            attributes,
            children,
        })
    }
}

impl<'opts> ParseElement<MjBodyChild> for MrmlParser<'opts> {
    fn parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        tag: StrSpan<'a>,
    ) -> Result<MjBodyChild, Error> {
        match tag.as_str() {
            MJ_ACCORDION => Ok(MjBodyChild::MjAccordion(self.parse(cursor, tag)?)),
            MJ_BUTTON => Ok(MjBodyChild::MjButton(self.parse(cursor, tag)?)),
            MJ_CAROUSEL => Ok(MjBodyChild::MjCarousel(self.parse(cursor, tag)?)),
            MJ_COLUMN => Ok(MjBodyChild::MjColumn(self.parse(cursor, tag)?)),
            MJ_DIVIDER => Ok(MjBodyChild::MjDivider(self.parse(cursor, tag)?)),
            MJ_GROUP => Ok(MjBodyChild::MjGroup(self.parse(cursor, tag)?)),
            MJ_HERO => Ok(MjBodyChild::MjHero(self.parse(cursor, tag)?)),
            MJ_IMAGE => Ok(MjBodyChild::MjImage(self.parse(cursor, tag)?)),
            MJ_INCLUDE => Ok(MjBodyChild::MjInclude(self.parse(cursor, tag)?)),
            MJ_NAVBAR => Ok(MjBodyChild::MjNavbar(self.parse(cursor, tag)?)),
            MJ_RAW => Ok(MjBodyChild::MjRaw(self.parse(cursor, tag)?)),
            MJ_SECTION => Ok(MjBodyChild::MjSection(self.parse(cursor, tag)?)),
            MJ_SOCIAL => Ok(MjBodyChild::MjSocial(self.parse(cursor, tag)?)),
            MJ_SPACER => Ok(MjBodyChild::MjSpacer(self.parse(cursor, tag)?)),
            MJ_TABLE => Ok(MjBodyChild::MjTable(self.parse(cursor, tag)?)),
            MJ_TEXT => Ok(MjBodyChild::MjText(self.parse(cursor, tag)?)),
            MJ_WRAPPER => Ok(MjBodyChild::MjWrapper(self.parse(cursor, tag)?)),
            _ => Ok(MjBodyChild::Node(self.parse(cursor, tag)?)),
        }
    }
}

#[cfg(feature = "async")]
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl AsyncParseElement<MjBodyChild> for AsyncMrmlParser {
    async fn async_parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        tag: StrSpan<'a>,
    ) -> Result<MjBodyChild, Error> {
        match tag.as_str() {
            MJ_ACCORDION => Ok(MjBodyChild::MjAccordion(
                self.async_parse(cursor, tag).await?,
            )),
            MJ_BUTTON => Ok(MjBodyChild::MjButton(self.async_parse(cursor, tag).await?)),
            MJ_CAROUSEL => Ok(MjBodyChild::MjCarousel(
                self.async_parse(cursor, tag).await?,
            )),
            MJ_COLUMN => Ok(MjBodyChild::MjColumn(self.async_parse(cursor, tag).await?)),
            MJ_DIVIDER => Ok(MjBodyChild::MjDivider(self.async_parse(cursor, tag).await?)),
            MJ_GROUP => Ok(MjBodyChild::MjGroup(self.async_parse(cursor, tag).await?)),
            MJ_HERO => Ok(MjBodyChild::MjHero(self.async_parse(cursor, tag).await?)),
            MJ_IMAGE => Ok(MjBodyChild::MjImage(self.async_parse(cursor, tag).await?)),
            MJ_INCLUDE => Ok(MjBodyChild::MjInclude(self.async_parse(cursor, tag).await?)),
            MJ_NAVBAR => Ok(MjBodyChild::MjNavbar(self.async_parse(cursor, tag).await?)),
            MJ_RAW => Ok(MjBodyChild::MjRaw(self.async_parse(cursor, tag).await?)),
            MJ_SECTION => Ok(MjBodyChild::MjSection(self.async_parse(cursor, tag).await?)),
            MJ_SOCIAL => Ok(MjBodyChild::MjSocial(self.async_parse(cursor, tag).await?)),
            MJ_SPACER => Ok(MjBodyChild::MjSpacer(self.async_parse(cursor, tag).await?)),
            MJ_TABLE => Ok(MjBodyChild::MjTable(self.async_parse(cursor, tag).await?)),
            MJ_TEXT => Ok(MjBodyChild::MjText(self.async_parse(cursor, tag).await?)),
            MJ_WRAPPER => Ok(MjBodyChild::MjWrapper(self.async_parse(cursor, tag).await?)),
            _ => Ok(MjBodyChild::Node(self.async_parse(cursor, tag).await?)),
        }
    }
}

impl<'opts> ParseChildren<Vec<MjBodyChild>> for MrmlParser<'opts> {
    fn parse_children(&self, cursor: &mut MrmlCursor<'_>) -> Result<Vec<MjBodyChild>, Error> {
        let mut result = Vec::new();
        while let Some(token) = cursor.next_token() {
            match token? {
                MrmlToken::Comment(inner) => {
                    result.push(MjBodyChild::Comment(Comment::from(inner.text.as_str())));
                }
                MrmlToken::Text(inner) => {
                    result.push(MjBodyChild::Text(Text::from(inner.text.as_str())));
                }
                MrmlToken::ElementStart(inner) => {
                    result.push(self.parse(cursor, inner.local)?);
                }
                MrmlToken::ElementClose(close) => {
                    cursor.rewind(MrmlToken::ElementClose(close));
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

#[cfg(feature = "async")]
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl AsyncParseChildren<Vec<MjBodyChild>> for AsyncMrmlParser {
    async fn async_parse_children<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
    ) -> Result<Vec<MjBodyChild>, Error> {
        let mut result = Vec::new();
        while let Some(token) = cursor.next_token() {
            match token? {
                MrmlToken::Comment(inner) => {
                    result.push(MjBodyChild::Comment(Comment::from(inner.text.as_str())));
                }
                MrmlToken::Text(inner) => {
                    result.push(MjBodyChild::Text(Text::from(inner.text.as_str())));
                }
                MrmlToken::ElementStart(inner) => {
                    result.push(self.async_parse(cursor, inner.local).await?);
                }
                MrmlToken::ElementClose(close) => {
                    cursor.rewind(MrmlToken::ElementClose(close));
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

impl<'opts> ParseElement<MjBody> for MrmlParser<'opts> {
    fn parse<'a>(&self, cursor: &mut MrmlCursor<'a>, _tag: StrSpan<'a>) -> Result<MjBody, Error> {
        let (attributes, children) = self.parse_attributes_and_children(cursor)?;

        Ok(MjBody {
            attributes,
            children,
        })
    }
}

#[cfg(feature = "async")]
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl AsyncParseElement<MjBody> for AsyncMrmlParser {
    async fn async_parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        _tag: StrSpan<'a>,
    ) -> Result<MjBody, Error> {
        let (attributes, children) = self.parse_attributes_and_children(cursor).await?;

        Ok(MjBody {
            attributes,
            children,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_body::MjBody;

    crate::should_parse!(
        parse_complete,
        MjBody,
        r#"<mj-body>
    <!-- Some comment -->
    <mj-button>Hello World</mj-button>
</mj-body>"#
    );

    crate::should_async_parse!(
        async_parse_complete,
        MjBody,
        r#"<mj-body>
    <!-- Some comment -->
    <mj-button>Hello World</mj-button>
</mj-body>"#
    );
}
