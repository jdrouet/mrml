use super::Node;
use crate::prelude::print::print_attributes;
use crate::prelude::render::{Error, Header, Render, Renderable};

struct NodeRender<'e> {
    element: &'e Node,
    header: &'e mut Header<'e>,
}

impl<'e> NodeRender<'e> {
    fn new(element: &'e Node, header: &'e mut Header<'e>) -> Self {
        Self { element, header }
    }
}

impl<'e> Render<'e> for NodeRender<'e> {
    fn render(&'e self, buf: &mut String) -> Result<(), Error> {
        buf.push_str("<");
        buf.push_str(&self.element.tag);
        print_attributes(buf, Some(&self.element.attributes));
        if self.element.children.is_empty() {
            buf.push_str(" />");
        } else {
            buf.push_str(">");
            for child in self.element.children {
                // TODO children
                child.renderer(self.header).render(buf);
            }
            buf.push_str("</");
            buf.push_str(&self.element.tag);
            buf.push_str(">");
        }
        Ok(())
    }
}

impl<'e> Renderable<'e> for Node {
    fn renderer(&'e self, header: &'e mut Header<'e>) -> Box<dyn Render<'e> + 'e> {
        Box::new(NodeRender::new(self, header))
    }
}
