use super::{MjTable, NAME};
use crate::helper::size::Pixel;
use crate::mj_body::MjBodyChild;
use crate::mj_section::WithMjSectionBackground;
use crate::prelude::render::*;

impl<'root> WithMjSectionBackground<'root> for Renderer<'root, MjTable, ()> {}

impl<'root> Renderer<'root, MjTable, ()> {
    fn children_iter(&self) -> impl Iterator<Item = &MjBodyChild> {
        fn folder<'root>(c: &'root MjBodyChild) -> Box<dyn Iterator<Item = &MjBodyChild> + 'root> {
            match c {
                MjBodyChild::Fragment(f) => Box::new(f.children.iter().flat_map(folder)),
                _ => Box::new(std::iter::once(c)),
            }
        }
        self.element.children.iter().flat_map(folder)
    }

    fn set_style_table<'a, 't>(&'a self, tag: Tag<'t>) -> Tag<'t>
    where
        'root: 'a,
        'a: 't,
    {
        tag.maybe_add_style("color", self.attribute("color"))
            .maybe_add_style("font-family", self.attribute("font-family"))
            .maybe_add_style("font-size", self.attribute("font-size"))
            .maybe_add_style("line-height", self.attribute("line-height"))
            .maybe_add_style("table-layout", self.attribute("table-layout"))
            .maybe_add_style("width", self.attribute("width"))
            .maybe_add_style("border", self.attribute("border"))
    }
}

impl<'root> Render<'root> for Renderer<'root, MjTable, ()> {
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

    fn raw_attribute(&self, key: &str) -> Option<&'root str> {
        self.element.attributes.get(key).map(|v| v.as_str())
    }

    fn tag(&self) -> Option<&str> {
        Some(NAME)
    }

    fn context(&self) -> &'root RenderContext<'root> {
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
        table.render_open(&mut cursor.buffer)?;
        for (index, child) in self.children_iter().enumerate() {
            let mut renderer = child.renderer(self.context());
            renderer.set_index(index);
            renderer.render(cursor)?;
        }
        table.render_close(&mut cursor.buffer);
        Ok(())
    }
}

impl<'render, 'root: 'render> Renderable<'render, 'root> for MjTable {
    fn renderer(
        &'root self,
        context: &'root RenderContext<'root>,
    ) -> Box<dyn Render<'root> + 'render> {
        Box::new(Renderer::new(context, self, ()))
    }
}

#[cfg(test)]
mod tests {
    crate::should_render!(basic, "mj-table");
    crate::should_render!(table, "mj-table-table");
    crate::should_render!(text, "mj-table-text");
    crate::should_render!(other, "mj-table-other");
}
