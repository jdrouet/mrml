use crate::parser::Error as ParserError;
use crate::Options;
use xmlparser::Tokenizer;

pub mod body;
pub mod error;
pub mod head;
pub mod mjml;
pub mod prelude;

pub fn parse(tokenizer: &mut Tokenizer, opts: &Options) -> Result<mjml::MJMLElement, ParserError> {
    mjml::MJMLElement::parse(tokenizer, opts)
}
