use super::Text;
use crate::elements::body::prelude::BodyComponent;
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
    fn is_raw(&self) -> bool {
        true
    }

    fn attributes(&self) -> Option<&Attributes> {
        None
    }

    fn get_current_width(&self) -> Option<Size> {
        None
    }
}
