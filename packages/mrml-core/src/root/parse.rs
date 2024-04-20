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
    ///
    /// You can specify the kind of loader mrml needs to use for loading the
    /// content of [`mj-include`](crate::mj_include) elements.
    ///
    /// You can take a look at the available loaders
    /// [here](crate::prelude::parser).
    ///
    /// ```rust
    /// use mrml::root::Root;
    /// use mrml::prelude::parser::ParserOptions;
    /// use mrml::prelude::parser::memory_loader::MemoryIncludeLoader;
    ///
    /// let options = ParserOptions {
    ///     include_loader: Box::new(MemoryIncludeLoader::default()),
    /// };
    /// match Root::parse_with_options("<mjml><mj-head /><mj-body /></mjml>", &options) {
    ///     Ok(_) => println!("Success!"),
    ///     Err(err) => eprintln!("Something went wrong: {err:?}"),
    /// }
    /// ```
    pub fn parse_with_options<T: AsRef<str>>(
        value: T,
        opts: &ParserOptions,
    ) -> Result<Self, Error> {
        let parser = MrmlParser::new(opts);
        let mut cursor = MrmlCursor::new(value.as_ref());
        Ok(Self(parser.parse_children(&mut cursor)?))
    }

    #[cfg(feature = "async")]
    pub async fn async_parse_with_options<T: AsRef<str>>(
        value: T,
        opts: std::sync::Arc<crate::prelude::parser::AsyncParserOptions>,
    ) -> Result<Self, Error> {
        use crate::prelude::parser::AsyncMrmlParser;
        use crate::prelude::parser::AsyncParseChildren;

        let parser = AsyncMrmlParser::new(opts);
        let mut cursor = MrmlCursor::new(value.as_ref());
        Ok(Self(parser.async_parse_children(&mut cursor).await?))
    }

    /// Function to parse a raw mjml template using the default parsing
    /// [options](crate::prelude::parser::ParserOptions).
    pub fn parse<T: AsRef<str>>(value: T) -> Result<Self, Error> {
        let opts = ParserOptions::default();
        let parser = MrmlParser::new(&opts);
        let mut cursor = MrmlCursor::new(value.as_ref());
        Ok(Self(parser.parse_children(&mut cursor)?))
    }

    #[cfg(feature = "async")]
    /// Function to parse a raw mjml template using the default parsing
    /// [options](crate::prelude::parser::ParserOptions).
    pub async fn async_parse<T: AsRef<str>>(value: T) -> Result<Self, Error> {
        use crate::prelude::parser::AsyncMrmlParser;
        use crate::prelude::parser::AsyncParseChildren;

        let parser = AsyncMrmlParser::default();
        let mut cursor = MrmlCursor::new(value.as_ref());
        Ok(Self(parser.async_parse_children(&mut cursor).await?))
    }
}
