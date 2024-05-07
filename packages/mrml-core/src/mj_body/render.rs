use std::convert::TryFrom;

use super::MjBody;
use crate::helper::size::Pixel;
use crate::prelude::render::*;

impl<'root> Renderer<'root, MjBody, ()> {
    fn get_width(&self) -> Option<Pixel> {
        self.attribute("width")
            .and_then(|value| Pixel::try_from(value).ok())
    }

    fn get_body_tag(&self) -> Tag {
        self.set_body_style(Tag::new("body").add_style("word-spacing", "normal"))
    }

    fn get_content_div_tag(&self) -> Tag {
        self.set_body_style(Tag::new("div"))
            .maybe_add_attribute("class", self.attribute("css-class"))
            .maybe_add_attribute("lang", self.context.header.lang())
    }

    fn set_body_style<'a, 't>(&'a self, tag: Tag<'t>) -> Tag<'t>
    where
        'root: 'a,
        'a: 't,
    {
        tag.maybe_add_style("background-color", self.attribute("background-color"))
    }

    fn render_preview(&self, buf: &mut RenderBuffer) {
        if let Some(value) = self.context.header.preview() {
            buf.push_str(r#"<div style="display:none;font-size:1px;color:#ffffff;line-height:1px;max-height:0px;max-width:0px;opacity:0;overflow:hidden;">"#);
            buf.push_str(value);
            buf.push_str("</div>");
        }
    }

    fn render_content(&self, cursor: &mut RenderCursor) -> Result<(), Error> {
        let div = self.get_content_div_tag();
        let element_width = self.get_width();

        div.render_open(&mut cursor.buffer)?;
        let raw_siblings = self
            .element
            .children
            .iter()
            .filter(|item| item.is_raw())
            .count();
        for (index, child) in self.element.children.iter().enumerate() {
            let mut renderer = child.renderer(self.context());
            renderer.set_container_width(element_width.clone());
            renderer.set_index(index);
            renderer.set_raw_siblings(raw_siblings);
            renderer.set_siblings(self.element.children.len());
            renderer.render(cursor)?;
        }
        div.render_close(&mut cursor.buffer);
        Ok(())
    }
}

impl<'root> Render<'root> for Renderer<'root, MjBody, ()> {
    fn raw_attribute(&self, key: &str) -> Option<&'root str> {
        self.element.attributes.get(key).map(|v| v.as_str())
    }

    fn default_attribute(&self, key: &str) -> Option<&'static str> {
        match key {
            "width" => Some("600px"),
            _ => None,
        }
    }

    fn context(&self) -> &'root RenderContext<'root> {
        self.context
    }

    fn render(&self, cursor: &mut RenderCursor) -> Result<(), Error> {
        let body = self.get_body_tag();
        body.render_open(&mut cursor.buffer)?;
        self.render_preview(&mut cursor.buffer);
        self.render_content(cursor)?;
        body.render_close(&mut cursor.buffer);
        Ok(())
    }
}

impl<'render, 'root: 'render> Renderable<'render, 'root> for MjBody {
    fn renderer(
        &'root self,
        context: &'root RenderContext<'root>,
    ) -> Box<dyn Render<'root> + 'render> {
        Box::new(Renderer::new(context, self, ()))
    }
}

#[cfg(test)]
mod tests {
    crate::should_render!(empty, "mj-body");
}
