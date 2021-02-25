use crate::elements::error::Error as RendererError;
use crate::parser::Error as ParserError;

#[derive(Debug)]
pub enum Error {
    Renderer(RendererError),
    Parser(ParserError),
}

impl Error {
    pub fn is_parser(&self) -> bool {
        matches!(self, Error::Renderer(_))
    }

    pub fn is_renderer(&self) -> bool {
        matches!(self, Error::Renderer(_))
    }
}

impl From<RendererError> for Error {
    fn from(error: RendererError) -> Error {
        Error::Renderer(error)
    }
}

impl From<ParserError> for Error {
    fn from(error: ParserError) -> Error {
        Error::Parser(error)
    }
}
