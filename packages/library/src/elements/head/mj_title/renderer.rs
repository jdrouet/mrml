use super::MJTitle;
use crate::elements::head::prelude::*;
use crate::util::header::Header;

impl HeadComponent for MJTitle {
    fn update_header(&self, header: &mut Header) {
        header.set_title(self.get_content());
    }
}
