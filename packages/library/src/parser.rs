use crate::elements::error::Error as ElementError;
use xmlparser::{StrSpan, Token, Tokenizer};

pub type Attributes<'a> = Vec<(StrSpan<'a>, StrSpan<'a>)>;
pub type Children<'a> = Vec<Element<'a>>;

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

pub struct Node<'a> {
    pub name: StrSpan<'a>,
    pub attributes: Attributes<'a>,
    pub children: Children<'a>,
}

impl<'a> Node<'a> {
    pub fn new(name: StrSpan<'a>, attributes: Attributes<'a>, children: Children<'a>) -> Self {
        Node {
            name,
            attributes,
            children,
        }
    }

    fn parse(parser: &mut Tokenizer<'a>, tag: StrSpan<'a>) -> Result<Self, Error> {
        let mut position = tag.end();
        let mut attributes = vec![];
        loop {
            let token = match parser.next() {
                Some(value) => value,
                // end before having the closing element
                None => return Err(Error::InvalidFormat { position }),
            };
            let token = token?;
            match token {
                Token::Attribute {
                    local,
                    value,
                    span,
                    prefix: _,
                } => {
                    position = span.end();
                    attributes.push((local, value));
                }
                Token::ElementEnd { end, span } => {
                    position = span.end();
                    match end {
                        xmlparser::ElementEnd::Empty => {
                            return Ok(Node::new(tag, attributes, vec![]));
                        }
                        xmlparser::ElementEnd::Open => {
                            return Ok(Node::new(
                                tag,
                                attributes,
                                Element::parse_children(parser, tag)?,
                            ));
                        }
                        // unexpected
                        _ => return Err(Error::InvalidFormat { position }),
                    }
                }
                _ => return Err(Error::InvalidFormat { position }),
            };
        }
    }

    pub fn parse_root(parser: &mut Tokenizer<'a>) -> Result<Self, Error> {
        let token = match parser.next() {
            Some(value) => value,
            None => return Err(Error::NoRootNode),
        };
        let token = token?;
        match token {
            Token::ElementStart {
                local,
                prefix: _,
                span: _,
            } => Node::parse(parser, local),
            _ => Err(Error::NoRootNode),
        }
    }
}

pub enum Element<'a> {
    Comment(StrSpan<'a>),
    Node(Node<'a>),
    Text(StrSpan<'a>),
}

impl<'a> Element<'a> {
    pub fn is_comment(&self) -> bool {
        matches!(self, Element::Comment(_))
    }
    pub fn as_comment(&self) -> Option<&StrSpan<'a>> {
        match self {
            Element::Comment(value) => Some(value),
            _ => None,
        }
    }
    pub fn is_node(&self) -> bool {
        matches!(self, Element::Node(_))
    }
    pub fn as_node(&self) -> Option<&Node<'a>> {
        match self {
            Element::Node(value) => Some(value),
            _ => None,
        }
    }
    pub fn is_text(&self) -> bool {
        matches!(self, Element::Text(_))
    }
    pub fn as_text(&self) -> Option<&StrSpan<'a>> {
        match self {
            Element::Text(value) => Some(value),
            _ => None,
        }
    }
    fn parse_children(parser: &mut Tokenizer<'a>, tag: StrSpan<'a>) -> Result<Vec<Self>, Error> {
        let mut position = tag.end();
        let mut children: Vec<Element<'a>> = vec![];
        loop {
            let token = match parser.next() {
                Some(value) => value,
                // end before having the closing element
                None => return Err(Error::InvalidFormat { position }),
            };
            let token = token?;
            match token {
                Token::ElementStart {
                    local,
                    prefix: _,
                    span,
                } => {
                    position = span.end();
                    children.push(Element::Node(Node::parse(parser, local)?));
                }
                Token::Text { text } => {
                    position = text.end();
                    if !text.as_str().trim().is_empty() {
                        children.push(Element::Text(text));
                    }
                }
                Token::ElementEnd { end, span } => {
                    position = span.end();
                    match end {
                        xmlparser::ElementEnd::Close(_prefix, local) => {
                            if local.as_str() == tag.as_str() {
                                return Ok(children);
                            }
                            // end before having the closing element
                            return Err(Error::InvalidFormat { position });
                        }
                        _ => return Err(Error::InvalidFormat { position }),
                    }
                }
                // TODO handle comments
                Token::Comment { text, span } => {
                    position = span.end();
                    children.push(Element::Comment(text));
                }
                _ => return Err(Error::InvalidFormat { position }),
            };
        }
    }
}

/// parse the mjml template into an mjml object
///
/// ```rust
/// let result = mrml::parse(r#"
///     <mjml>
///         <mj-head>
///             <mj-title>Something</mj-title>
///         </mj-head>
///     </mjml>
/// "#, mrml::Options::default());
/// assert!(result.is_ok());
/// ```
///
/// ```rust
/// let result = mrml::parse("<mjml", mrml::Options::default());
/// assert!(result.is_err());
/// ```
pub fn parse(text: &'_ str) -> Result<Node<'_>, Error> {
    let mut parser = Tokenizer::from(text);
    Node::parse_root(&mut parser)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_root() {
        let root = parse("<mjml><mj-head></mj-head><mj-body /></mjml>");
        let root = root.unwrap();
        assert_eq!(root.children.len(), 2);
    }

    #[test]
    fn parse_with_weird_text() {
        let root = parse("<mjml><mj-body>&copy;</mj-body></mjml>");
        assert!(root.is_ok());
    }

    #[test]
    fn parse_with_html() {
        let root = parse("<mjml><mj-body><a href=\"toto\">yolo</a></mj-body></mjml>");
        assert!(root.is_ok());
    }

    fn parse_with_invalid_format(template: &str, pos: usize) {
        let root = parse(template);
        if let Err(error) = root {
            println!("error: {:?}", error);
            if let Error::InvalidFormat { position } = error {
                assert_eq!(position, pos);
            } else {
                panic!();
            }
        } else {
            panic!();
        }
    }

    #[test]
    fn parse_with_error() {
        parse_with_invalid_format("<mjml", 5);
        parse_with_invalid_format("<mjml attr=\"val\"><toto></mjml>", 30);
        parse_with_invalid_format("<mjml attr=\"val\"></toto></mjml>", 24);
    }

    #[test]
    fn parse_with_comment() {
        let root = parse("<mjml><mj-body><!--<a href=\"toto\">yolo</a>--></mj-body></mjml>");
        assert!(root.is_ok());
        let root = root.unwrap();
        if let Element::Node(node) = root.children.get(0).as_ref().unwrap() {
            assert_eq!(node.children.len(), 1);
        } else {
            panic!("should have node");
        }
    }
}
