use super::MJAttributes;
use crate::elements::head::prelude::*;
use crate::util::header::DefaultAttributes;
use crate::util::header::Header;

impl HeadComponent for MJAttributes {
    fn update_header(&self, header: &mut Header) {
        let mut result = DefaultAttributes::default();
        self.children
            .iter()
            .for_each(|child| child.update_attributes(&mut result));
        header.set_default_attributes(result);
    }
}
