use super::MJBreakpoint;
use crate::elements::error::Error;
use crate::parser::Node;
use crate::util::size::Size;

impl MJBreakpoint {
    pub fn parse(node: &Node) -> Result<Self, Error> {
        let value = node
            .attributes
            .iter()
            .find(|(key, _value)| key.as_str() == "width")
            .and_then(|(_key, value)| value.as_str().parse::<Size>().ok());
        Ok(Self { value })
    }
}
