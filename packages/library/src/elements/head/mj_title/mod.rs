mod parser;

use super::prelude::*;
use crate::util::header::Header;

#[derive(Clone, Debug)]
pub struct MJTitle {
    content: String,
}

impl MJTitle {
    pub fn get_content(&self) -> String {
        self.content.clone()
    }
}

impl HeadComponent for MJTitle {
    fn update_header(&self, header: &mut Header) {
        header.set_title(self.get_content());
    }
}
