use super::prelude::*;
use crate::mjml::error::Error;
use crate::util::{Header, Size};
use crate::Options;
use roxmltree::Node;

#[derive(Clone, Debug)]
pub struct MJBreakpoint {
    value: Size,
}

impl MJBreakpoint {
    pub fn parse<'a, 'b>(node: &Node<'a, 'b>, opts: &Options) -> Result<Self, Error> {
        let value = match node
            .attribute("width")
            .and_then(|attr| attr.parse::<Size>().ok())
        {
            Some(value) => value,
            None => opts.breakpoint.clone(),
        };
        Ok(Self { value })
    }
}

impl HeadComponent for MJBreakpoint {
    fn update_header(&self, header: &mut Header) {
        header.set_breakpoint(self.value.clone());
    }
}

#[cfg(test)]
pub mod tests {
    use crate::tests::compare_render;

    #[test]
    fn base() {
        compare_render(
            include_str!("../../../test/mj-breakpoint.mjml"),
            include_str!("../../../test/mj-breakpoint.html"),
        );
    }
}
