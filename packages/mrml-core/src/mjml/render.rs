use super::MJML;
use crate::mj_head::MJHead;
use crate::prelude::render::{Error, Header, Options, Render, Renderable};
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

    fn render(&self, opts: &Options) -> Result<String, Error> {
        let body_content = if let Some(body) = self.element.body() {
            body.renderer(Rc::clone(&self.header)).render(opts)?
        } else {
            String::from("<body></body>")
        };
        let mut buf = String::from("<!doctype html>");
        buf.push_str("<html xmlns=\"http://www.w3.org/1999/xhtml\" xmlns:v=\"urn:schemas-microsoft-com:vml\" xmlns:o=\"urn:schemas-microsoft-com:office:office\">");
        if let Some(head) = self.element.head() {
            buf.push_str(&head.renderer(Rc::clone(&self.header)).render(opts)?);
        } else {
            buf.push_str(
                &MJHead::default()
                    .renderer(Rc::clone(&self.header))
                    .render(opts)?,
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
    pub fn render(&self, opts: &Options) -> Result<String, Error> {
        let header = Rc::new(RefCell::new(Header::new(&self.children.head)));
        self.renderer(header).render(opts)
    }

    pub fn get_title(&self) -> Option<String> {
        self.head()
            .and_then(|head| head.title())
            .map(|title| title.content().to_string())
    }

    pub fn get_preview(&self) -> Option<String> {
        self.head()
            .and_then(|head| head.preview())
            .map(|preview| preview.content().to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::helper::test::compare;
    use crate::mjml::MJML;
    use crate::prelude::render::Options;

    #[test]
    fn empty() {
        let opts = Options::default();
        let template = include_str!("../../resources/compare/success/mjml.mjml");
        let expected = include_str!("../../resources/compare/success/mjml.html");
        let root = MJML::parse(template.to_string()).unwrap();
        compare(expected, root.render(&opts).unwrap().as_str());
    }

    #[test]
    fn template_amario() {
        let opts = Options::default();
        let template = include_str!("../../resources/template/amario.mjml");
        let root = MJML::parse(template.to_string()).unwrap();
        assert!(root.render(&opts).is_ok());
    }
}
