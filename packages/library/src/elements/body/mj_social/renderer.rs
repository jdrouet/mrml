use super::MJSocial;
use crate::elements::body::prelude::*;
use crate::elements::body::BodyElement;
use crate::elements::error::Error;
use crate::elements::prelude::*;
use crate::util::attributes::*;
use crate::util::condition::*;
use crate::util::context::Context;
use crate::util::header::Header;
use crate::util::size::Size;
use crate::util::tag::Tag;

impl MJSocial {
    fn set_style_table_vertical(&self, tag: Tag) -> Tag {
        tag.set_style("margin", "0px")
    }

    fn is_horizontal(&self) -> bool {
        self.get_attribute("mode")
            .map(|mode| mode == "horizontal")
            .unwrap_or(true)
    }

    fn render_horizontal(&self, header: &Header) -> Result<String, Error> {
        let table =
            Tag::table_presentation().maybe_set_attribute("align", self.get_attribute("align"));
        let tr = Tag::tr();
        let td = Tag::td();
        let inner_table = Tag::table_presentation()
            .maybe_set_attribute("align", self.get_attribute("align"))
            .set_style("display", "inline-table")
            .set_style("float", "none");
        let mut res = vec![];
        res.push(START_CONDITIONAL_TAG.into());
        res.push(table.open());
        res.push(tr.open());
        res.push(END_CONDITIONAL_TAG.into());
        for child in self.children.iter() {
            res.push(conditional_tag(td.open()));
            res.push(inner_table.render(child.render(header)?));
            res.push(conditional_tag(td.close()));
        }
        res.push(START_CONDITIONAL_TAG.into());
        res.push(tr.close());
        res.push(table.close());
        res.push(END_CONDITIONAL_TAG.into());
        Ok(res.join(""))
    }

    fn render_vertical(&self, header: &Header) -> Result<String, Error> {
        let table = self.set_style_table_vertical(Tag::table_presentation());
        let mut res = vec![];
        for child in self.children.iter() {
            // TODO set child attributes
            res.push(child.render(header)?);
        }
        Ok(table.render(res.join("")))
    }
}

impl Component for MJSocial {
    fn update_header(&self, header: &mut Header) {
        for child in self.children.iter() {
            child.update_header(header);
        }
    }

    fn context(&self) -> Option<&Context> {
        self.context.as_ref()
    }

    fn set_context(&mut self, ctx: Context) {
        self.context = Some(ctx);
        let child_base = Context::new(
            self.get_container_width(),
            self.get_siblings(),
            self.get_raw_siblings(),
            0,
        );
        for (idx, child) in self.children.iter_mut().enumerate() {
            child.set_context(child_base.clone().set_index(idx));
        }
    }

    fn render(&self, header: &Header) -> Result<String, Error> {
        if self.is_horizontal() {
            self.render_horizontal(header)
        } else {
            self.render_vertical(header)
        }
    }
}

impl BodyComponent for MJSocial {
    fn get_children(&self) -> &Vec<BodyElement> {
        &self.children
    }

    fn get_current_width(&self) -> Option<Size> {
        self.context().and_then(|ctx| ctx.container_width())
    }

    fn attributes(&self) -> Option<&Attributes> {
        Some(&self.attributes)
    }

    fn set_style(&self, name: &str, tag: Tag) -> Tag {
        match name {
            "table-vertical" => self.set_style_table_vertical(tag),
            _ => tag,
        }
    }
}
