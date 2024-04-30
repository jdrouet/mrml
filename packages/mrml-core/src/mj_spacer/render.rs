use super::{MjSpacer, NAME};
use crate::helper::size::Pixel;
use crate::prelude::render::*;

struct MjSpacerRender<'e, 'h> {
    context: &'h RenderContext<'h>,
    element: &'e MjSpacer,
    container_width: Option<Pixel>,
}

impl<'e, 'h> Render<'e, 'h> for MjSpacerRender<'e, 'h> {
    fn default_attribute(&self, key: &str) -> Option<&'static str> {
        match key {
            "height" => Some("20px"),
            _ => None,
        }
    }

    fn raw_attribute(&self, key: &str) -> Option<&'e str> {
        self.element.attributes.get(key).map(|v| v.as_str())
    }

    fn tag(&self) -> Option<&str> {
        Some(NAME)
    }

    fn set_container_width(&mut self, width: Option<Pixel>) {
        self.container_width = width;
    }

    fn context(&self) -> &'h RenderContext<'h> {
        self.context
    }

    fn render(&self, _header: &mut VariableHeader, buf: &mut RenderBuffer) -> Result<(), Error> {
        Tag::div()
            .maybe_add_style("height", self.attribute("height"))
            .maybe_add_style("line-height", self.attribute("height"))
            .render_text(buf, "&#8202;");
        Ok(())
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MjSpacer {
    fn renderer(&'e self, context: &'h RenderContext<'h>) -> Box<dyn Render<'e, 'h> + 'r> {
        Box::new(MjSpacerRender::<'e, 'h> {
            element: self,
            context,
            container_width: None,
        })
    }
}

#[cfg(test)]
mod tests {
    crate::should_render!(basic, "mj-spacer");
}
