use super::Mjml;
use crate::mj_head::MjHead;
use crate::prelude::render::*;

pub struct MjmlRender<'e, 'h> {
    context: &'h RenderContext<'h>,
    element: &'e Mjml,
}

impl<'e, 'h> Render<'e, 'h> for MjmlRender<'e, 'h> {
    fn context(&self) -> &'h RenderContext<'h> {
        self.context
    }

    fn render(&self, header: &mut VariableHeader, buf: &mut RenderBuffer) -> Result<(), Error> {
        let mut body_buf = RenderBuffer::default();
        if let Some(body) = self.element.body() {
            body.renderer(self.context).render(header, &mut body_buf)?;
        } else {
            body_buf.push_str("<body></body>");
        }
        buf.push_str("<!doctype html>");
        buf.push_str("<html ");
        if let Some(ref lang) = self.element.attributes.lang {
            buf.push_str("lang=\"");
            buf.push_str(lang);
            buf.push_str("\" ");
        }
        buf.push_str("xmlns=\"http://www.w3.org/1999/xhtml\" xmlns:v=\"urn:schemas-microsoft-com:vml\" xmlns:o=\"urn:schemas-microsoft-com:office:office\">");
        if let Some(head) = self.element.head() {
            head.renderer(self.context).render(header, buf)?;
        } else {
            MjHead::default()
                .renderer(self.context)
                .render(header, buf)?;
        }
        buf.push_str(body_buf.as_ref());
        buf.push_str("</html>");
        Ok(())
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for Mjml {
    fn renderer(&'e self, context: &'h RenderContext<'h>) -> Box<dyn Render<'e, 'h> + 'r> {
        Box::new(MjmlRender::<'e, 'h> {
            element: self,
            context,
        })
    }
}

impl Mjml {
    pub fn render(&self, opts: &RenderOptions) -> Result<String, Error> {
        let header = Header::new(self.children.head.as_ref(), self.attributes.lang.as_deref());
        let context = RenderContext::new(opts, header);
        let mut vheader = VariableHeader::default();
        let mut buf = RenderBuffer::default();
        self.renderer(&context).render(&mut vheader, &mut buf)?;
        Ok(buf.into())
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
