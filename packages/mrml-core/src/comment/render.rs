use super::Comment;
use crate::prelude::render::{Error, Header, Options, Render, Renderable};
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

    fn render(&self, opts: &Options) -> Result<String, Error> {
        if opts.disable_comments {
            Ok(String::default())
        } else {
            Ok(String::from("<!--") + &self.element.0 + "-->")
        }
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for Comment {
    fn is_raw(&'e self) -> bool {
        true
    }

    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'h> + 'r> {
        Box::new(CommentRender::<'e, 'h> {
            element: self,
            header,
        })
    }
}
