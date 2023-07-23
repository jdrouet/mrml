use super::Mjml;
use crate::mj_body::{MjBody, NAME as MJ_BODY};
use crate::mj_head::{MjHead, NAME as MJ_HEAD};
use crate::prelude::parse::{is_element_start, next_token, Error, Parsable, Parser};
use xmlparser::{StrSpan, Tokenizer};

#[derive(Debug)]
struct MjmlParser {
    opts: std::rc::Rc<crate::prelude::parse::ParserOptions>,
    element: Mjml,
}

impl MjmlParser {
    fn new(opts: std::rc::Rc<crate::prelude::parse::ParserOptions>) -> Self {
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
    /// Function to parse a raw mjml template with some parsing [options](crate::prelude::parse::ParserOptions).
    ///
    /// You can specify the kind of loader mrml needs to use for loading the content of
    /// [`mj-include`](crate::mj_include) elements.
    ///
    /// You can take a look at the available loaders [here](crate::prelude::parse).
    ///
    /// ```rust
    /// use mrml::mjml::Mjml;
    /// use mrml::prelude::parse::ParserOptions;
    /// use mrml::prelude::parse::memory_loader::MemoryIncludeLoader;
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
        opts: std::rc::Rc<crate::prelude::parse::ParserOptions>,
    ) -> Result<Self, Error> {
        let mut tokenizer = Tokenizer::from(value.as_ref());
        let token = next_token(&mut tokenizer)?;
        if is_element_start(&token).is_some() {
            MjmlParser::new(opts).parse(&mut tokenizer)?.build()
        } else {
            Err(Error::InvalidFormat)
        }
    }

    /// Function to parse a raw mjml template using the default parsing [options](crate::prelude::parse::ParserOptions).
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
        let opts = std::rc::Rc::new(crate::prelude::parse::ParserOptions::default());
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
