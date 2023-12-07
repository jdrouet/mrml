use std::cell::{Ref, RefCell};
use std::rc::Rc;

use super::{MjTable, NAME};
use crate::helper::size::Pixel;
use crate::helper::tag::Tag;
use crate::mj_section::WithMjSectionBackground;
use crate::prelude::hash::Map;
use crate::prelude::render::{Error, Header, Render, RenderOptions, Renderable};

struct MjTableRender<'e, 'h> {
    header: Rc<RefCell<Header<'h>>>,
    element: &'e MjTable,
    container_width: Option<Pixel>,
}

impl<'e, 'h> WithMjSectionBackground<'h> for MjTableRender<'e, 'h> {}

impl<'e, 'h> MjTableRender<'e, 'h> {
    fn set_style_table(&self, tag: Tag) -> Tag {
        tag.maybe_add_style("color", self.attribute("color"))
            .maybe_add_style("font-family", self.attribute("font-family"))
            .maybe_add_style("font-size", self.attribute("font-size"))
            .maybe_add_style("line-height", self.attribute("line-height"))
            .maybe_add_style("table-layout", self.attribute("table-layout"))
            .maybe_add_style("width", self.attribute("width"))
            .maybe_add_style("border", self.attribute("border"))
    }
}

impl<'e, 'h> Render<'h> for MjTableRender<'e, 'h> {
    fn default_attribute(&self, name: &str) -> Option<&str> {
        match name {
            "align" => Some("left"),
            "border" => Some("none"),
            "cellpadding" => Some("0"),
            "cellspacing" => Some("0"),
            "color" => Some("#000000"),
            "font-family" => Some("Ubuntu, Helvetica, Arial, sans-serif"),
            "font-size" => Some("13px"),
            "line-height" => Some("22px"),
            "padding" => Some("10px 25px"),
            "table-layout" => Some("auto"),
            "width" => Some("100%"),
            _ => None,
        }
    }

    fn attributes(&self) -> Option<&Map<String, String>> {
        Some(&self.element.attributes)
    }

    fn tag(&self) -> Option<&str> {
        Some(NAME)
    }

    fn header(&self) -> Ref<Header<'h>> {
        self.header.borrow()
    }

    fn set_container_width(&mut self, width: Option<Pixel>) {
        self.container_width = width;
    }

    fn render(&self, opts: &RenderOptions) -> Result<String, Error> {
        let font_family = self.attribute("font-family");
        self.header
            .borrow_mut()
            .maybe_add_font_families(font_family);
        let children = self.element.children.iter().enumerate().try_fold(
            String::default(),
            |res, (index, child)| {
                let mut renderer = child.renderer(Rc::clone(&self.header));
                renderer.set_index(index);
                Ok(res + &renderer.render(opts)?)
            },
        )?;
        let table = self
            .set_style_table(Tag::table())
            .add_attribute("border", "0")
            .maybe_add_attribute("cellpadding", self.attribute("cellpadding"))
            .maybe_add_attribute("cellspacing", self.attribute("cellspacing"))
            .maybe_add_attribute("width", self.attribute("width"));
        Ok(table.render(children))
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MjTable {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'h> + 'r> {
        Box::new(MjTableRender::<'e, 'h> {
            element: self,
            header,
            container_width: None,
        })
    }
}

#[cfg(test)]
mod tests {
    crate::should_render!(basic, "mj-table");
    crate::should_render!(table, "mj-table-table");
    crate::should_render!(text, "mj-table-text");
    crate::should_render!(other, "mj-table-other");
}
