use crate::{
    comment::Comment,
    prelude::parser::{Error, MrmlCursor, MrmlParser, MrmlToken, ParseChildren, ParserOptions},
};

use super::RootChild;

impl<'opts> crate::prelude::parser::ParseChildren<Vec<RootChild>> for MrmlParser<'opts> {
    fn parse_children(&self, cursor: &mut MrmlCursor<'_>) -> Result<Vec<RootChild>, Error> {
        use crate::prelude::parser::ParseElement;

        let mut result = Vec::new();
        while let Some(token) = cursor.next_token() {
            match token? {
                MrmlToken::Comment(inner) => {
                    result.push(RootChild::Comment(Comment::from(inner.text.as_str())));
                }
                MrmlToken::ElementStart(inner) if inner.local.eq("mjml") => {
                    result.push(RootChild::Mjml(self.parse(cursor, inner.local)?));
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
impl crate::prelude::parser::AsyncParseChildren<Vec<RootChild>>
    for crate::prelude::parser::AsyncMrmlParser
{
    async fn async_parse_children<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
    ) -> Result<Vec<RootChild>, Error> {
        use crate::prelude::parser::AsyncParseElement;

        let mut result = Vec::new();
        while let Some(token) = cursor.next_token() {
            match token? {
                MrmlToken::Comment(inner) => {
                    result.push(RootChild::Comment(Comment::from(inner.text.as_str())));
                }
                MrmlToken::ElementStart(inner) if inner.local.eq("mjml") => {
                    let element = self.async_parse(cursor, inner.local).await?;
                    result.push(RootChild::Mjml(element));
                }
                other => {
                    return Err(Error::UnexpectedToken(other.span()));
                }
            }
        }
        Ok(result)
    }
}

impl super::Root {
    /// Function to parse a raw mjml template with some parsing
    /// [options](crate::prelude::parser::ParserOptions).
    pub(crate) fn parse_with_options<T: AsRef<str>>(
        value: T,
        opts: &ParserOptions,
    ) -> Result<Self, Error> {
        let parser = MrmlParser::new(opts);
        let mut cursor = MrmlCursor::new(value.as_ref());
        Ok(Self(parser.parse_children(&mut cursor)?))
    }

    #[cfg(feature = "async")]
    pub(crate) async fn async_parse_with_options<T: AsRef<str>>(
        value: T,
        opts: std::sync::Arc<crate::prelude::parser::AsyncParserOptions>,
    ) -> Result<Self, Error> {
        use crate::prelude::parser::AsyncMrmlParser;
        use crate::prelude::parser::AsyncParseChildren;

        let parser = AsyncMrmlParser::new(opts);
        let mut cursor = MrmlCursor::new(value.as_ref());
        Ok(Self(parser.async_parse_children(&mut cursor).await?))
    }
}
