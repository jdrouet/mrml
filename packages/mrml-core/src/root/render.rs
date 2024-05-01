use crate::prelude::render::*;

impl<'element, 'header> Render<'element, 'header> for Renderer<'element, 'header, super::Root, ()> {
    fn context(&self) -> &'header RenderContext<'header> {
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

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for super::Root {
    fn renderer(&'e self, context: &'h RenderContext<'h>) -> Box<dyn Render<'e, 'h> + 'r> {
        Box::new(Renderer::new(context, self, ()))
    }
}
