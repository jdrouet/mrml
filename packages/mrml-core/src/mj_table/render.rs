use super::{MjTable, NAME};
use crate::helper::size::Pixel;
use crate::mj_section::WithMjSectionBackground;
use crate::prelude::render::*;

struct MjTableRender<'e, 'h> {
    context: &'h RenderContext<'h>,
    element: &'e MjTable,
    container_width: Option<Pixel>,
}

impl<'e, 'h> WithMjSectionBackground<'e, 'h> for MjTableRender<'e, 'h> {}

impl<'e, 'h> MjTableRender<'e, 'h> {
    fn set_style_table<'a>(&self, tag: Tag<'a>) -> Tag<'a> {
        tag.maybe_add_style("color", self.attribute("color"))
            .maybe_add_style("font-family", self.attribute("font-family"))
            .maybe_add_style("font-size", self.attribute("font-size"))
            .maybe_add_style("line-height", self.attribute("line-height"))
            .maybe_add_style("table-layout", self.attribute("table-layout"))
            .maybe_add_style("width", self.attribute("width"))
            .maybe_add_style("border", self.attribute("border"))
    }
}

impl<'e, 'h> Render<'e, 'h> for MjTableRender<'e, 'h> {
    fn default_attribute(&self, name: &str) -> Option<&'static str> {
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

    fn raw_attribute(&self, key: &str) -> Option<&'e str> {
        self.element.attributes.get(key).map(|v| v.as_str())
    }

    fn tag(&self) -> Option<&str> {
        Some(NAME)
    }

    fn context(&self) -> &'h RenderContext<'h> {
        self.context
    }

    fn set_container_width(&mut self, width: Option<Pixel>) {
        self.container_width = width;
    }

    fn render(&self, cursor: &mut RenderCursor) -> Result<(), Error> {
        let font_family = self.attribute("font-family");
        cursor.header.maybe_add_font_families(font_family);

        let table = self
            .set_style_table(Tag::table())
            .add_attribute("border", "0")
            .maybe_add_attribute("cellpadding", self.attribute("cellpadding"))
            .maybe_add_attribute("cellspacing", self.attribute("cellspacing"))
            .maybe_add_attribute("width", self.attribute("width"));
        table.render_open(&mut cursor.buffer);
        for (index, child) in self.element.children.iter().enumerate() {
            let mut renderer = child.renderer(self.context());
            renderer.set_index(index);
            renderer.render(cursor)?;
        }
        table.render_close(&mut cursor.buffer);
        Ok(())
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MjTable {
    fn renderer(&'e self, context: &'h RenderContext<'h>) -> Box<dyn Render<'e, 'h> + 'r> {
        Box::new(MjTableRender::<'e, 'h> {
            element: self,
            context,
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
