use super::Comment;
use crate::prelude::render::*;

struct CommentRender<'e, 'h> {
    header: &'h Header<'h>,
    element: &'e Comment,
}

impl<'e, 'h> Render<'e, 'h> for CommentRender<'e, 'h> {
    fn header(&self) -> &'h Header<'h> {
        self.header
    }

    fn render(
        &self,
        opts: &RenderOptions,
        _header: &mut VariableHeader,
        buf: &mut RenderBuffer,
    ) -> Result<(), Error> {
        if !opts.disable_comments {
            buf.push_str("<!--");
            buf.push_str(self.element.children.as_str());
            buf.push_str("-->");
        }
        Ok(())
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for Comment {
    fn is_raw(&'e self) -> bool {
        true
    }

    fn renderer(&'e self, header: &'h Header<'h>) -> Box<dyn Render<'e, 'h> + 'r> {
        Box::new(CommentRender::<'e, 'h> {
            element: self,
            header,
        })
    }
}

#[cfg(all(test, feature = "parse"))]
mod tests {
    use crate::mjml::Mjml;
    use crate::prelude::render::RenderOptions;

    #[test]
    fn render_enabled() {
        let opts = RenderOptions::default();
        let root = Mjml::parse(r#"<mjml><mj-body><!-- Hello World! --></mj-body></mjml>"#).unwrap();
        let result = root.render(&opts).unwrap();
        assert!(result.contains("Hello World!"));
    }

    #[test]
    fn render_disabled() {
        let opts = RenderOptions {
            disable_comments: true,
            ..Default::default()
        };
        let root = Mjml::parse(r#"<mjml><mj-body><!-- Hello World! --></mj-body></mjml>"#).unwrap();
        let result = root.render(&opts).unwrap();
        assert!(!result.contains("Hello World!"));
    }

    #[test]
    fn render_with_is_raw() {
        let opts = RenderOptions::default();
        let root = Mjml::parse(r#"<mjml><mj-body><mj-section><mj-column><!-- Hello World! --></mj-column></mj-section></mj-body></mjml>"#).unwrap();
        let result = root.render(&opts).unwrap();
        assert!(result.contains("Hello World!"));
    }
}
