use super::ConditionalComment;
use crate::prelude::render::*;

impl<'root> Render<'root> for Renderer<'root, ConditionalComment, ()> {
    fn context(&self) -> &'root RenderContext<'root> {
        self.context
    }

    fn render(&self, cursor: &mut RenderCursor) -> Result<(), Error> {
        cursor.buffer.push_str(self.element.inner_str());
        Ok(())
    }
}

impl<'render, 'root: 'render> Renderable<'render, 'root> for ConditionalComment {
    fn is_raw(&'root self) -> bool {
        true
    }

    fn renderer(
        &'root self,
        context: &'root RenderContext<'root>,
    ) -> Box<dyn Render<'root> + 'render> {
        Box::new(Renderer::new(context, self, ()))
    }
}
