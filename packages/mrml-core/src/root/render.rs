use crate::prelude::render::*;

pub(crate) struct RootRender<'e, 'h> {
    header: &'h Header<'h>,
    element: &'e super::Root,
}

impl<'e, 'h> Render<'e, 'h> for RootRender<'e, 'h> {
    fn header(&self) -> &'h Header<'h> {
        self.header
    }

    fn render(
        &self,
        opts: &RenderOptions,
        header: &mut VariableHeader,
        buf: &mut RenderBuffer,
    ) -> Result<(), Error> {
        for element in self.element.as_ref().iter() {
            match element {
                super::RootChild::Comment(inner) => {
                    inner.renderer(self.header).render(opts, header, buf)?
                }
                super::RootChild::Mjml(inner) => {
                    inner.renderer(self.header).render(opts, header, buf)?
                }
            };
        }
        Ok(())
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for super::Root {
    fn renderer(&'e self, header: &'h Header<'h>) -> Box<dyn Render<'e, 'h> + 'r> {
        Box::new(RootRender::<'e, 'h> {
            element: self,
            header,
        })
    }
}
