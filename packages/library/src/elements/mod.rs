use crate::Options;
use xmlparser::Tokenizer;

pub mod body;
pub mod error;
pub mod head;
mod mjml;
pub mod prelude;

use error::Error;
use prelude::Component;

pub use mjml::MJMLElement;

pub fn parse(tokenizer: &mut Tokenizer, opts: Options) -> Result<mjml::MJMLElement, Error> {
    mjml::MJMLElement::parse(tokenizer, opts)
}
