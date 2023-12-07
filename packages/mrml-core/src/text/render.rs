use std::cell::{Ref, RefCell};
use std::rc::Rc;

use super::Text;
use crate::prelude::render::{Error, Header, Render, RenderOptions, Renderable};

struct TextRender<'e, 'h> {
    header: Rc<RefCell<Header<'h>>>,
    element: &'e Text,
}

impl<'e, 'h> Render<'h> for TextRender<'e, 'h> {
    fn header(&self) -> Ref<Header<'h>> {
        self.header.borrow()
    }

    fn render(&self, _opts: &RenderOptions) -> Result<String, Error> {
        Ok(self.element.0.clone())
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for Text {
    fn is_raw(&'e self) -> bool {
        true
    }

    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'h> + 'r> {
        Box::new(TextRender::<'e, 'h> {
            element: self,
            header,
        })
    }
}
