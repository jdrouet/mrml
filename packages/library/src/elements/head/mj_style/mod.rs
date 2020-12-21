mod parser;

use super::prelude::*;
use crate::util::header::Header;

#[derive(Clone, Debug)]
pub struct MJStyle {
    content: String,
    inline: bool,
}

impl HeadComponent for MJStyle {
    fn update_header(&self, header: &mut Header) {
        if self.inline {
            // TODO
        } else {
            header.add_style(self.content.clone());
        }
    }
}
