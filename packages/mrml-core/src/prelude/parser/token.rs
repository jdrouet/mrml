use std::fmt::Display;

use xmlparser::{StrSpan, Token};

use super::MrmlCursor;

#[derive(Clone, Copy, Debug)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.start, self.end)
    }
}

impl<'a> From<&StrSpan<'a>> for Span {
    fn from(value: &StrSpan<'a>) -> Self {
        Self {
            start: value.start(),
            end: value.end(),
        }
    }
}

impl<'a> From<StrSpan<'a>> for Span {
    fn from(value: StrSpan<'a>) -> Self {
        Self {
            start: value.start(),
            end: value.end(),
        }
    }
}

impl<'a> From<Token<'a>> for Span {
    fn from(value: Token<'a>) -> Self {
        match value {
            Token::Attribute { span, .. }
            | Token::Cdata { span, .. }
            | Token::Comment { span, .. }
            | Token::Declaration { span, .. }
            | Token::DtdEnd { span }
            | Token::DtdStart { span, .. }
            | Token::ElementEnd { span, .. }
            | Token::ElementStart { span, .. }
            | Token::EmptyDtd { span, .. }
            | Token::EntityDeclaration { span, .. }
            | Token::ProcessingInstruction { span, .. } => span.into(),
            Token::Text { text } => text.into(),
        }
    }
}

#[derive(Debug)]
pub(crate) enum MrmlToken<'a> {
    Attribute(Attribute<'a>),
    Comment(Comment<'a>),
    ElementClose(ElementClose<'a>),
    ElementEnd(ElementEnd<'a>),
    ElementStart(ElementStart<'a>),
    Text(Text<'a>),
}

impl<'a> MrmlToken<'a> {
    pub(crate) fn parse(
        cursor: &mut MrmlCursor<'a>,
        value: Token<'a>,
    ) -> Result<Self, super::Error> {
        match value {
            Token::Attribute {
                prefix,
                local,
                value,
                span,
            } => Ok(MrmlToken::Attribute(Attribute {
                prefix,
                local,
                value,
                span,
            })),
            Token::Comment { text, span } => Ok(MrmlToken::Comment(Comment { span, text })),
            Token::ElementEnd {
                end: xmlparser::ElementEnd::Close(prefix, local),
                span,
            } => Ok(MrmlToken::ElementClose(ElementClose {
                span,
                prefix,
                local,
            })),
            Token::ElementEnd {
                end: xmlparser::ElementEnd::Empty,
                span,
            } => Ok(MrmlToken::ElementEnd(ElementEnd { span, empty: true })),
            Token::ElementEnd {
                end: xmlparser::ElementEnd::Open,
                span,
            } => Ok(MrmlToken::ElementEnd(ElementEnd { span, empty: false })),
            Token::ElementStart {
                prefix,
                local,
                span,
            } => Ok(MrmlToken::ElementStart(ElementStart {
                prefix,
                local,
                span,
            })),
            Token::Text { text } => Ok(MrmlToken::Text(Text { text })),
            other => Err(super::Error::UnexpectedToken {
                origin: cursor.origin(),
                position: other.into(),
            }),
        }
    }
}

impl<'a> MrmlToken<'a> {
    pub fn span(&self) -> Span {
        match self {
            Self::Attribute(item) => item.span,
            Self::Comment(item) => item.span,
            Self::ElementClose(item) => item.span,
            Self::ElementEnd(item) => item.span,
            Self::ElementStart(item) => item.span,
            Self::Text(item) => item.text,
        }
        .into()
    }
}

#[derive(Debug)]
pub(crate) struct Attribute<'a> {
    #[allow(unused)]
    pub prefix: StrSpan<'a>,
    pub local: StrSpan<'a>,
    pub value: StrSpan<'a>,
    pub span: StrSpan<'a>,
}

#[derive(Debug)]
pub(crate) struct Comment<'a> {
    pub span: StrSpan<'a>,
    pub text: StrSpan<'a>,
}

#[derive(Debug)]
pub(crate) struct ElementClose<'a> {
    #[allow(unused)]
    pub prefix: StrSpan<'a>,
    pub local: StrSpan<'a>,
    pub span: StrSpan<'a>,
}

#[derive(Debug)]
pub(crate) struct ElementStart<'a> {
    #[allow(unused)]
    pub prefix: StrSpan<'a>,
    pub local: StrSpan<'a>,
    pub span: StrSpan<'a>,
}

#[derive(Debug)]
pub(crate) struct ElementEnd<'a> {
    pub span: StrSpan<'a>,
    pub empty: bool,
}

#[derive(Debug)]
pub(crate) struct Text<'a> {
    pub text: StrSpan<'a>,
}

impl<'a> super::MrmlCursor<'a> {
    fn read_next_token(&mut self) -> Option<Result<MrmlToken<'a>, super::Error>> {
        self.tokenizer
            .next()
            .map(|res| {
                res.map_err(|source| super::Error::ParserError {
                    origin: self.origin(),
                    source,
                })
                .and_then(|token| MrmlToken::parse(self, token))
            })
            .and_then(|token| match token {
                Ok(MrmlToken::Text(inner))
                    if inner.text.starts_with('\n') && inner.text.trim().is_empty() =>
                {
                    self.read_next_token()
                }
                other => Some(other),
            })
    }

    pub(crate) fn next_token(&mut self) -> Option<Result<MrmlToken<'a>, super::Error>> {
        if let Some(item) = self.buffer.pop() {
            Some(Ok(item))
        } else {
            self.read_next_token()
        }
    }

    pub(crate) fn rewind(&mut self, token: MrmlToken<'a>) {
        self.buffer.push(token);
    }

    pub(crate) fn assert_next(&mut self) -> Result<MrmlToken<'a>, super::Error> {
        self.next_token().unwrap_or_else(|| {
            Err(super::Error::EndOfStream {
                origin: self.origin(),
            })
        })
    }

    pub(crate) fn next_attribute(&mut self) -> Result<Option<Attribute<'a>>, super::Error> {
        match self.next_token() {
            Some(Ok(MrmlToken::Attribute(inner))) => Ok(Some(inner)),
            Some(Ok(other)) => {
                self.rewind(other);
                Ok(None)
            }
            Some(Err(inner)) => Err(inner),
            None => Err(super::Error::EndOfStream {
                origin: self.origin(),
            }),
        }
    }

    pub(crate) fn assert_element_start(&mut self) -> Result<ElementStart<'a>, super::Error> {
        match self.next_token() {
            Some(Ok(MrmlToken::ElementStart(inner))) => Ok(inner),
            Some(Ok(other)) => Err(super::Error::UnexpectedToken {
                origin: self.origin(),
                position: other.span(),
            }),
            Some(Err(inner)) => Err(inner),
            None => Err(super::Error::EndOfStream {
                origin: self.origin(),
            }),
        }
    }

    pub(crate) fn assert_element_end(&mut self) -> Result<ElementEnd<'a>, super::Error> {
        match self.next_token() {
            Some(Ok(MrmlToken::ElementEnd(inner))) => Ok(inner),
            Some(Ok(other)) => Err(super::Error::UnexpectedToken {
                origin: self.origin(),
                position: other.span(),
            }),
            Some(Err(inner)) => Err(inner),
            None => Err(super::Error::EndOfStream {
                origin: self.origin(),
            }),
        }
    }

    pub(crate) fn assert_element_close(&mut self) -> Result<ElementClose<'a>, super::Error> {
        match self.next_token() {
            Some(Ok(MrmlToken::ElementClose(inner))) => Ok(inner),
            Some(Ok(MrmlToken::Text(inner))) if inner.text.trim().is_empty() => {
                self.assert_element_close()
            }
            Some(Ok(other)) => Err(super::Error::UnexpectedToken {
                origin: self.origin(),
                position: other.span(),
            }),
            Some(Err(inner)) => Err(inner),
            None => Err(super::Error::EndOfStream {
                origin: self.origin(),
            }),
        }
    }

    pub(crate) fn next_text(&mut self) -> Result<Option<Text<'a>>, super::Error> {
        match self.next_token() {
            Some(Ok(MrmlToken::Text(inner))) => Ok(Some(inner)),
            Some(Ok(other)) => {
                self.rewind(other);
                Ok(None)
            }
            Some(Err(inner)) => Err(inner),
            None => Err(super::Error::EndOfStream {
                origin: self.origin(),
            }),
        }
    }
}
