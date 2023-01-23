use super::MJML;
use crate::mj_body::{MjBody, NAME as MJ_BODY};
use crate::mj_head::{MjHead, NAME as MJ_HEAD};
use crate::prelude::parse::{is_element_start, next_token, Error, Parsable, Parser};
use xmlparser::{StrSpan, Tokenizer};

#[derive(Debug, Default)]
struct MJMLParser(MJML);

impl Parser for MJMLParser {
    type Output = MJML;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(self.0)
    }

    fn parse_attribute<'a>(&mut self, name: StrSpan<'a>, value: StrSpan<'a>) -> Result<(), Error> {
        match name.as_str() {
            "lang" | "owa" => self
                .0
                .attributes
                .insert(name.to_string(), value.to_string()),
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
                let elt = MjBody::parse(tag, tokenizer)?;
                self.0.children.body = Some(elt);
            }
            MJ_HEAD => {
                let elt = MjHead::parse(tag, tokenizer)?;
                self.0.children.head = Some(elt);
            }
            _ => return Err(Error::UnexpectedElement(tag.start())),
        };
        Ok(())
    }
}

impl MJML {
    pub fn parse<T: AsRef<str>>(value: T) -> Result<Self, Error> {
        let mut tokenizer = Tokenizer::from(value.as_ref());
        let token = next_token(&mut tokenizer)?;
        if is_element_start(&token) {
            MJMLParser::default().parse(&mut tokenizer)?.build()
        } else {
            Err(Error::InvalidFormat)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let template = "<mjml></mjml>";
        let elt = MJML::parse(template).unwrap();
        assert!(elt.children.body.is_none());
        assert!(elt.children.head.is_none());
    }

    #[test]
    fn with_lang() {
        let template = "<mjml lang=\"fr\"></mjml>";
        let elt = MJML::parse(template).unwrap();
        assert_eq!(elt.attributes.get("lang"), Some(&"fr".to_string()));
    }

    #[test]
    fn with_owa() {
        let template = "<mjml owa=\"desktop\"></mjml>";
        let elt = MJML::parse(template).unwrap();
        assert_eq!(elt.attributes.get("owa"), Some(&"desktop".to_string()));
    }
}
