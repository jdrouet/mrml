use super::body::mj_body::{MJBody, NAME as MJ_BODY};
use super::head::mj_head::{MJHead, NAME as MJ_HEAD};
use crate::elements::error::Error;
use crate::elements::prelude::Component;
use crate::parser::{next_token, Error as ParserError, MJMLParser};
use crate::util::context::Context;
use crate::Options;
use log::debug;
use xmlparser::{StrSpan, Token, Tokenizer};

#[derive(Debug)]
pub struct MJMLElement {
    context: Option<Context>,
    head: MJHead,
    body: MJBody,
}

struct MJMLElementParser<'o> {
    options: &'o Options,
    context: Option<Context>,
    head: MJHead,
    body: Option<MJBody>,
}

impl<'o> MJMLElementParser<'o> {
    pub fn new(options: &'o Options) -> Self {
        Self {
            options,
            context: None,
            head: MJHead::empty(options),
            body: None,
        }
    }
}

impl<'o> MJMLParser for MJMLElementParser<'o> {
    type Output = MJMLElement;

    fn build(mut self) -> Result<Self::Output, ParserError> {
        let mut body = self.body.unwrap_or_else(MJBody::empty);
        body.set_context(Context::default());
        body.update_header(self.head.get_mut_header());
        Ok(MJMLElement {
            context: self.context,
            head: self.head,
            body,
        })
    }

    fn parse_child_comment(&mut self, _value: StrSpan) -> Result<(), ParserError> {
        log::warn!("comment ignored in mjml root element");
        Ok(())
    }

    fn parse_child_element<'a>(
        &mut self,
        tag: StrSpan<'a>,
        tokenizer: &mut Tokenizer,
    ) -> Result<(), ParserError> {
        match tag.as_str() {
            MJ_HEAD => {
                self.head = MJHead::parse(tokenizer, self.options)?;
            }
            MJ_BODY => {
                self.body = Some(MJBody::parse(tokenizer, self.head.get_header())?);
            }
            _ => return Err(ParserError::UnexpectedElement(tag.start())),
        };
        Ok(())
    }
}

impl<'a> MJMLElement {
    pub fn parse(
        tokenizer: &mut Tokenizer<'a>,
        opts: &Options,
    ) -> Result<MJMLElement, ParserError> {
        MJMLElementParser::new(opts).parse(tokenizer)?.build()
    }

    pub fn parse_root(
        tokenizer: &mut Tokenizer<'a>,
        opts: &Options,
    ) -> Result<MJMLElement, ParserError> {
        let token = next_token(tokenizer)?;
        match token {
            Token::ElementStart {
                prefix: _,
                local,
                span: _,
            } => match local.as_str() {
                "mjml" => Self::parse(tokenizer, opts),
                _ => Err(ParserError::UnexpectedElement(local.start())),
            },
            _ => Err(ParserError::InvalidFormat),
        }
    }

    pub fn get_title(&self) -> String {
        debug!("get_title");
        self.head.get_title()
    }

    pub fn get_preview(&self) -> String {
        debug!("get_preview");
        self.head.get_preview()
    }

    pub fn get_html(&self) -> Result<String, Error> {
        debug!("get_html");
        let header = self.head.get_header();
        Ok(String::from("<!doctype html>")
           + "<html xmlns=\"http://www.w3.org/1999/xhtml\" xmlns:v=\"urn:schemas-microsoft-com:vml\" xmlns:o=\"urn:schemas-microsoft-com:office:office\">"
           + self.head.render(&header)?.as_str()
           + self.body.render(&header)?.as_str()
           + "</html>")
    }
}
