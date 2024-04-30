use crate::prelude::render::*;

pub(crate) struct RootRender<'e, 'h> {
    context: &'h RenderContext<'h>,
    element: &'e super::Root,
}

impl<'e, 'h> Render<'e, 'h> for RootRender<'e, 'h> {
    fn context(&self) -> &'h RenderContext<'h> {
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
        Box::new(RootRender::<'e, 'h> {
            element: self,
            context,
        })
    }
}
