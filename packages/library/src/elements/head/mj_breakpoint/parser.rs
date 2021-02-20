use super::MJBreakpoint;
use crate::elements::error::Error;
use crate::parser::{MJMLParser, Node};
use crate::util::size::Size;

#[derive(Default)]
struct MJBreakpointParser {
    width: Option<Size>,
}

impl MJMLParser for MJBreakpointParser {
    type Output = MJBreakpoint;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(MJBreakpoint { value: self.width })
    }

    fn parse(mut self, node: &Node) -> Result<Self, Error> {
        self.width = node
            .attributes
            .iter()
            .find(|(key, _value)| key.as_str() == "width")
            .and_then(|(_key, value)| value.as_str().parse::<Size>().ok());
        Ok(self)
    }
}

impl MJBreakpoint {
    pub fn parse(node: &Node) -> Result<Self, Error> {
        MJBreakpointParser::default().parse(node)?.build()
    }
}
