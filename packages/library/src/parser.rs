use crate::elements::error::Error as ElementError;
use xmlparser::{StrSpan, Token, Tokenizer};

#[derive(Debug)]
pub enum Error {
    InvalidFormat {
        position: usize,
    },
    /// The input string should be smaller than 4GiB.
    SizeLimit,
    /// Errors detected by the `xmlparser` crate.
    ParserError(xmlparser::Error),
    /// The MJML document must have at least one element.
    NoRootNode,
}

impl From<xmlparser::Error> for Error {
    fn from(err: xmlparser::Error) -> Self {
        Error::ParserError(err)
    }
}

pub fn next_token<'a>(tokenizer: &mut Tokenizer<'a>) -> Result<Token<'a>, ElementError> {
    if let Some(token) = tokenizer.next() {
        token.map_err(|err| ElementError::ParseError(err.to_string()))
    } else {
        Err(ElementError::ParseError("no more token found".into()))
    }
}

pub trait MJMLParser: Sized {
    type Output;

    fn build(self) -> Result<Self::Output, ElementError>;

    fn parse_attribute<'a>(
        &mut self,
        name: StrSpan<'a>,
        _value: StrSpan<'a>,
    ) -> Result<(), ElementError> {
        Err(ElementError::UnexpectedAttribute(name.to_string()))
    }

    fn parse_children<'a>(&mut self, tokenizer: &mut Tokenizer<'a>) -> Result<(), ElementError> {
        loop {
            let token = next_token(tokenizer)?;
            match token {
                Token::Comment { text, span: _ } => {
                    self.parse_child_comment(text)?;
                }
                Token::Text { text } => {
                    if !text.trim().is_empty() {
                        self.parse_child_text(text)?;
                    }
                }
                Token::ElementStart {
                    prefix: _,
                    local,
                    span: _,
                } => {
                    self.parse_child_element(local, tokenizer)?;
                }
                Token::ElementEnd { end: _, span: _ } => return Ok(()),
                _ => return Err(ElementError::ParseError("unexpected value".into())),
            };
        }
    }

    fn parse_child_element<'a>(
        &mut self,
        tag: StrSpan<'a>,
        _tokenizer: &mut Tokenizer<'a>,
    ) -> Result<(), ElementError> {
        Err(ElementError::UnexpectedElement(tag.to_string()))
    }

    fn parse_child_comment(&mut self, _value: StrSpan) -> Result<(), ElementError> {
        Err(ElementError::UnexpectedComment)
    }

    fn parse_child_text(&mut self, _value: StrSpan) -> Result<(), ElementError> {
        Err(ElementError::UnexpectedText)
    }

    fn parse(mut self, tokenizer: &mut Tokenizer) -> Result<Self, ElementError> {
        loop {
            let token = next_token(tokenizer)?;
            match token {
                Token::Attribute {
                    prefix: _,
                    local,
                    value,
                    span: _,
                } => {
                    self.parse_attribute(local, value)?;
                }
                Token::ElementEnd { end, span: _ } => {
                    match end {
                        xmlparser::ElementEnd::Empty => {
                            return Ok(self);
                        }
                        xmlparser::ElementEnd::Open => {
                            self.parse_children(tokenizer)?;
                            return Ok(self);
                        }
                        // unexpected
                        _ => return Err(ElementError::ParseError("invalid element".into())),
                    }
                }
                _ => {
                    return Err(ElementError::ParseError("unexpected token".into()));
                }
            };
        }
    }
}
