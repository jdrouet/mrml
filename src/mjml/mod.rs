use crate::Options;
use roxmltree::Node;

pub mod body;
pub mod error;
pub mod head;
mod mjml;
pub mod prelude;

use error::Error;
use prelude::Component;

pub use mjml::MJMLElement;

pub fn parse<'a, 'b>(node: Node<'a, 'b>, opts: &Options) -> Result<mjml::MJMLElement, Error> {
    mjml::MJMLElement::parse(node, opts)
}
