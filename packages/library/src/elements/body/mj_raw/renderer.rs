use super::MJRaw;
use crate::elements::body::mj_body::children::MJBodyChild;
use crate::elements::body::prelude::*;
use crate::elements::error::Error;
use crate::elements::prelude::*;
use crate::util::attributes::Attributes;
use crate::util::context::Context;
use crate::util::header::Header;
use crate::util::size::Size;

impl Component for MJRaw {
    fn context(&self) -> Option<&Context> {
        self.context.as_ref()
    }

    fn set_context(&mut self, ctx: Context) {
        self.context = Some(ctx);
    }

    fn render(&self, header: &Header) -> Result<String, Error> {
        let mut res = vec![];
        for child in self.children.iter() {
            res.push(child.render(header)?);
        }
        Ok(res.join(""))
    }
}

impl BodyComponent for MJRaw {
    fn attributes(&self) -> Option<&Attributes> {
        None
    }

    fn get_children<'p>(&'p self) -> Box<dyn Iterator<Item = &'p MJBodyChild> + 'p> {
        Box::new(self.children.iter())
    }

    fn get_children_len(&self) -> usize {
        self.children.len()
    }

    fn get_current_width(&self) -> Option<Size> {
        None
    }
}
