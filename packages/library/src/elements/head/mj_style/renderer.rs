use super::MJStyle;
use crate::elements::head::prelude::*;
use crate::util::header::Header;

impl HeadComponent for MJStyle {
    fn update_header(&self, header: &mut Header) {
        if self.inline {
            // TODO
        } else {
            header.add_style(self.content.clone());
        }
    }
}
