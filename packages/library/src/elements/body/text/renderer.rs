use super::Text;
use crate::elements::body::mj_body::children::MJBodyChild;
use crate::elements::body::prelude::{BodyComponent, EMPTY_CHILDREN};
use crate::elements::{Component, Error};
use crate::util::attributes::Attributes;
use crate::util::context::Context;
use crate::util::header::Header;
use crate::util::size::Size;

impl Component for Text {
    fn context(&self) -> Option<&Context> {
        None
    }

    fn set_context(&mut self, _ctx: Context) {
        //
    }

    fn render(&self, _header: &Header) -> Result<String, Error> {
        Ok(self.content.clone())
    }
}

impl BodyComponent for Text {
    fn attributes(&self) -> Option<&Attributes> {
        None
    }
    fn get_children(&self) -> &Vec<MJBodyChild> {
        &EMPTY_CHILDREN
    }

    fn get_current_width(&self) -> Option<Size> {
        None
    }
}
