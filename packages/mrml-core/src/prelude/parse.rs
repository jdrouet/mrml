use xmlparser::{StrSpan, Token, Tokenizer};

#[macro_export]
macro_rules! parse_attribute {
    () => {
        fn parse_attribute<'a>(
            &mut self,
            name: xmlparser::StrSpan<'a>,
            value: xmlparser::StrSpan<'a>,
        ) -> Result<(), Error> {
            self.0
                .attributes
                .insert(name.to_string(), value.to_string());
            Ok(())
        }
    };
}

#[macro_export]
macro_rules! parse_child {
    ($child_parser:ident) => {
        fn parse_child_element<'a>(
            &mut self,
            tag: xmlparser::StrSpan<'a>,
            tokenizer: &mut xmlparser::Tokenizer<'a>,
        ) -> Result<(), Error> {
            self.0.children.push($child_parser::parse(tag, tokenizer)?);
            Ok(())
        }
    };
}

#[macro_export]
macro_rules! parse_comment {
    () => {
        fn parse_child_comment(&mut self, value: xmlparser::StrSpan) -> Result<(), Error> {
            self.0
                .children
                .push($crate::comment::Comment::from(value.as_str()).into());
            Ok(())
        }
    };
}

#[macro_export]
macro_rules! parse_text {
    () => {
        fn parse_child_text(&mut self, value: xmlparser::StrSpan) -> Result<(), Error> {
            self.0
                .children
                .push($crate::text::Text::from(value.as_str()).into());
            Ok(())
        }
    };
}

#[derive(Debug)]
pub enum Error {
    UnexpectedAttribute(usize),
    UnexpectedElement(usize),
    UnexpectedComment(usize),
    UnexpectedText(usize),
    InvalidElement(String),
    InvalidFormat,
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

impl ToString for Error {
    fn to_string(&self) -> String {
        match self {
            Self::UnexpectedAttribute(position) => {
                format!("unexpected attribute at position {}", position)
            }
            Self::UnexpectedElement(position) => {
                format!("unexpected element at position {}", position)
            }
            Self::UnexpectedComment(position) => {
                format!("unexpected comment at position {}", position)
            }
            Self::UnexpectedText(position) => format!("unexpected text at position {}", position),
            Self::InvalidElement(elt) => format!("invalid element: {}", elt),
            Self::InvalidFormat => "invalid format".to_string(),
            Self::SizeLimit => "size limit reached".to_string(),
            Self::ParserError(inner) => format!("parsing error: {}", inner),
            Self::NoRootNode => "no root not found".to_string(),
        }
    }
}

pub(crate) fn next_token<'a>(tokenizer: &mut Tokenizer<'a>) -> Result<Token<'a>, Error> {
    if let Some(token) = tokenizer.next() {
        Ok(token?)
    } else {
        Err(Error::InvalidFormat)
    }
}

pub(crate) fn is_element_start(token: &Token) -> bool {
    matches!(
        token,
        Token::ElementStart {
            prefix: _,
            local: _,
            span: _,
        }
    )
}

pub trait Parser: Sized {
    type Output;

    fn build(self) -> Result<Self::Output, Error>;

    /// for elements like <br> or <meta>, they have no closing elements and
    /// the parser should not check for children
    fn should_ignore_children(&self) -> bool {
        false
    }

    fn parse_attribute<'a>(&mut self, name: StrSpan<'a>, _value: StrSpan<'a>) -> Result<(), Error> {
        Err(Error::UnexpectedAttribute(name.start()))
    }

    fn parse_children<'a>(&mut self, tokenizer: &mut Tokenizer<'a>) -> Result<(), Error> {
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
                _ => return Err(Error::InvalidFormat),
            };
        }
    }

    fn parse_child_element<'a>(
        &mut self,
        tag: StrSpan<'a>,
        _tokenizer: &mut Tokenizer<'a>,
    ) -> Result<(), Error> {
        Err(Error::UnexpectedElement(tag.start()))
    }

    fn parse_child_comment(&mut self, value: StrSpan) -> Result<(), Error> {
        Err(Error::UnexpectedComment(value.start()))
    }

    fn parse_child_text(&mut self, value: StrSpan) -> Result<(), Error> {
        Err(Error::UnexpectedText(value.start()))
    }

    fn parse(mut self, tokenizer: &mut Tokenizer) -> Result<Self, Error> {
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
                            if !self.should_ignore_children() {
                                self.parse_children(tokenizer)?;
                            }
                            return Ok(self);
                        }
                        // unexpected
                        _ => return Err(Error::InvalidFormat),
                    }
                }
                _ => {
                    return Err(Error::InvalidFormat);
                }
            };
        }
    }
}

pub trait Parsable: Sized {
    fn parse<'a>(tag: StrSpan<'a>, tokenizer: &mut Tokenizer<'a>) -> Result<Self, Error>;
}
