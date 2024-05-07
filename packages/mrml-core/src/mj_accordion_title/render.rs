use super::{MjAccordionTitle, NAME};
use crate::prelude::hash::Map;
use crate::prelude::render::*;

struct MjAccordionTitleExtra<'a> {
    attributes: Map<&'a str, &'a str>,
}

impl<'root> Renderer<'root, MjAccordionTitle, MjAccordionTitleExtra<'root>> {
    fn set_style_img<'a, 't>(&'a self, tag: Tag<'t>) -> Tag<'t>
    where
        'root: 'a,
        'a: 't,
    {
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

        td.render_open(&mut cursor.buffer)?;
        for child in self.element.children.iter() {
            let renderer = child.renderer(self.context());
            renderer.render(cursor)?;
        }
        td.render_close(&mut cursor.buffer);

        Ok(())
    }

    fn render_icons(&self, buf: &mut RenderBuffer) -> Result<(), Error> {
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
        td.render_open(buf)?;
        img_more.render_closed(buf)?;
        img_less.render_closed(buf)?;
        td.render_close(buf);
        buf.end_negation_conditional_tag();

        Ok(())
    }
}

impl<'root> Render<'root> for Renderer<'root, MjAccordionTitle, MjAccordionTitleExtra<'root>> {
    fn add_extra_attribute(&mut self, key: &'root str, value: &'root str) {
        self.extra.attributes.insert(key, value);
    }

    fn raw_extra_attribute(&self, key: &str) -> Option<&'root str> {
        self.extra.attributes.get(key).copied()
    }

    fn default_attribute(&self, name: &str) -> Option<&'static str> {
        match name {
            "font-size" => Some("13px"),
            "padding" => Some("16px"),
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

        div.render_open(&mut cursor.buffer)?;
        table.render_open(&mut cursor.buffer)?;
        tbody.render_open(&mut cursor.buffer)?;
        tr.render_open(&mut cursor.buffer)?;

        if self.attribute_equals("icon-position", "right") {
            self.render_title(cursor)?;
            self.render_icons(&mut cursor.buffer)?;
        } else {
            self.render_icons(&mut cursor.buffer)?;
            self.render_title(cursor)?;
        }

        tr.render_close(&mut cursor.buffer);
        tbody.render_close(&mut cursor.buffer);
        table.render_close(&mut cursor.buffer);
        div.render_close(&mut cursor.buffer);

        Ok(())
    }
}

impl<'render, 'root: 'render> Renderable<'render, 'root> for MjAccordionTitle {
    fn renderer(
        &'root self,
        context: &'root RenderContext<'root>,
    ) -> Box<dyn Render<'root> + 'render> {
        Box::new(Renderer::new(
            context,
            self,
            MjAccordionTitleExtra {
                attributes: Map::new(),
            },
        ))
    }
}
