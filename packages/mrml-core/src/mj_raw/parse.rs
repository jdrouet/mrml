use htmlparser::StrSpan;

use super::MjRawChild;
use crate::comment::Comment;
use crate::conditional_comment::ConditionalComment;
use crate::node::Node;
use crate::prelude::is_void_element;
#[cfg(feature = "async")]
use crate::prelude::parser::{AsyncMrmlParser, AsyncParseChildren, AsyncParseElement};
use crate::prelude::parser::{
    Error, MrmlCursor, MrmlParser, MrmlToken, ParseAttributes, ParseChildren, ParseElement,
};
use crate::text::Text;

impl ParseElement<Node<MjRawChild>> for MrmlParser<'_> {
    fn parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        tag: StrSpan<'a>,
    ) -> Result<Node<MjRawChild>, Error> {
        let attributes = self.parse_attributes(cursor, &tag)?;
        let ending = cursor.assert_element_end()?;
        if ending.empty || is_void_element(tag.as_str()) {
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

#[cfg(feature = "async")]
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl AsyncParseElement<Node<MjRawChild>> for AsyncMrmlParser {
    async fn async_parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        tag: StrSpan<'a>,
    ) -> Result<Node<MjRawChild>, Error> {
        let attributes = self.parse_attributes(cursor, &tag)?;
        let ending = cursor.assert_element_end()?;
        if ending.empty || is_void_element(tag.as_str()) {
            return Ok(Node {
                tag: tag.to_string(),
                attributes,
                children: Vec::new(),
            });
        }

        let children = self.async_parse_children(cursor).await?;
        cursor.assert_element_close()?;

        Ok(Node {
            tag: tag.to_string(),
            attributes,
            children,
        })
    }
}

impl ParseChildren<Vec<MjRawChild>> for MrmlParser<'_> {
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
                MrmlToken::ConditionalCommentStart(start) => {
                    children.push(MjRawChild::ConditionalComment(ConditionalComment::from(
                        start.span.as_str(),
                    )));
                }
                MrmlToken::ConditionalCommentEnd(end) => {
                    children.push(MjRawChild::ConditionalComment(ConditionalComment::from(
                        end.span.as_str(),
                    )));
                }
                other => {
                    return Err(Error::UnexpectedToken {
                        origin: cursor.origin(),
                        position: other.span(),
                    });
                }
            }
        }
    }
}

#[cfg(feature = "async")]
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl AsyncParseChildren<Vec<MjRawChild>> for AsyncMrmlParser {
    async fn async_parse_children<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
    ) -> Result<Vec<MjRawChild>, Error> {
        let mut children = Vec::new();
        loop {
            let token = cursor.assert_next()?;
            match token {
                MrmlToken::Comment(inner) => {
                    children.push(MjRawChild::Comment(Comment::from(inner.text.as_str())));
                }
                MrmlToken::ElementStart(elt) => {
                    children.push(MjRawChild::Node(self.async_parse(cursor, elt.local).await?));
                }
                MrmlToken::Text(inner) => {
                    children.push(MjRawChild::Text(Text::from(inner.text.as_str())));
                }
                MrmlToken::ElementClose(close) => {
                    cursor.rewind(MrmlToken::ElementClose(close));
                    return Ok(children);
                }
                MrmlToken::ConditionalCommentStart(start) => {
                    children.push(MjRawChild::ConditionalComment(ConditionalComment::from(
                        start.span.as_str(),
                    )));
                }
                MrmlToken::ConditionalCommentEnd(end) => {
                    children.push(MjRawChild::ConditionalComment(ConditionalComment::from(
                        end.span.as_str(),
                    )));
                }
                other => {
                    return Err(Error::UnexpectedToken {
                        origin: cursor.origin(),
                        position: other.span(),
                    })
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_raw::MjRaw;

    crate::should_parse!(
        should_parse_start_conditional_comment_child,
        MjRaw,
        "<mj-raw><!--[if mso]></mj-raw>"
    );

    crate::should_parse!(
        should_parse_end_conditional_comment_child,
        MjRaw,
        "<mj-raw><![endif]--></mj-raw>"
    );

    crate::should_parse!(
        should_parse_conditional_comment_children,
        MjRaw,
        "<mj-raw><!--[if mso]>--><!--[if mso]><!--<![endif]--></mj-raw>"
    );

    crate::should_not_parse!(
        should_not_parse_malformed_conditional_comment_child,
        MjRaw,
        "<mj-raw><!- -[if mso]>--></mj-raw>"
    );
}
