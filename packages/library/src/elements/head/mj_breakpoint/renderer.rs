use super::MJBreakpoint;
use crate::elements::head::prelude::*;
use crate::util::header::Header;

impl HeadComponent for MJBreakpoint {
    fn update_header(&self, header: &mut Header) {
        header.set_breakpoint(self.value.clone());
    }
}
