use super::{MjAccordionText, NAME};
use crate::prelude::hash::Map;
use crate::prelude::render::*;

struct MjAccordionTextRender<'e, 'h> {
    header: &'h Header<'h>,
    element: &'e MjAccordionText,
    extra: Map<String, String>,
}

impl<'e, 'h> MjAccordionTextRender<'e, 'h> {
    fn render_children(
        &self,
        opts: &RenderOptions,
        header: &mut VariableHeader,
        buf: &mut RenderBuffer,
    ) -> Result<(), Error> {
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

        td.render_open(buf);
        for child in self.element.children.iter() {
            let renderer = child.renderer(self.header);
            renderer.render(opts, header, buf)?;
        }
        td.render_close(buf);

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

    fn header(&self) -> &'h Header<'h> {
        self.header
    }

    fn render(
        &self,
        opts: &RenderOptions,
        header: &mut VariableHeader,
        buf: &mut RenderBuffer,
    ) -> Result<(), Error> {
        let font_families = self.attribute("font-family");
        header.maybe_add_font_families(font_families);

        let tr = Tag::tr();
        let tbody = Tag::tbody();
        let table = Tag::table()
            .add_attribute("cellspacing", "0")
            .add_attribute("cellpadding", "0")
            .add_style("width", "100%")
            .maybe_add_style("border-bottom", self.attribute("border"));
        let div = Tag::div().add_class("mj-accordion-content");

        div.render_open(buf);
        table.render_open(buf);
        tbody.render_open(buf);
        tr.render_open(buf);
        self.render_children(opts, header, buf)?;
        tr.render_close(buf);
        tbody.render_close(buf);
        table.render_close(buf);
        div.render_close(buf);

        Ok(())
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MjAccordionText {
    fn renderer(&'e self, header: &'h Header<'h>) -> Box<dyn Render<'e, 'h> + 'r> {
        Box::new(MjAccordionTextRender::<'e, 'h> {
            element: self,
            header,
            extra: Map::default(),
        })
    }
}
