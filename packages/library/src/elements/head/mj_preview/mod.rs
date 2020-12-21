mod parser;

use super::prelude::*;
use crate::util::header::Header;

#[derive(Clone, Debug, Default)]
pub struct MJPreview {
    pub content: String,
}

impl HeadComponent for MJPreview {
    fn update_header(&self, header: &mut Header) {
        header.set_preview(self.content.clone());
    }
}
