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

#[cfg(test)]
pub mod tests {
    use crate::tests::{compare_render, compare_render_with_options};
    use crate::util::size::Size;
    use crate::Options;

    #[test]
    fn base() {
        compare_render(
            include_str!("../../../test/mj-breakpoint.mjml"),
            include_str!("../../../test/mj-breakpoint.html"),
        );
    }

    #[test]
    fn default_options() {
        compare_render(
            include_str!("../../../test/mj-breakpoint-default.mjml"),
            include_str!("../../../test/mj-breakpoint-default.html"),
        );
    }

    #[test]
    fn with_options() {
        let mut opts = Options::default();
        opts.breakpoint = Size::Pixel(800.0);
        compare_render_with_options(
            include_str!("../../../test/mj-breakpoint-options.mjml"),
            include_str!("../../../test/mj-breakpoint-options.html"),
            opts,
        );
    }
}
