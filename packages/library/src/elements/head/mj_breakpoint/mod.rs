mod parser;

use super::prelude::*;
use crate::util::header::Header;
use crate::util::size::Size;

#[derive(Clone, Debug)]
pub struct MJBreakpoint {
    value: Option<Size>,
}

impl HeadComponent for MJBreakpoint {
    fn update_header(&self, header: &mut Header) {
        if let Some(value) = self.value.as_ref() {
            header.set_breakpoint(value.clone());
        }
    }
}
