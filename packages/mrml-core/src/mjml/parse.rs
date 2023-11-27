use xmlparser::StrSpan;

use super::{Mjml, MjmlAttributes, MjmlChildren};
use crate::mj_body::NAME as MJ_BODY;
use crate::mj_head::NAME as MJ_HEAD;
use crate::prelude::parser::{
    Error, MrmlCursor, MrmlParser, MrmlToken, ParseAttributes, ParseChildren, ParseElement,
};

impl ParseAttributes<MjmlAttributes> for MrmlParser {
    fn parse_attributes<'a>(&self, cursor: &mut MrmlCursor<'a>) -> Result<MjmlAttributes, Error> {
        let mut attrs = MjmlAttributes::default();
        while let Some(token) = cursor.next_attribute()? {
            match token.local.as_str() {
                "owa" => attrs.owa = Some(token.value.to_string()),
                "lang" => attrs.lang = Some(token.value.to_string()),
                "dir" => attrs.dir = Some(token.value.to_string()),
                _ => return Err(Error::UnexpectedAttribute(token.span.into())),
            }
        }
        Ok(attrs)
    }
}

impl ParseChildren<MjmlChildren> for MrmlParser {
    fn parse_children<'a>(&self, cursor: &mut MrmlCursor<'a>) -> Result<MjmlChildren, Error> {
        let mut children = MjmlChildren::default();

        loop {
            match cursor.assert_next()? {
                MrmlToken::ElementClose(close) if close.local.as_str() == super::NAME => {
                    cursor.rewind(MrmlToken::ElementClose(close));
                    return Ok(children);
                }
                MrmlToken::ElementStart(start) => match start.local.as_str() {
                    MJ_HEAD => {
                        children.head = Some(self.parse(cursor, start.local)?);
                    }
                    MJ_BODY => {
                        children.body = Some(self.parse(cursor, start.local)?);
                    }
                    _ => {
                        return Err(Error::UnexpectedElement(start.span.into()));
                    }
                },
                other => {
                    return Err(Error::UnexpectedToken(other.span()));
                }
            }
        }
    }
}

#[cfg(feature = "async")]
#[async_trait::async_trait(?Send)]
impl crate::prelude::parser::AsyncParseChildren<MjmlChildren> for MrmlParser {
    async fn async_parse_children<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
    ) -> Result<MjmlChildren, Error> {
        use crate::prelude::parser::AsyncParseElement;

        let mut children = MjmlChildren::default();

        loop {
            match cursor.assert_next()? {
                MrmlToken::ElementClose(close) if close.local.as_str() == super::NAME => {
                    cursor.rewind(MrmlToken::ElementClose(close));
                    return Ok(children);
                }
                MrmlToken::ElementStart(start) => match start.local.as_str() {
                    MJ_HEAD => {
                        children.head = Some(self.async_parse(cursor, start.local).await?);
                    }
                    MJ_BODY => {
                        children.body = Some(self.async_parse(cursor, start.local).await?);
                    }
                    _ => {
                        return Err(Error::UnexpectedElement(start.span.into()));
                    }
                },
                other => {
                    return Err(Error::UnexpectedToken(other.span()));
                }
            }
        }
    }
}

impl ParseElement<Mjml> for MrmlParser {
    fn parse<'a>(&self, cursor: &mut MrmlCursor<'a>, _tag: StrSpan<'a>) -> Result<Mjml, Error> {
        let (attributes, children) = self.parse_attributes_and_children(cursor)?;

        Ok(Mjml {
            attributes,
            children,
        })
    }
}

#[cfg(feature = "async")]
#[async_trait::async_trait(?Send)]
impl crate::prelude::parser::AsyncParseElement<Mjml> for MrmlParser {
    async fn async_parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        _tag: StrSpan<'a>,
    ) -> Result<Mjml, Error> {
        let (attributes, children) = self.async_parse_attributes_and_children(cursor).await?;

        Ok(Mjml {
            attributes,
            children,
        })
    }
}

impl Mjml {
    /// Function to parse a raw mjml template with some parsing
    /// [options](crate::prelude::parser::ParserOptions).
    ///
    /// You can specify the kind of loader mrml needs to use for loading the
    /// content of [`mj-include`](crate::mj_include) elements.
    ///
    /// You can take a look at the available loaders
    /// [here](crate::prelude::parse).
    ///
    /// ```rust
    /// use mrml::mjml::Mjml;
    /// use mrml::prelude::parser::ParserOptions;
    /// use mrml::prelude::parser::memory_loader::MemoryIncludeLoader;
    /// use std::sync::Arc;
    ///
    /// let options = Arc::new(ParserOptions {
    ///     include_loader: Box::new(MemoryIncludeLoader::default()),
    /// });
    /// match Mjml::parse_with_options("<mjml><mj-head /><mj-body /></mjml>", options) {
    ///     Ok(_) => println!("Success!"),
    ///     Err(err) => eprintln!("Something went wrong: {err:?}"),
    /// }
    /// ```
    pub fn parse_with_options<T: AsRef<str>>(
        value: T,
        opts: std::sync::Arc<crate::prelude::parser::ParserOptions>,
    ) -> Result<Self, Error> {
        let parser = MrmlParser::new(opts);
        let mut cursor = MrmlCursor::new(value.as_ref());
        parser.parse_root(&mut cursor)
    }

    /// Function to parse a raw mjml template using the default parsing
    /// [options](crate::prelude::parser::ParserOptions).
    pub fn parse<T: AsRef<str>>(value: T) -> Result<Self, Error> {
        let parser = MrmlParser::default();
        let mut cursor = MrmlCursor::new(value.as_ref());
        parser.parse_root(&mut cursor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_with_options() {
        let template = "<mjml></mjml>";
        let elt = Mjml::parse_with_options(template, Default::default()).unwrap();
        assert!(elt.children.body.is_none());
        assert!(elt.children.head.is_none());
    }

    #[test]
    fn should_parse() {
        let template = "<mjml></mjml>";
        let elt = Mjml::parse(template).unwrap();
        assert!(elt.children.body.is_none());
        assert!(elt.children.head.is_none());
    }

    #[test]
    fn should_parse_without_children() {
        let template = "<mjml />";
        let elt: Mjml = Mjml::parse(template).unwrap();
        assert!(elt.children.body.is_none());
        assert!(elt.children.head.is_none());
    }

    #[test]
    fn should_parse_with_lang() {
        let template = "<mjml lang=\"fr\"></mjml>";
        let elt = Mjml::parse(template).unwrap();
        assert_eq!(elt.attributes.lang.unwrap(), "fr");
    }

    #[test]
    fn should_parse_with_owa() {
        let template = "<mjml owa=\"desktop\"></mjml>";
        let elt = Mjml::parse(template).unwrap();
        assert_eq!(elt.attributes.owa.unwrap(), "desktop");
    }

    #[test]
    fn should_parse_with_dir() {
        let template = "<mjml dir=\"rtl\"></mjml>";
        let elt = Mjml::parse(template).unwrap();
        assert_eq!(elt.attributes.dir.unwrap(), "rtl");
    }

    #[test]
    #[should_panic(expected = "UnexpectedAttribute(Span { start: 6, end: 20 })")]
    fn should_fail_with_unknown_param() {
        let template = "<mjml unknown=\"true\"></mjml>";
        let elt = Mjml::parse(template).unwrap();
        assert_eq!(elt.attributes.dir.unwrap(), "rtl");
    }

    #[test]
    #[should_panic(expected = "UnexpectedToken(Span { start: 6, end: 11 })")]
    fn should_fail_with_text_as_child() {
        let template = "<mjml>Hello</mjml>";
        let _ = Mjml::parse(template).unwrap();
    }

    #[test]
    #[should_panic(expected = "UnexpectedElement(Span { start: 6, end: 10 })")]
    fn should_fail_with_other_child() {
        let template = "<mjml><div /></mjml>";
        let _ = Mjml::parse(template).unwrap();
    }
}
