use std::cell::{Ref, RefCell};
use std::rc::Rc;

use crate::prelude::render::{Error, Header, Render, RenderBuffer, RenderOptions, Renderable};

pub(crate) struct RootRender<'e, 'h> {
    header: Rc<RefCell<Header<'h>>>,
    element: &'e super::Root,
}

impl<'e, 'h> Render<'e, 'h> for RootRender<'e, 'h> {
    fn header(&self) -> Ref<Header<'h>> {
        self.header.borrow()
    }

    fn render(&self, opts: &RenderOptions, buf: &mut RenderBuffer) -> Result<(), Error> {
        for element in self.element.as_ref().iter() {
            match element {
                super::RootChild::Comment(inner) => {
                    inner.renderer(self.header.clone()).render(opts, buf)?
                }
                super::RootChild::Mjml(inner) => {
                    inner.renderer(self.header.clone()).render(opts, buf)?
                }
            };
        }
        Ok(())
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for super::Root {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'e, 'h> + 'r> {
        Box::new(RootRender::<'e, 'h> {
            element: self,
            header,
        })
    }
}
