use std::convert::TryFrom;

use super::MjBody;
use crate::helper::size::Pixel;
use crate::prelude::render::*;

struct MjBodyRender<'e, 'h> {
    context: &'h RenderContext<'h>,
    element: &'e MjBody,
}

impl<'e, 'h> MjBodyRender<'e, 'h> {
    fn get_width(&self) -> Option<Pixel> {
        self.attribute("width")
            .and_then(|value| Pixel::try_from(value.as_str()).ok())
    }

    fn get_body_tag(&self) -> Tag {
        self.set_body_style(Tag::new("body").add_style("word-spacing", "normal"))
    }

    fn get_content_div_tag(&self) -> Tag {
        self.set_body_style(Tag::new("div"))
            .maybe_add_attribute("class", self.attribute("css-class"))
            .maybe_add_attribute("lang", self.context.header.lang().map(ToString::to_string))
    }

    fn set_body_style<'a>(&self, tag: Tag<'a>) -> Tag<'a> {
        tag.maybe_add_style("background-color", self.attribute("background-color"))
    }

    fn render_preview(&self, buf: &mut RenderBuffer) {
        if let Some(value) = self.context.header.preview() {
            buf.push_str(r#"<div style="display:none;font-size:1px;color:#ffffff;line-height:1px;max-height:0px;max-width:0px;opacity:0;overflow:hidden;">"#);
            buf.push_str(value);
            buf.push_str("</div>");
        }
    }

    fn render_content(
        &self,
        header: &mut VariableHeader,
        buf: &mut RenderBuffer,
    ) -> Result<(), Error> {
        let div = self.get_content_div_tag();
        let element_width = self.get_width();

        div.render_open(buf);
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
            renderer.render(header, buf)?;
        }
        div.render_close(buf);
        Ok(())
    }
}

impl<'e, 'h> Render<'e, 'h> for MjBodyRender<'e, 'h> {
    fn raw_attribute(&self, key: &str) -> Option<&'e str> {
        self.element.attributes.get(key).map(|v| v.as_str())
    }

    fn default_attribute(&self, key: &str) -> Option<&'static str> {
        match key {
            "width" => Some("600px"),
            _ => None,
        }
    }

    fn context(&self) -> &'h RenderContext<'h> {
        self.context
    }

    fn render(&self, header: &mut VariableHeader, buf: &mut RenderBuffer) -> Result<(), Error> {
        let body = self.get_body_tag();
        body.render_open(buf);
        self.render_preview(buf);
        self.render_content(header, buf)?;
        body.render_close(buf);
        Ok(())
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MjBody {
    fn renderer(&'e self, context: &'h RenderContext<'h>) -> Box<dyn Render<'e, 'h> + 'r> {
        Box::new(MjBodyRender::<'e, 'h> {
            element: self,
            context,
        })
    }
}

#[cfg(test)]
mod tests {
    crate::should_render!(empty, "mj-body");
}
