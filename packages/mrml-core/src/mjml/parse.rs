use xmlparser::StrSpan;

use super::{Mjml, MjmlAttributes, MjmlChildren};
use crate::mj_body::NAME as MJ_BODY;
use crate::mj_head::NAME as MJ_HEAD;
use crate::prelude::parser::Error;
use crate::prelude::parser::{
    self, AttributesParser, ChildrenParser, ElementParser, MrmlParser, MrmlToken,
};

impl<'a> AttributesParser<'a, MjmlAttributes> for MrmlParser<'a> {
    fn parse_attributes(&mut self) -> Result<MjmlAttributes, Error> {
        let mut attrs = MjmlAttributes::default();
        while let Some(token) = self.next_attribute()? {
            match token.local.as_str() {
                "owa" => attrs.owa = Some(token.value.to_string()),
                "lang" => attrs.lang = Some(token.value.to_string()),
                "dir" => attrs.dir = Some(token.value.to_string()),
                _ => return Err(Error::UnexpectedAttribute(token.span.start())),
            }
        }
        Ok(attrs)
    }
}

impl<'a> ChildrenParser<'a, MjmlChildren> for MrmlParser<'a> {
    fn parse_children(&mut self) -> Result<MjmlChildren, Error> {
        let mut children = MjmlChildren::default();

        loop {
            match self.assert_next()? {
                MrmlToken::ElementClose(close) if close.local.as_str() == super::NAME => {
                    return Ok(children);
                }
                MrmlToken::ElementStart(start) => match start.local.as_str() {
                    MJ_HEAD => {
                        children.head = Some(self.parse(start.local)?);
                    }
                    MJ_BODY => {
                        children.body = Some(self.parse(start.local)?);
                    }
                    _ => {
                        return Err(Error::UnexpectedElement(start.span.start()));
                    }
                },
                MrmlToken::Text(inner) if inner.text.trim().is_empty() => {}
                other => {
                    return Err(Error::unexpected_token(other.range()));
                }
            }
        }
    }
}

impl<'a> ElementParser<'a, Mjml> for parser::MrmlParser<'a> {
    fn parse(&mut self, _tag: StrSpan<'a>) -> Result<Mjml, Error> {
        let attributes = self.parse_attributes()?;
        let ending = self.assert_element_end()?;
        let children = if !ending.empty {
            self.parse_children()?
        } else {
            Default::default()
        };

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
        MrmlParser::new(value.as_ref(), opts).parse_root()
    }

    /// Function to parse a raw mjml template using the default parsing
    /// [options](crate::prelude::parser::ParserOptions).
    ///
    /// ```rust
    /// use mrml::mjml::Mjml;
    ///
    /// match Mjml::parse("<mjml><mj-head /><mj-body /></mjml>") {
    ///     Ok(_) => println!("Success!"),
    ///     Err(err) => eprintln!("Something went wrong: {err:?}"),
    /// }
    /// ```
    pub fn parse<T: AsRef<str>>(value: T) -> Result<Self, Error> {
        MrmlParser::new(value.as_ref(), Default::default()).parse_root()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let template = "<mjml></mjml>";
        let elt: Mjml = MrmlParser::new(template, Default::default())
            .parse_root()
            .unwrap();
        assert!(elt.children.body.is_none());
        assert!(elt.children.head.is_none());
    }

    #[test]
    fn with_lang() {
        let template = "<mjml lang=\"fr\"></mjml>";
        let elt = Mjml::parse(template).unwrap();
        assert_eq!(elt.attributes.lang.unwrap(), "fr");
    }

    #[test]
    fn with_owa() {
        let template = "<mjml owa=\"desktop\"></mjml>";
        let elt = Mjml::parse(template).unwrap();
        assert_eq!(elt.attributes.owa.unwrap(), "desktop");
    }
}
