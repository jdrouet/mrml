use super::Comment;
use crate::elements::body::prelude::BodyComponent;
use crate::elements::{Component, Error};
use crate::util::attributes::Attributes;
use crate::util::context::Context;
use crate::util::header::Header;
use crate::util::size::Size;

impl Component for Comment {
    fn context(&self) -> Option<&Context> {
        None
    }

    fn set_context(&mut self, _ctx: Context) {
        //
    }

    fn render(&self, header: &Header) -> Result<String, Error> {
        if header.keep_comments {
            Ok(format!("<!-- {} -->", self.content))
        } else {
            Ok(String::new())
        }
    }
}

impl BodyComponent for Comment {
    fn attributes(&self) -> Option<&Attributes> {
        None
    }

    fn is_raw(&self) -> bool {
        true
    }

    fn get_current_width(&self) -> Option<Size> {
        None
    }
}
