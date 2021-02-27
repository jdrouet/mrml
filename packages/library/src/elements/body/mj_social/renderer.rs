use super::MJSocial;
use crate::elements::body::prelude::{
    to_children_iterator, BodyChild, BodyComponent, BodyComponentChildIterator,
};
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
        let before = conditional_tag(table.open() + &tr.open());
        let after = conditional_tag(tr.close() + &table.close());
        let content = self
            .get_children()
            .try_fold(String::default(), |res, child| {
                Ok(res
                    + &conditional_tag(td.open())
                    + &inner_table.render(child.render(header)?)
                    + &conditional_tag(td.close()))
            })?;
        Ok(before + &content + &after)
    }

    fn render_vertical(&self, header: &Header) -> Result<String, Error> {
        let table = self.set_style_table_vertical(Tag::table_presentation());
        let content = self
            .get_children()
            .try_fold(String::default(), |res, child| {
                Ok(res + &child.render(header)?)
            })?;
        Ok(table.render(content))
    }
}

impl Component for MJSocial {
    fn update_header(&self, header: &mut Header) {
        self.get_children()
            .for_each(|child| child.update_header(header));
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
        self.children
            .iter_mut()
            .enumerate()
            .for_each(|(idx, child)| {
                child
                    .inner_mut()
                    .set_context(child_base.clone().set_index(idx));
            });
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
    fn get_children(&self) -> BodyComponentChildIterator {
        to_children_iterator(&self.children)
    }

    fn get_children_len(&self) -> usize {
        self.children.len()
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
