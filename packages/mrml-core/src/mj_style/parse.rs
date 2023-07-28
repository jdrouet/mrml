use std::rc::Rc;

use xmlparser::{StrSpan, Tokenizer};

use super::{MjStyle, MjStyleAttributes};
use crate::prelude::parser::{
    AttributesParser, ElementParser, Error, MrmlParser, Parsable, Parser, ParserOptions,
};

impl<'a> AttributesParser<'a, MjStyleAttributes> for MrmlParser<'a> {
    fn parse_attributes(&mut self) -> Result<MjStyleAttributes, Error> {
        let mut result = MjStyleAttributes::default();
        while let Some(attr) = self.next_attribute()? {
            if attr.local.as_str() == "inline" {
                result.inline = Some(attr.value.to_string());
            } else {
                return Err(Error::UnexpectedAttribute(attr.span.start()));
            }
        }
        Ok(result)
    }
}

impl<'a> ElementParser<'a, MjStyle> for MrmlParser<'a> {
    fn parse(&mut self, _tag: StrSpan<'a>) -> Result<MjStyle, Error> {
        let attributes = self.parse_attributes()?;
        let ending = self.next_element_end()?.ok_or(Error::EndOfStream)?;
        if !ending.empty {
            let children = self
                .next_text()?
                .map(|txt| txt.text.to_string())
                .unwrap_or_default();
            self.assert_element_close()?;

            Ok(MjStyle {
                attributes,
                children,
            })
        } else {
            Ok(MjStyle {
                attributes,
                children: String::new(),
            })
        }
    }
}

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
