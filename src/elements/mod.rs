use crate::parser::Node;
use crate::Options;

pub mod body;
pub mod error;
pub mod head;
mod mjml;
pub mod prelude;

use error::Error;
use prelude::Component;

pub use mjml::MJMLElement;

pub fn parse<'a>(node: &Node<'a>, opts: Options) -> Result<mjml::MJMLElement<'a>, Error> {
    mjml::MJMLElement::parse(node, opts)
}
