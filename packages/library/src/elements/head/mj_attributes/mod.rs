mod parser;

use super::prelude::*;
use crate::util::header::{DefaultAttributes, Header};

#[derive(Clone, Debug)]
pub struct MJAttributes(DefaultAttributes);

impl MJAttributes {
    fn new() -> Self {
        Self(DefaultAttributes::default())
    }
}

impl HeadComponent for MJAttributes {
    fn update_header(&self, header: &mut Header) {
        header.set_default_attributes(self.0.clone());
    }
}
