use std::cell::{Ref, RefCell};
use std::rc::Rc;

use super::Mjml;
use crate::mj_head::MjHead;
use crate::prelude::render::{Error, Header, Render, RenderOptions, Renderable};

pub struct MjmlRender<'e, 'h> {
    header: Rc<RefCell<Header<'h>>>,
    element: &'e Mjml,
}

impl<'e, 'h> Render<'h> for MjmlRender<'e, 'h> {
    fn header(&self) -> Ref<Header<'h>> {
        self.header.borrow()
    }

    fn render(&self, opts: &RenderOptions) -> Result<String, Error> {
        let body_content = if let Some(body) = self.element.body() {
            body.renderer(Rc::clone(&self.header)).render(opts)?
        } else {
            String::from("<body></body>")
        };
        let mut buf = String::from("<!doctype html>");
        buf.push_str("<html ");
        if let Some(ref lang) = self.element.attributes.lang {
            buf.push_str("lang=\"");
            buf.push_str(lang);
            buf.push_str("\" ");
        }
        buf.push_str("xmlns=\"http://www.w3.org/1999/xhtml\" xmlns:v=\"urn:schemas-microsoft-com:vml\" xmlns:o=\"urn:schemas-microsoft-com:office:office\">");
        if let Some(head) = self.element.head() {
            buf.push_str(&head.renderer(Rc::clone(&self.header)).render(opts)?);
        } else {
            buf.push_str(
                &MjHead::default()
                    .renderer(Rc::clone(&self.header))
                    .render(opts)?,
            );
        }
        buf.push_str(&body_content);
        buf.push_str("</html>");
        Ok(buf)
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for Mjml {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'h> + 'r> {
        Box::new(MjmlRender::<'e, 'h> {
            element: self,
            header,
        })
    }
}

impl Mjml {
    pub fn render(&self, opts: &RenderOptions) -> Result<String, Error> {
        let mut header = Header::new(&self.children.head);
        header.maybe_set_lang(self.attributes.lang.clone());
        let header = Rc::new(RefCell::new(header));
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

#[cfg(all(test, feature = "parse"))]
mod tests {
    use crate::mjml::Mjml;
    use crate::prelude::render::RenderOptions;

    crate::should_render!(empty, "mjml");

    #[test]
    fn template_amario() {
        let opts = RenderOptions::default();
        let template = include_str!("../../resources/template/amario.mjml");
        let root = Mjml::parse(template).unwrap();
        assert!(root.render(&opts).is_ok());
    }

    #[test]
    fn template_air_astana() {
        let opts = RenderOptions::default();
        let template = include_str!("../../resources/template/air-astana.mjml");
        let expected = include_str!("../../resources/template/air-astana.html");
        let root = Mjml::parse(template).unwrap();
        html_compare::assert_similar(expected, root.render(&opts).unwrap().as_str());
    }

    #[test]
    #[cfg(feature = "orderedmap")]
    fn stable_output() {
        let source = "<mjml><mj-body><mj-section><mj-column><mj-text>hi</mj-text></mj-column></mj-section></mj-body></mjml>";
        let options = RenderOptions::default();

        let root_1 = Mjml::parse(source).unwrap();
        let root_2 = Mjml::parse(source).unwrap();

        let output_1 = root_1.render(&options).unwrap();
        let output_2 = root_2.render(&options).unwrap();

        assert_eq!(output_1, output_2);
    }
}
