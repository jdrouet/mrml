use super::{MjRaw, MjRawChild, NAME};
use crate::helper::size::Pixel;
use crate::prelude::render::*;

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MjRawChild {
    fn renderer(&'e self, context: &'h RenderContext<'h>) -> Box<dyn Render<'e, 'h> + 'r> {
        match self {
            Self::Comment(elt) => elt.renderer(context),
            Self::Node(elt) => elt.renderer(context),
            Self::Text(elt) => elt.renderer(context),
        }
    }
}

#[derive(Default)]
struct MjRawExtra {
    container_width: Option<Pixel>,
}

impl<'element, 'header> Render<'element, 'header>
    for Renderer<'element, 'header, MjRaw, MjRawExtra>
{
    fn tag(&self) -> Option<&str> {
        Some(NAME)
    }

    fn context(&self) -> &'header RenderContext<'header> {
        self.context
    }

    fn set_container_width(&mut self, width: Option<Pixel>) {
        self.extra.container_width = width;
    }

    fn render(&self, cursor: &mut RenderCursor) -> Result<(), Error> {
        let siblings = self.element.children.len();
        for (index, child) in self.element.children.iter().enumerate() {
            let mut renderer = child.renderer(self.context());
            renderer.set_index(index);
            renderer.set_siblings(siblings);
            renderer.set_raw_siblings(siblings);
            renderer.set_container_width(self.extra.container_width.clone());
            renderer.render(cursor)?;
        }
        Ok(())
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MjRaw {
    fn renderer(&'e self, context: &'h RenderContext<'h>) -> Box<dyn Render<'e, 'h> + 'r> {
        Box::new(Renderer::new(context, self, MjRawExtra::default()))
    }
}

#[cfg(test)]
mod tests {
    crate::should_render!(basic, "mj-raw");
    crate::should_render!(in_head, "mj-raw-head");
}
