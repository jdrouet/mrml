use super::MJText;
use crate::elements::body::prelude::{
    to_children_iterator, BodyComponent, BodyComponentChildIterator,
};
use crate::elements::error::Error;
use crate::elements::prelude::*;
use crate::util::attributes::*;
use crate::util::condition::*;
use crate::util::context::Context;
use crate::util::header::Header;
use crate::util::size::Size;
use crate::util::tag::Tag;

impl MJText {
    fn set_style_text(&self, tag: Tag) -> Tag {
        tag.maybe_set_style("font-family", self.get_attribute("font-family"))
            .maybe_set_style("font-size", self.get_attribute("font-size"))
            .maybe_set_style("font-style", self.get_attribute("font-style"))
            .maybe_set_style("font-weight", self.get_attribute("font-weight"))
            .maybe_set_style("letter-spacing", self.get_attribute("letter-spacing"))
            .maybe_set_style("line-height", self.get_attribute("line-height"))
            .maybe_set_style("text-align", self.get_attribute("align"))
            .maybe_set_style("text-decoration", self.get_attribute("text-decoration"))
            .maybe_set_style("text-transform", self.get_attribute("text-transform"))
            .maybe_set_style("color", self.get_attribute("color"))
            .maybe_set_style("height", self.get_attribute("height"))
    }

    fn render_content(&self, header: &Header) -> Result<String, Error> {
        let mut res = vec![];
        for child in self.get_children() {
            res.push(child.render(header)?);
        }
        Ok(self.set_style_text(Tag::div()).render(res.join("")))
    }

    fn render_with_height(&self, header: &Header, height: &str) -> Result<String, Error> {
        let table = Tag::table_presentation();
        let tr = Tag::tr();
        let td = Tag::td()
            .set_attribute("height", height)
            .set_style("height", height)
            .set_style("vertical-align", "top");
        Ok(conditional_tag(table.render(
            tr.render(td.render(self.render_content(header)?)),
        )))
    }
}

impl Component for MJText {
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
        match self.get_attribute("height") {
            Some(value) => self.render_with_height(header, value),
            None => self.render_content(header),
        }
    }
}

impl BodyComponent for MJText {
    fn attributes(&self) -> Option<&Attributes> {
        Some(&self.attributes)
    }

    fn get_children(&self) -> BodyComponentChildIterator {
        to_children_iterator(&self.children)
    }

    fn get_children_len(&self) -> usize {
        self.children.len()
    }

    fn get_current_width(&self) -> Option<Size> {
        None
    }

    fn set_style(&self, name: &str, tag: Tag) -> Tag {
        match name {
            "text" => self.set_style_text(tag),
            _ => tag,
        }
    }
}
