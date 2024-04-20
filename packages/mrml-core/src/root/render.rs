use std::cell::{Ref, RefCell};
use std::rc::Rc;

use crate::prelude::render::{Error, Header, Render, RenderOptions, Renderable};

pub struct RootRender<'e, 'h> {
    header: Rc<RefCell<Header<'h>>>,
    element: &'e super::Root,
}

impl<'e, 'h> Render<'h> for RootRender<'e, 'h> {
    fn header(&self) -> Ref<Header<'h>> {
        self.header.borrow()
    }

    fn render(&self, opts: &RenderOptions) -> Result<String, Error> {
        let mut result = String::new();
        for element in self.element.as_ref().iter() {
            let content = match element {
                super::RootChild::Comment(inner) => {
                    inner.renderer(self.header.clone()).render(opts)?
                }
                super::RootChild::Mjml(inner) => {
                    inner.renderer(self.header.clone()).render(opts)?
                }
            };
            result.push_str(&content);
        }
        Ok(result)
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for super::Root {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'h> + 'r> {
        Box::new(RootRender::<'e, 'h> {
            element: self,
            header,
        })
    }
}
