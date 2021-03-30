use super::Node;
use crate::prelude::render::{Error, Header, Render, Renderable};
use std::cell::{Ref, RefCell};
use std::rc::Rc;

struct NodeRender<'e, 'h> {
    header: Rc<RefCell<Header<'h>>>,
    element: &'e Node,
}

impl<'e, 'h> Render<'h> for NodeRender<'e, 'h> {
    fn tag(&self) -> Option<&str> {
        Some(self.element.tag.as_str())
    }

    fn header(&self) -> Ref<Header<'h>> {
        self.header.borrow()
    }

    fn render(&self) -> Result<String, Error> {
        let mut buf = String::from("<");
        buf.push_str(&self.element.tag);
        // print_attributes(buf, Some(&self.element.attributes));
        if self.element.children.is_empty() {
            buf.push_str(" />");
        } else {
            buf.push_str(">");
            for (index, child) in self.element.children.iter().enumerate() {
                // TODO children
                let mut renderer = child.renderer(Rc::clone(&self.header));
                renderer.set_index(index);
                buf.push_str(&renderer.render()?);
            }
            buf.push_str("</");
            buf.push_str(&self.element.tag);
            buf.push_str(">");
        }
        Ok(buf)
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for Node {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'h> + 'r> {
        Box::new(NodeRender::<'e, 'h> {
            element: self,
            header,
        })
    }
}
