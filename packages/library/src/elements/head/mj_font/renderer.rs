use super::MJFont;
use crate::elements::head::prelude::*;
use crate::util::header::Header;

impl HeadComponent for MJFont {
    fn update_header(&self, header: &mut Header) {
        header.register_font(self.name.as_str(), self.href.as_str());
    }
}
