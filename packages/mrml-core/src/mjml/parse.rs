use std::convert::TryFrom;

use xmlparser::{StrSpan, Tokenizer};

use super::{Mjml, MjmlAttributes, MjmlChildren};
use crate::mj_body::{MjBody, NAME as MJ_BODY};
use crate::mj_head::{MjHead, NAME as MJ_HEAD};
use crate::prelude::parser::{
    self, AttributesParser, ChildrenParser, ElementEnd, ElementParser, MrmlParser, MrmlToken,
};
use crate::prelude::parser::{into_element_start, next_token, Error, Parsable, Parser};

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
            match self.assert_iterate().and_then(MrmlToken::try_from)? {
                MrmlToken::ElementClose(close) if close.local.as_str() == super::NAME => {
                    return Ok(children);
                }
                MrmlToken::ElementStart(start) if start.local.as_str() == MJ_HEAD => {
                    children.head = Some(self.parse(start.local)?);
                }
                // MrmlToken::ElementStart(start) if start.local.as_str() == MJ_BODY => {
                //     children.body = Some(self.parse(start.local)?);
                // }
                other => return Err(Error::unexpected_token(other.range())),
            }
        }
    }
}

impl<'a> ElementParser<'a, Mjml> for parser::MrmlParser<'a> {
    fn parse(&mut self, tag: StrSpan<'a>) -> Result<Mjml, Error> {
        let attributes = self.parse_attributes()?;
        let ending = self.assert_next_as::<ElementEnd>()?;
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

#[derive(Debug)]
struct MjmlParser {
    opts: std::rc::Rc<crate::prelude::parser::ParserOptions>,
    element: Mjml,
}

impl MjmlParser {
    fn new(opts: std::rc::Rc<crate::prelude::parser::ParserOptions>) -> Self {
        Self {
            opts,
            element: Default::default(),
        }
    }
}

impl Parser for MjmlParser {
    type Output = Mjml;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(self.element)
    }

    fn parse_attribute<'a>(&mut self, name: StrSpan<'a>, value: StrSpan<'a>) -> Result<(), Error> {
        match name.as_str() {
            "dir" => self.element.attributes.dir = Some(value.to_string()),
            "lang" => self.element.attributes.lang = Some(value.to_string()),
            "owa" => self.element.attributes.owa = Some(value.to_string()),
            _ => return Err(Error::UnexpectedAttribute(name.start())),
        };
        Ok(())
    }

    fn parse_child_element<'a>(
        &mut self,
        tag: StrSpan<'a>,
        tokenizer: &mut Tokenizer<'a>,
    ) -> Result<(), Error> {
        match tag.as_str() {
            MJ_BODY => {
                let elt = MjBody::parse(tag, tokenizer, self.opts.clone())?;
                self.element.children.body = Some(elt);
            }
            MJ_HEAD => {
                let elt = MjHead::parse(tag, tokenizer, self.opts.clone())?;
                self.element.children.head = Some(elt);
            }
            _ => return Err(Error::UnexpectedElement(tag.start())),
        };
        Ok(())
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
    /// use std::rc::Rc;
    ///
    /// let options = Rc::new(ParserOptions {
    ///     include_loader: Box::new(MemoryIncludeLoader::default()),
    /// });
    /// match Mjml::parse_with_options("<mjml><mj-head /><mj-body /></mjml>", options) {
    ///     Ok(_) => println!("Success!"),
    ///     Err(err) => eprintln!("Something went wrong: {err:?}"),
    /// }
    /// ```
    pub fn parse_with_options<T: AsRef<str>>(
        value: T,
        opts: std::rc::Rc<crate::prelude::parser::ParserOptions>,
    ) -> Result<Self, Error> {
        let mut tokenizer = Tokenizer::from(value.as_ref());
        let token = next_token(&mut tokenizer)?;
        // TODO make sure that it's mjml
        let _start = into_element_start(&token)?;
        MjmlParser::new(opts).parse(&mut tokenizer)?.build()
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
        let opts = std::rc::Rc::new(crate::prelude::parser::ParserOptions::default());
        Self::parse_with_options(value, opts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let template = "<mjml></mjml>";
        let elt = Mjml::parse(template).unwrap();
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
