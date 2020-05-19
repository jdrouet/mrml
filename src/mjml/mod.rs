use crate::util::Context;
use roxmltree::Node;

pub mod body;
pub mod error;
pub mod head;
mod mjml;
pub mod prelude;

use error::Error;
use prelude::Component;

pub use mjml::MJMLElement;

pub fn parse<'a, 'b>(node: Node<'a, 'b>, ctx: Context) -> Result<mjml::MJMLElement, Error> {
    let mut element = mjml::MJMLElement::parse(node)?;
    element.set_context(ctx);
    Ok(element)
}
