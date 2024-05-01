use crate::prelude::render::*;

impl<'root> Render<'root> for Renderer<'root, super::Root, ()> {
    fn context(&self) -> &'root RenderContext<'root> {
        self.context
    }

    fn render(&self, cursor: &mut RenderCursor) -> Result<(), Error> {
        for element in self.element.as_ref().iter() {
            match element {
                super::RootChild::Comment(inner) => inner.renderer(self.context).render(cursor)?,
                super::RootChild::Mjml(inner) => inner.renderer(self.context).render(cursor)?,
            };
        }
        Ok(())
    }
}

impl<'render, 'root: 'render> Renderable<'render, 'root> for super::Root {
    fn renderer(
        &'root self,
        context: &'root RenderContext<'root>,
    ) -> Box<dyn Render<'root> + 'render> {
        Box::new(Renderer::new(context, self, ()))
    }
}
