use super::{MjAccordionTitle, NAME};
use crate::prelude::hash::Map;
use crate::prelude::render::*;

struct MjAccordionTitleRender<'e, 'h> {
    context: &'h RenderContext<'h>,
    element: &'e MjAccordionTitle,
    extra: Map<String, String>,
}

impl<'e, 'h> MjAccordionTitleRender<'e, 'h> {
    fn set_style_img<'a>(&self, tag: Tag<'a>) -> Tag<'a> {
        tag.add_style("display", "none")
            .maybe_add_style("width", self.attribute("icon-width"))
            .maybe_add_style("height", self.attribute("icon-height"))
    }

    fn render_title(&self, cursor: &mut RenderCursor) -> Result<(), Error> {
        let td = Tag::td()
            .add_style("width", "100%")
            .maybe_add_style("background-color", self.attribute("background-color"))
            .maybe_add_style("color", self.attribute("color"))
            .maybe_add_style("font-size", self.attribute("font-size"))
            .maybe_add_style("font-family", self.attribute("font-family"))
            .maybe_add_style("padding-top", self.attribute("padding-top"))
            .maybe_add_style("padding-right", self.attribute("padding-right"))
            .maybe_add_style("padding-bottom", self.attribute("padding-bottom"))
            .maybe_add_style("padding-left", self.attribute("padding-left"))
            .maybe_add_style("padding", self.attribute("padding"))
            .maybe_add_class(self.attribute("css-class"));

        td.render_open(&mut cursor.buffer);
        for child in self.element.children.iter() {
            let renderer = child.renderer(self.context());
            renderer.render(cursor)?;
        }
        td.render_close(&mut cursor.buffer);

        Ok(())
    }

    fn render_icons(&self, buf: &mut RenderBuffer) {
        let img_more = self
            .set_style_img(Tag::new("img"))
            .maybe_add_attribute("src", self.attribute("icon-wrapped-url"))
            .maybe_add_attribute("alt", self.attribute("icon-wrapped-alt"))
            .add_class("mj-accordion-more");
        let img_less = self
            .set_style_img(Tag::new("img"))
            .maybe_add_attribute("src", self.attribute("icon-unwrapped-url"))
            .maybe_add_attribute("alt", self.attribute("icon-unwrapped-alt"))
            .add_class("mj-accordion-less");
        let td = Tag::td()
            .add_style("padding", "16px")
            .maybe_add_style("background", self.attribute("background-color"))
            .maybe_add_style("vertical-align", self.attribute("icon-align"))
            .add_class("mj-accordion-ico");

        buf.start_negation_conditional_tag();
        td.render_open(buf);
        img_more.render_closed(buf);
        img_less.render_closed(buf);
        td.render_close(buf);
        buf.end_negation_conditional_tag();
    }
}

impl<'e, 'h> Render<'e, 'h> for MjAccordionTitleRender<'e, 'h> {
    fn add_extra_attribute(&mut self, key: &str, value: &str) {
        self.extra.insert(key.to_string(), value.to_string());
    }

    fn raw_extra_attribute(&self, key: &str) -> Option<&str> {
        self.extra.get(key).map(|v| v.as_str())
    }

    fn default_attribute(&self, name: &str) -> Option<&'static str> {
        match name {
            "font-size" => Some("13px"),
            "padding" => Some("16px"),
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
        let div = Tag::div().add_class("mj-accordion-title");

        div.render_open(&mut cursor.buffer);
        table.render_open(&mut cursor.buffer);
        tbody.render_open(&mut cursor.buffer);
        tr.render_open(&mut cursor.buffer);

        if self.attribute_equals("icon-position", "right") {
            self.render_title(cursor)?;
            self.render_icons(&mut cursor.buffer);
        } else {
            self.render_icons(&mut cursor.buffer);
            self.render_title(cursor)?;
        }

        tr.render_close(&mut cursor.buffer);
        tbody.render_close(&mut cursor.buffer);
        table.render_close(&mut cursor.buffer);
        div.render_close(&mut cursor.buffer);

        Ok(())
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MjAccordionTitle {
    fn renderer(&'e self, context: &'h RenderContext<'h>) -> Box<dyn Render<'e, 'h> + 'r> {
        Box::new(MjAccordionTitleRender::<'e, 'h> {
            element: self,
            context,
            extra: Map::new(),
        })
    }
}
