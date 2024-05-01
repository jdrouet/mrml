use super::{MjRaw, MjRawChild, NAME};
use crate::helper::size::Pixel;
use crate::prelude::render::*;

impl<'render, 'root: 'render> Renderable<'render, 'root> for MjRawChild {
    fn renderer(
        &'root self,
        context: &'root RenderContext<'root>,
    ) -> Box<dyn Render<'root> + 'render> {
        match self {
            Self::Comment(elt) => elt.renderer(context),
            Self::Node(elt) => elt.renderer(context),
            Self::Text(elt) => elt.renderer(context),
        }
    }
}

impl<'root> Render<'root> for Renderer<'root, MjRaw, ()> {
    fn tag(&self) -> Option<&str> {
        Some(NAME)
    }

    fn context(&self) -> &'root RenderContext<'root> {
        self.context
    }

    fn set_container_width(&mut self, width: Option<Pixel>) {
        self.container_width = width;
    }

    fn render(&self, cursor: &mut RenderCursor) -> Result<(), Error> {
        let siblings = self.element.children.len();
        for (index, child) in self.element.children.iter().enumerate() {
            let mut renderer = child.renderer(self.context());
            renderer.set_index(index);
            renderer.set_siblings(siblings);
            renderer.set_raw_siblings(siblings);
            renderer.set_container_width(self.container_width.clone());
            renderer.render(cursor)?;
        }
        Ok(())
    }
}

impl<'render, 'root: 'render> Renderable<'render, 'root> for MjRaw {
    fn renderer(
        &'root self,
        context: &'root RenderContext<'root>,
    ) -> Box<dyn Render<'root> + 'render> {
        Box::new(Renderer::new(context, self, ()))
    }
}

#[cfg(test)]
mod tests {
    crate::should_render!(basic, "mj-raw");
    crate::should_render!(in_head, "mj-raw-head");
}
