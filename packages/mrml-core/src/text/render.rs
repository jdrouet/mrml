use std::cell::{Ref, RefCell};
use std::rc::Rc;

use super::Text;
use crate::prelude::render::{Error, Header, Render, RenderBuffer, RenderOptions, Renderable};

struct TextRender<'e, 'h> {
    header: Rc<RefCell<Header<'h>>>,
    element: &'e Text,
}

impl<'e, 'h> Render<'e, 'h> for TextRender<'e, 'h> {
    fn header(&self) -> Ref<Header<'h>> {
        self.header.borrow()
    }

    fn render(&self, _opts: &RenderOptions, buf: &mut RenderBuffer) -> Result<(), Error> {
        buf.push_str(self.element.inner_str());
        Ok(())
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for Text {
    fn is_raw(&'e self) -> bool {
        true
    }

    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'e, 'h> + 'r> {
        Box::new(TextRender::<'e, 'h> {
            element: self,
            header,
        })
    }
}
