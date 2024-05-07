use super::{MjSpacer, NAME};
use crate::helper::size::Pixel;
use crate::prelude::render::*;

impl<'root> Render<'root> for Renderer<'root, MjSpacer, ()> {
    fn default_attribute(&self, key: &str) -> Option<&'static str> {
        match key {
            "height" => Some("20px"),
            _ => None,
        }
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
        Tag::div()
            .maybe_add_style("height", self.attribute("height"))
            .maybe_add_style("line-height", self.attribute("height"))
            .render_text(&mut cursor.buffer, "&#8202;")
            .map_err(Error::from)
    }
}

impl<'render, 'root: 'render> Renderable<'render, 'root> for MjSpacer {
    fn renderer(
        &'root self,
        context: &'root RenderContext<'root>,
    ) -> Box<dyn Render<'root> + 'render> {
        Box::new(Renderer::new(context, self, ()))
    }
}

#[cfg(test)]
mod tests {
    crate::should_render!(basic, "mj-spacer");
}
