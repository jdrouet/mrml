use super::{MjAccordionText, NAME};
use crate::prelude::hash::Map;
use crate::prelude::render::*;

struct MjAccordionTextRender<'e, 'h> {
    context: &'h RenderContext<'h>,
    element: &'e MjAccordionText,
    extra: Map<String, String>,
}

impl<'e, 'h> MjAccordionTextRender<'e, 'h> {
    fn render_children(&self, cursor: &mut RenderCursor) -> Result<(), Error> {
        let td = Tag::td()
            .maybe_add_class(self.attribute("css-class"))
            .maybe_add_style("background", self.attribute("background-color"))
            .maybe_add_style("font-size", self.attribute("font-size"))
            .maybe_add_style("font-family", self.attribute("font-family"))
            .maybe_add_style("line-height", self.attribute("line-height"))
            .maybe_add_style("color", self.attribute("color"))
            .maybe_add_style("padding-top", self.attribute("padding-top"))
            .maybe_add_style("padding-right", self.attribute("padding-right"))
            .maybe_add_style("padding-bottom", self.attribute("padding-bottom"))
            .maybe_add_style("padding-left", self.attribute("padding-left"))
            .maybe_add_style("padding", self.attribute("padding"));

        td.render_open(&mut cursor.buffer);
        for child in self.element.children.iter() {
            let renderer = child.renderer(self.context());
            renderer.render(cursor)?;
        }
        td.render_close(&mut cursor.buffer);

        Ok(())
    }
}

impl<'e, 'h> Render<'e, 'h> for MjAccordionTextRender<'e, 'h> {
    fn default_attribute(&self, name: &str) -> Option<&'static str> {
        match name {
            "line-height" => Some("1"),
            "font-size" => Some("13px"),
            "padding" => Some("16px"),
            _ => None,
        }
    }

    fn add_extra_attribute(&mut self, key: &str, value: &str) {
        self.extra.insert(key.to_string(), value.to_string());
    }

    fn raw_extra_attribute(&self, key: &str) -> Option<&str> {
        self.extra.get(key).map(|v| v.as_str())
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

    fn render(&self, cursor: &mut RenderCursor) -> Result<(), Error> {
        let font_families = self.attribute("font-family");
        cursor.header.maybe_add_font_families(font_families);

        let tr = Tag::tr();
        let tbody = Tag::tbody();
        let table = Tag::table()
            .add_attribute("cellspacing", "0")
            .add_attribute("cellpadding", "0")
            .add_style("width", "100%")
            .maybe_add_style("border-bottom", self.attribute("border"));
        let div = Tag::div().add_class("mj-accordion-content");

        div.render_open(&mut cursor.buffer);
        table.render_open(&mut cursor.buffer);
        tbody.render_open(&mut cursor.buffer);
        tr.render_open(&mut cursor.buffer);
        self.render_children(cursor)?;
        tr.render_close(&mut cursor.buffer);
        tbody.render_close(&mut cursor.buffer);
        table.render_close(&mut cursor.buffer);
        div.render_close(&mut cursor.buffer);

        Ok(())
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MjAccordionText {
    fn renderer(&'e self, context: &'h RenderContext<'h>) -> Box<dyn Render<'e, 'h> + 'r> {
        Box::new(MjAccordionTextRender::<'e, 'h> {
            element: self,
            context,
            extra: Map::default(),
        })
    }
}
