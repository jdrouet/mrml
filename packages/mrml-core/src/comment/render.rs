use super::Comment;
use crate::prelude::render::*;

struct CommentRender<'e, 'h> {
    context: &'h RenderContext<'h>,
    element: &'e Comment,
}

impl<'e, 'h> Render<'e, 'h> for CommentRender<'e, 'h> {
    fn context(&self) -> &'h RenderContext<'h> {
        self.context
    }

    fn render(&self, cursor: &mut RenderCursor) -> Result<(), Error> {
        if !self.context.options.disable_comments {
            cursor.buffer.push_str("<!--");
            cursor.buffer.push_str(self.element.children.as_str());
            cursor.buffer.push_str("-->");
        }
        Ok(())
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for Comment {
    fn is_raw(&'e self) -> bool {
        true
    }

    fn renderer(&'e self, context: &'h RenderContext<'h>) -> Box<dyn Render<'e, 'h> + 'r> {
        Box::new(CommentRender::<'e, 'h> {
            element: self,
            context,
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
