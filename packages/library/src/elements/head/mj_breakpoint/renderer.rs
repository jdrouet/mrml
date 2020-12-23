use super::MJBreakpoint;
use crate::elements::head::prelude::*;
use crate::util::header::Header;

impl HeadComponent for MJBreakpoint {
    fn update_header(&self, header: &mut Header) {
        if let Some(value) = self.value.as_ref() {
            header.set_breakpoint(value.clone());
        }
    }
}
