use std::rc::Rc;

use xmlparser::{StrSpan, Tokenizer};

use super::{MjStyle, MjStyleAttributes};
use crate::prelude::parse::{Error, Parsable, Parser, ParserOptions};

#[derive(Debug)]
struct MjStyleParser {
    attributes: MjStyleAttributes,
    children: String,
}

impl MjStyleParser {
    fn new(_opts: Rc<ParserOptions>) -> Self {
        Self {
            attributes: Default::default(),
            children: Default::default(),
        }
    }
}

impl Parser for MjStyleParser {
    type Output = MjStyle;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(MjStyle {
            attributes: self.attributes,
            children: self.children,
        })
    }

    fn parse_attribute<'a>(&mut self, name: StrSpan<'a>, value: StrSpan<'a>) -> Result<(), Error> {
        if name.as_str() == "inline" {
            self.attributes.inline = Some(value.to_string());
            Ok(())
        } else {
            Err(Error::UnexpectedAttribute(name.start()))
        }
    }

    fn parse_child_text(&mut self, value: StrSpan) -> Result<(), Error> {
        self.children = value.to_string();
        Ok(())
    }
}

impl Parsable for MjStyle {
    fn parse(
        _tag: StrSpan,
        tokenizer: &mut Tokenizer,
        opts: Rc<ParserOptions>,
    ) -> Result<Self, Error> {
        MjStyleParser::new(opts).parse(tokenizer)?.build()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn success() {
        let res = crate::mjml::Mjml::parse(
            r#"<mjml><mj-head><mj-style>.whatever {background-color: red};</mj-style></mj-head></mjml>"#,
        );
        assert!(res.is_ok());
    }
}
