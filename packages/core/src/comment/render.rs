use super::Comment;
use crate::prelude::render::{Error, Header, Render, Renderable};
use std::cell::{Ref, RefCell};
use std::rc::Rc;

struct CommentRender<'e, 'h> {
    header: Rc<RefCell<Header<'h>>>,
    element: &'e Comment,
}

impl<'e, 'h> Render<'h> for CommentRender<'e, 'h> {
    fn header(&self) -> Ref<Header<'h>> {
        self.header.borrow()
    }

    fn render(&self, buf: &mut String) -> Result<(), Error> {
        buf.push_str("<!--");
        buf.push_str(&self.element.0);
        buf.push_str("-->");
        Ok(())
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for Comment {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'h> + 'r> {
        Box::new(CommentRender::<'e, 'h> {
            element: self,
            header,
        })
    }
}
