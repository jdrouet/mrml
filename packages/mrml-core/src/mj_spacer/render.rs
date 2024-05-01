use super::{MjSpacer, NAME};
use crate::helper::size::Pixel;
use crate::prelude::render::*;

#[derive(Default)]
struct MjSpacerExtra {
    container_width: Option<Pixel>,
}

impl<'element, 'header> Render<'element, 'header>
    for Renderer<'element, 'header, MjSpacer, MjSpacerExtra>
{
    fn default_attribute(&self, key: &str) -> Option<&'static str> {
        match key {
            "height" => Some("20px"),
            _ => None,
        }
    }

    fn raw_attribute(&self, key: &str) -> Option<&'element str> {
        self.element.attributes.get(key).map(|v| v.as_str())
    }

    fn tag(&self) -> Option<&str> {
        Some(NAME)
    }

    fn set_container_width(&mut self, width: Option<Pixel>) {
        self.extra.container_width = width;
    }

    fn context(&self) -> &'header RenderContext<'header> {
        self.context
    }

    fn render(&self, cursor: &mut RenderCursor) -> Result<(), Error> {
        Tag::div()
            .maybe_add_style("height", self.attribute("height"))
            .maybe_add_style("line-height", self.attribute("height"))
            .render_text(&mut cursor.buffer, "&#8202;");
        Ok(())
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MjSpacer {
    fn renderer(&'e self, context: &'h RenderContext<'h>) -> Box<dyn Render<'e, 'h> + 'r> {
        Box::new(Renderer::new(context, self, MjSpacerExtra::default()))
    }
}

#[cfg(test)]
mod tests {
    crate::should_render!(basic, "mj-spacer");
}
