use super::MJAttributes;
use crate::elements::head::prelude::*;
use crate::util::header::Header;

impl HeadComponent for MJAttributes {
    fn update_header(&self, header: &mut Header) {
        header.set_default_attributes(self.0.clone());
    }
}
