use super::prelude::*;
use crate::elements::error::Error;
use crate::parser::Node;
use crate::util::header::Header;
use crate::util::size::Size;

#[derive(Clone, Debug)]
pub struct MJBreakpoint {
    value: Option<Size>,
}

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

impl HeadComponent for MJBreakpoint {
    fn update_header(&self, header: &mut Header) {
        if let Some(value) = self.value.as_ref() {
            header.set_breakpoint(value.clone());
        }
    }
}
