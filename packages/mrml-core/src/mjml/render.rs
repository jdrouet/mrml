use super::Mjml;
use crate::mj_head::MjHead;
use crate::prelude::render::*;

impl<'element, 'header> Render<'element, 'header> for Renderer<'element, 'header, Mjml, ()> {
    fn context(&self) -> &'header RenderContext<'header> {
        self.context
    }

    fn render(&self, cursor: &mut RenderCursor) -> Result<(), Error> {
        if let Some(body) = self.element.body() {
            body.renderer(self.context).render(cursor)?;
        } else {
            cursor.buffer.push_str("<body></body>");
        }
        let mut body = RenderBuffer::default();
        std::mem::swap(&mut body, &mut cursor.buffer);
        cursor.buffer.push_str("<!doctype html>");
        cursor.buffer.push_str("<html ");
        if let Some(ref lang) = self.element.attributes.lang {
            cursor.buffer.push_str("lang=\"");
            cursor.buffer.push_str(lang);
            cursor.buffer.push_str("\" ");
        }
        cursor.buffer.push_str("xmlns=\"http://www.w3.org/1999/xhtml\" xmlns:v=\"urn:schemas-microsoft-com:vml\" xmlns:o=\"urn:schemas-microsoft-com:office:office\">");
        if let Some(head) = self.element.head() {
            head.renderer(self.context).render(cursor)?;
        } else {
            MjHead::default().renderer(self.context).render(cursor)?;
        }
        cursor.buffer.push_str(body.as_ref());
        cursor.buffer.push_str("</html>");
        Ok(())
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for Mjml {
    fn renderer(&'e self, context: &'h RenderContext<'h>) -> Box<dyn Render<'e, 'h> + 'r> {
        Box::new(Renderer::new(context, self, ()))
    }
}

impl Mjml {
    pub fn render(&self, opts: &RenderOptions) -> Result<String, Error> {
        let header = Header::new(self.children.head.as_ref(), self.attributes.lang.as_deref());
        let context = RenderContext::new(opts, header);
        let mut cursor = RenderCursor::default();
        self.renderer(&context).render(&mut cursor)?;
        Ok(cursor.buffer.into())
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
