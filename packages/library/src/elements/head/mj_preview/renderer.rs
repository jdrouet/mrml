use super::MJPreview;
use crate::elements::head::prelude::*;
use crate::util::header::Header;

impl HeadComponent for MJPreview {
    fn update_header(&self, header: &mut Header) {
        header.set_preview(self.content.clone());
    }
}
