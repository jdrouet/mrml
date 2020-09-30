#[derive(Debug)]
pub enum Error {
    InvalidChild,
    MissingAttribute(String),
    ParseError(String),
    UnexpectedText,
    UnexpectedElement(String),
    UnexpectedAttribute(String),
}
