use super::{MjNavbarLink, NAME};
use crate::helper::size::Pixel;
use crate::prelude::hash::Map;
use crate::prelude::render::*;

struct MjNavbarLinkExtra<'a> {
    attributes: Map<&'a str, &'a str>,
}

impl<'a> Default for MjNavbarLinkExtra<'a> {
    fn default() -> Self {
        Self {
            attributes: Map::new(),
        }
    }
}

impl<'root> Renderer<'root, MjNavbarLink, MjNavbarLinkExtra<'root>> {
    fn set_style_a<'a, 't>(&'a self, tag: Tag<'t>) -> Tag<'t>
    where
        'root: 'a,
        'a: 't,
    {
        tag.add_style("display", "inline-block")
            .maybe_add_style("color", self.attribute("color"))
            .maybe_add_style("font-family", self.attribute("font-family"))
            .maybe_add_style("font-size", self.attribute("font-size"))
            .maybe_add_style("font-style", self.attribute("font-style"))
            .maybe_add_style("font-weight", self.attribute("font-weight"))
            .maybe_add_style("letter-spacing", self.attribute("letter-spacing"))
            .maybe_add_style("line-height", self.attribute("line-height"))
            .maybe_add_style("text-decoration", self.attribute("text-decoration"))
            .maybe_add_style("text-transform", self.attribute("text-transform"))
            .maybe_add_style("padding", self.attribute("padding"))
            .maybe_add_style("padding-top", self.attribute("padding-top"))
            .maybe_add_style("padding-right", self.attribute("padding-right"))
            .maybe_add_style("padding-bottom", self.attribute("padding-bottom"))
            .maybe_add_style("padding-left", self.attribute("padding-left"))
    }

    fn set_style_td<'a, 't>(&'a self, tag: Tag<'t>) -> Tag<'t>
    where
        'root: 'a,
        'a: 't,
    {
        tag.maybe_add_style("padding", self.attribute("padding"))
            .maybe_add_style("padding-top", self.attribute("padding-top"))
            .maybe_add_style("padding-right", self.attribute("padding-right"))
            .maybe_add_style("padding-bottom", self.attribute("padding-bottom"))
            .maybe_add_style("padding-left", self.attribute("padding-left"))
    }

    fn get_link(&self) -> Option<String> {
        self.attribute("href").as_ref().and_then(|href| {
            self.attribute("navbar-base-url")
                .map(move |base| format!("{base}{href}"))
                .or_else(|| Some(href.to_string()))
        })
    }

    fn render_content(&self, cursor: &mut RenderCursor) -> Result<(), Error> {
        let link = self
            .set_style_a(Tag::new("a"))
            .add_class("mj-link")
            .maybe_add_class(self.attribute("css-class"))
            .maybe_add_attribute("href", self.get_link())
            .maybe_add_attribute("rel", self.attribute("rel"))
            .maybe_add_attribute("target", self.attribute("target"))
            .maybe_add_attribute("name", self.attribute("name"));

        link.render_open(&mut cursor.buffer)?;
        for child in self.element.children.iter() {
            let renderer = child.renderer(self.context());
            renderer.render(cursor)?;
        }
        link.render_close(&mut cursor.buffer);

        Ok(())
    }
}

impl<'root> Render<'root> for Renderer<'root, MjNavbarLink, MjNavbarLinkExtra<'root>> {
    fn default_attribute(&self, key: &str) -> Option<&'static str> {
        match key {
            "color" => Some("#000000"),
            "font-family" => Some("Ubuntu, Helvetica, Arial, sans-serif"),
            "font-size" => Some("13px"),
            "font-weight" => Some("normal"),
            "line-height" => Some("22px"),
            "padding" => Some("15px 10px"),
            "target" => Some("_blank"),
            "text-decoration" => Some("none"),
            "text-transform" => Some("uppercase"),
            _ => None,
        }
    }

    fn add_extra_attribute(&mut self, key: &'root str, value: &'root str) {
        self.extra.attributes.insert(key, value);
    }

    fn raw_extra_attribute(&self, key: &str) -> Option<&'root str> {
        self.extra.attributes.get(key).copied()
    }

    fn raw_attribute(&self, key: &str) -> Option<&'root str> {
        self.element.attributes.get(key).map(|v| v.as_str())
    }

    fn tag(&self) -> Option<&str> {
        Some(NAME)
    }

    fn set_container_width(&mut self, width: Option<Pixel>) {
        self.container_width = width;
    }

    fn context(&self) -> &'root RenderContext<'root> {
        self.context
    }

    fn render(&self, cursor: &mut RenderCursor) -> Result<(), Error> {
        let font_families = self.attribute("font-family");
        cursor.header.maybe_add_font_families(font_families);

        let td = self
            .set_style_td(Tag::td())
            .maybe_add_suffixed_class(self.attribute("css-class"), "outlook");

        cursor.buffer.start_conditional_tag();
        td.render_open(&mut cursor.buffer)?;
        cursor.buffer.end_conditional_tag();
        self.render_content(cursor)?;
        cursor.buffer.start_conditional_tag();
        td.render_close(&mut cursor.buffer);
        cursor.buffer.end_conditional_tag();

        Ok(())
    }
}

impl<'render, 'root: 'render> Renderable<'render, 'root> for MjNavbarLink {
    fn renderer(
        &'root self,
        context: &'root RenderContext<'root>,
    ) -> Box<dyn Render<'root> + 'render> {
        Box::new(Renderer::new(context, self, MjNavbarLinkExtra::default()))
    }
}
