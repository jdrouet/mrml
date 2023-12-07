use std::cell::{Ref, RefCell};
use std::rc::Rc;

use super::Node;
use crate::prelude::render::{Error, Header, Render, RenderOptions, Renderable};

const SHOULD_SELF_CLOSE: [&str; 1] = ["script"];

struct NodeRender<'e, 'h, T> {
    header: Rc<RefCell<Header<'h>>>,
    element: &'e Node<T>,
}

impl<'r, 'e: 'r, 'h: 'r, T> Render<'h> for NodeRender<'e, 'h, T>
where
    T: Renderable<'r, 'e, 'h>,
{
    fn tag(&self) -> Option<&str> {
        Some(self.element.tag.as_str())
    }

    fn header(&self) -> Ref<Header<'h>> {
        self.header.borrow()
    }

    fn render(&self, opts: &RenderOptions) -> Result<String, Error> {
        let mut buf = String::from("<");
        buf.push_str(&self.element.tag);
        for (key, value) in self.element.attributes.iter() {
            buf.push(' ');
            buf.push_str(key);
            buf.push_str("=\"");
            buf.push_str(value);
            buf.push('"');
        }
        if self.element.children.is_empty()
            && !SHOULD_SELF_CLOSE.contains(&self.element.tag.as_str())
        {
            buf.push_str(" />");
        } else {
            buf.push('>');
            for (index, child) in self.element.children.iter().enumerate() {
                // TODO children
                let mut renderer = child.renderer(Rc::clone(&self.header));
                renderer.set_index(index);
                buf.push_str(&renderer.render(opts)?);
            }
            buf.push_str("</");
            buf.push_str(&self.element.tag);
            buf.push('>');
        }
        Ok(buf)
    }
}

impl<'r, 'e: 'r, 'h: 'r, T: Renderable<'r, 'e, 'h>> Renderable<'r, 'e, 'h> for Node<T> {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'h> + 'r> {
        Box::new(NodeRender::<'e, 'h> {
            element: self,
            header,
        })
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "parse")]
    #[test]
    fn empty_script_should_have_closing_element() {
        use crate::mjml::Mjml;
        use crate::prelude::render::RenderOptions;

        let opts = RenderOptions::default();
        let template = r#"<mjml>
    <mj-body>
        <mj-section>
        <mj-column>
            <mj-raw><script src="http://example.com/hello.js"></script></mj-raw>
            <mj-text>
            Hello World!
            </mj-text>
        </mj-column>
        </mj-section>
    </mj-body>
</mjml>"#;
        let root = Mjml::parse(template).unwrap();
        let result = root.render(&opts).unwrap();
        assert!(result.contains("<script src=\"http://example.com/hello.js\"></script>"));
    }
}
