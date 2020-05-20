use crate::mjml::error::Error as MJMLError;
use roxmltree::Error as XMLError;

#[derive(Debug)]
pub enum Error {
    MJMLError(MJMLError),
    XMLError(XMLError),
}

impl Error {
    pub fn is_mjml_error(&self) -> bool {
        match self {
            Error::MJMLError(_) => true,
            _ => false,
        }
    }
}

impl From<MJMLError> for Error {
    fn from(error: MJMLError) -> Error {
        Error::MJMLError(error)
    }
}

impl From<XMLError> for Error {
    fn from(error: XMLError) -> Error {
        Error::XMLError(error)
    }
}
