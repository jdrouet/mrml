use super::MJML;
use crate::mj_head::MJHead;
use crate::prelude::render::{Error, Header, Render, Renderable};
use std::cell::{Ref, RefCell};
use std::rc::Rc;

pub struct MJMLRender<'e, 'h> {
    header: Rc<RefCell<Header<'h>>>,
    element: &'e MJML,
}

impl<'e, 'h> Render<'h> for MJMLRender<'e, 'h> {
    fn header(&self) -> Ref<Header<'h>> {
        self.header.borrow()
    }

    fn render(&self) -> Result<String, Error> {
        let body_content = if let Some(body) = self.element.body() {
            body.renderer(Rc::clone(&self.header)).render()?
        } else {
            String::from("<body></body>")
        };
        let mut buf = String::from("<!doctype html>");
        buf.push_str("<html xmlns=\"http://www.w3.org/1999/xhtml\" xmlns:v=\"urn:schemas-microsoft-com:vml\" xmlns:o=\"urn:schemas-microsoft-com:office:office\">");
        if let Some(head) = self.element.head() {
            buf.push_str(&head.renderer(Rc::clone(&self.header)).render()?);
        } else {
            buf.push_str(
                &MJHead::default()
                    .renderer(Rc::clone(&self.header))
                    .render()?,
            );
        }
        buf.push_str(&body_content);
        buf.push_str("</html>");
        Ok(buf)
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MJML {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'h> + 'r> {
        Box::new(MJMLRender::<'e, 'h> {
            element: self,
            header,
        })
    }
}

impl MJML {
    pub fn render(&self) -> Result<String, Error> {
        let header = Rc::new(RefCell::new(Header::new(&self.head)));
        self.renderer(header).render()
    }
}

#[cfg(test)]
mod tests {
    use crate::helper::test::cleanup;
    use crate::mjml::MJML;

    #[test]
    fn empty() {
        let template = include_str!("../../resources/compare/success/mjml.mjml");
        let expected = include_str!("../../resources/compare/success/mjml.html");
        let root = MJML::parse(template.to_string()).unwrap();
        let result = cleanup(root.render().unwrap().as_str());
        let expected = cleanup(expected);
        assert_eq!(result, expected);
    }
}
