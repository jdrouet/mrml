use crate::elements::error::Error as MJMLError;
use crate::parser::Error as ParserError;

#[derive(Debug)]
pub enum Error {
    MJMLError(MJMLError),
    ParserError(ParserError),
}

impl Error {
    pub fn is_mjml_error(&self) -> bool {
        matches!(self, Error::MJMLError(_))
    }
}

impl From<MJMLError> for Error {
    fn from(error: MJMLError) -> Error {
        Error::MJMLError(error)
    }
}

impl From<ParserError> for Error {
    fn from(error: ParserError) -> Error {
        Error::ParserError(error)
    }
}
