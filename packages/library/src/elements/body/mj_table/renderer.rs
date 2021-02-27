use super::MJTable;
use crate::elements::body::prelude::{
    to_children_iterator, BodyComponent, BodyComponentChildIterator,
};
use crate::elements::error::Error;
use crate::elements::prelude::*;
use crate::util::attributes::*;
use crate::util::context::Context;
use crate::util::header::Header;
use crate::util::size::Size;
use crate::util::tag::Tag;

impl MJTable {
    fn set_style_table(&self, tag: Tag) -> Tag {
        tag.maybe_set_style("border", self.get_attribute("border"))
            .maybe_set_style("color", self.get_attribute("color"))
            .maybe_set_style("font-family", self.get_attribute("font-family"))
            .maybe_set_style("font-size", self.get_attribute("font-size"))
            .maybe_set_style("line-height", self.get_attribute("line-height"))
            .maybe_set_style("table-layout", self.get_attribute("table-layout"))
            .maybe_set_style("width", self.get_attribute("width"))
    }
}

impl Component for MJTable {
    fn update_header(&self, header: &mut Header) {
        header.maybe_add_font_families(self.get_attribute("font-family"));
    }

    fn context(&self) -> Option<&Context> {
        self.context.as_ref()
    }

    fn set_context(&mut self, ctx: Context) {
        self.context = Some(ctx);
    }

    fn render(&self, header: &Header) -> Result<String, Error> {
        let res = self
            .get_children()
            .try_fold(String::default(), |res, child| {
                Ok(res + &child.render(header)?)
            })?;
        let table = self
            .set_style_table(Tag::new("table"))
            .set_attribute("border", 0)
            .maybe_set_attribute("cellpadding", self.get_attribute("cellpadding"))
            .maybe_set_attribute("cellspacing", self.get_attribute("cellspacing"))
            .maybe_set_attribute("width", self.get_attribute("width"));
        Ok(table.render(res))
    }
}

impl BodyComponent for MJTable {
    fn attributes(&self) -> Option<&Attributes> {
        Some(&self.attributes)
    }

    fn get_current_width(&self) -> Option<Size> {
        None
    }

    fn get_children(&self) -> BodyComponentChildIterator {
        to_children_iterator(&self.children)
    }

    fn get_children_len(&self) -> usize {
        self.children.len()
    }
}
