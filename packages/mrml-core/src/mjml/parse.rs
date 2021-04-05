use super::MJML;
use crate::mj_body::{MJBody, NAME as MJ_BODY};
use crate::mj_head::{MJHead, NAME as MJ_HEAD};
use crate::prelude::parse::{is_element_start, next_token, Error, Parsable, Parser};
use xmlparser::{StrSpan, Tokenizer};

#[derive(Debug, Default)]
struct MJMLParser(MJML);

impl Parser for MJMLParser {
    type Output = MJML;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(self.0)
    }

    fn parse_child_element<'a>(
        &mut self,
        tag: StrSpan<'a>,
        tokenizer: &mut Tokenizer<'a>,
    ) -> Result<(), Error> {
        match tag.as_str() {
            MJ_BODY => {
                let elt = MJBody::parse(tag, tokenizer)?;
                self.0.body = Some(elt);
            }
            MJ_HEAD => {
                let elt = MJHead::parse(tag, tokenizer)?;
                self.0.head = Some(elt);
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
        assert!(elt.body.is_none());
        assert!(elt.head.is_none());
    }
}
