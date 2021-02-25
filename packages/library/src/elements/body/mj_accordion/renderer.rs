use super::MJAccordion;
use crate::elements::body::prelude::{BodyComponent, BodyComponentChildIterator};
use crate::elements::error::Error;
use crate::elements::prelude::*;
use crate::util::attributes::*;
use crate::util::context::Context;
use crate::util::header::Header;
use crate::util::size::Size;
use crate::util::tag::Tag;

impl Component for MJAccordion {
    fn update_header(&self, header: &mut Header) {
        header.add_style(r#"
        noinput.mj-accordion-checkbox { display: block! important; }

        @media yahoo, only screen and (min-width:0) {
          .mj-accordion-element { display:block; }
          input.mj-accordion-checkbox, .mj-accordion-less { display: none !important; }
          input.mj-accordion-checkbox + * .mj-accordion-title { cursor: pointer; touch-action: manipulation; -webkit-user-select: none; -moz-user-select: none; user-select: none; }
          input.mj-accordion-checkbox + * .mj-accordion-content { overflow: hidden; display: none; }
          input.mj-accordion-checkbox + * .mj-accordion-more { display: block !important; }
          input.mj-accordion-checkbox:checked + * .mj-accordion-content { display: block; }
          input.mj-accordion-checkbox:checked + * .mj-accordion-more { display: none !important; }
          input.mj-accordion-checkbox:checked + * .mj-accordion-less { display: block !important; }
        }

        .moz-text-html input.mj-accordion-checkbox + * .mj-accordion-title { cursor: auto; touch-action: auto; -webkit-user-select: auto; -moz-user-select: auto; user-select: auto; }
        .moz-text-html input.mj-accordion-checkbox + * .mj-accordion-content { overflow: hidden; display: block; }
        .moz-text-html input.mj-accordion-checkbox + * .mj-accordion-ico { display: none; }

        @goodbye { @gmail }
        "#);
        header.maybe_add_font_families(self.get_attribute("font-family"));
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
        let mut children = vec![];
        for child in self.children.iter() {
            children.push(child.render(header)?);
        }
        let tbody = Tag::tbody().render(children.join(""));
        let table = Tag::table()
            .set_style("width", "100%")
            .set_style("border-collapse", "collapse")
            .maybe_set_style("border", self.get_attribute("border"))
            .set_style("border-bottom", "none")
            .maybe_set_style("font-family", self.get_attribute("font-family"))
            .set_attribute("cell-spacing", 0)
            .set_attribute("cell-padding", 0)
            .set_class("mj-accordion")
            .render(tbody);
        Ok(table)
    }
}

impl BodyComponent for MJAccordion {
    fn attributes(&self) -> Option<&Attributes> {
        Some(&self.attributes)
    }

    fn get_children(&self) -> BodyComponentChildIterator {
        Box::new(self.children.iter().map(|item| item.inner()))
    }

    fn get_children_len(&self) -> usize {
        self.children.len()
    }

    fn get_current_width(&self) -> Option<Size> {
        self.context().and_then(|ctx| ctx.container_width())
    }

    fn set_style(&self, _name: &str, tag: Tag) -> Tag {
        tag
    }
}
