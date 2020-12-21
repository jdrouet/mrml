mod parser;

use super::prelude::*;
use crate::util::header::Header;

#[derive(Clone, Debug)]
pub struct MJFont {
    name: String,
    href: String,
}

impl HeadComponent for MJFont {
    fn update_header(&self, header: &mut Header) {
        header.register_font(self.name.as_str(), self.href.as_str());
    }
}
