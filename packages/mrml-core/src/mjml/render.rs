use super::Mjml;
use crate::mj_head::MjHead;
use crate::prelude::render::*;

impl<'root> Render<'root> for Renderer<'root, Mjml, ()> {
    fn context(&self) -> &'root RenderContext<'root> {
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
        cursor.buffer.open_tag("html");
        if let Some(ref lang) = self.element.attributes.lang {
            cursor.buffer.push_attribute("lang", lang.as_str())?;
        }
        cursor
            .buffer
            .push_attribute("xmlns", "http://www.w3.org/1999/xhtml")?;
        cursor
            .buffer
            .push_attribute("xmlns:v", "urn:schemas-microsoft-com:vml")?;
        cursor
            .buffer
            .push_attribute("xmlns:o", "urn:schemas-microsoft-com:office:office")?;
        cursor.buffer.close_tag();
        if let Some(head) = self.element.head() {
            head.renderer(self.context).render(cursor)?;
        } else {
            MjHead::default().renderer(self.context).render(cursor)?;
        }
        cursor.buffer.push_str(body.as_ref());
        cursor.buffer.end_tag("html");
        Ok(())
    }
}

impl<'render, 'root: 'render> Renderable<'render, 'root> for Mjml {
    fn renderer(
        &'root self,
        context: &'root RenderContext<'root>,
    ) -> Box<dyn Render<'root> + 'render> {
        Box::new(Renderer::new(context, self, ()))
    }
}

impl Mjml {
    pub fn render(&self, opts: &RenderOptions) -> Result<String, Error> {
        let header = Header::new(self.children.head.as_ref(), self.attributes.lang.as_deref());
        let context = RenderContext::new(opts, header);
        let mut cursor = RenderCursor::default();
        self.renderer(&context).render(&mut cursor)?;

        let html = String::from(cursor.buffer.as_ref());

        // Only inline CSS if there are inline styles
        if cursor.header.has_inline_styles() {
            match css_inline::inline(&html) {
                Ok(inlined_html) => return Ok(inlined_html),
                Err(_) => {}
            }
        }

        Ok(html)
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
    use crate::prelude::render::{Header, RenderContext, RenderCursor, RenderOptions, Renderable};

    crate::should_render!(empty, "mjml");

    #[test]
    fn template_amario() {
        let opts = RenderOptions::default();
        let template = include_str!("../../resources/template/amario.mjml");
        let root = Mjml::parse(template).unwrap();
        assert!(root.element.render(&opts).is_ok());
    }

    #[test]
    fn template_air_astana() {
        let opts = RenderOptions::default();
        let template = include_str!("../../resources/template/air-astana.mjml");
        let expected = include_str!("../../resources/template/air-astana.html");
        let root = Mjml::parse(template).unwrap();
        html_compare::assert_similar(expected, root.element.render(&opts).unwrap().as_str());
    }

    #[test]
    fn stable_output() {
        let source = "<mjml><mj-body><mj-section><mj-column><mj-text>hi</mj-text></mj-column></mj-section></mj-body></mjml>";
        let options = RenderOptions::default();

        let root_1 = Mjml::parse(source).unwrap();
        let root_2 = Mjml::parse(source).unwrap();

        let output_1 = root_1.element.render(&options).unwrap();
        let output_2 = root_2.element.render(&options).unwrap();

        assert_eq!(output_1, output_2);
    }

    #[test]
    fn test_css_inlining() {
        // Simple template with inline style
        let source_with_inline = r#"<mjml>
            <mj-head>
                <mj-style inline="true">
                    .blue { color: blue; }
                </mj-style>
            </mj-head>
            <mj-body>
                <mj-section>
                    <mj-column>
                        <mj-text><p class="blue">Blue text</p></mj-text>
                    </mj-column>
                </mj-section>
            </mj-body>
        </mjml>"#;

        // Same template without inline style
        let source_without_inline = r#"<mjml>
            <mj-head>
                <mj-style>
                    .blue { color: blue; }
                </mj-style>
            </mj-head>
            <mj-body>
                <mj-section>
                    <mj-column>
                        <mj-text><p class="blue">Blue text</p></mj-text>
                    </mj-column>
                </mj-section>
            </mj-body>
        </mjml>"#;

        let options = RenderOptions::default();
        let output_with_inline = Mjml::parse(source_with_inline)
            .unwrap()
            .element
            .render(&options)
            .unwrap();
        let output_without_inline = Mjml::parse(source_without_inline)
            .unwrap()
            .element
            .render(&options)
            .unwrap();

        // Check that CSS inlining happened with inline styles
        assert!(
            output_with_inline.contains("style=\"color: blue;\""),
            "CSS inlining should happen when inline styles are present"
        );

        // Check that CSS inlining did not happen without inline styles
        assert!(
            !output_without_inline.contains("style=\"color: blue;\""),
            "CSS inlining should not happen when inline styles are not present"
        );
        assert!(
            output_without_inline.contains(".blue { color: blue; }"),
            "Original style should remain in the output without inline styles"
        );
    }
}
