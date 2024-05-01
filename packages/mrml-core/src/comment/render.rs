use super::Comment;
use crate::prelude::render::*;

impl<'root> Render<'root> for Renderer<'root, Comment, ()> {
    fn context(&self) -> &'root RenderContext<'root> {
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

impl<'render, 'root: 'render> Renderable<'render, 'root> for Comment {
    fn is_raw(&self) -> bool {
        true
    }

    fn renderer(
        &'root self,
        context: &'root RenderContext<'root>,
    ) -> Box<dyn Render<'root> + 'render> {
        Box::new(Renderer::new(context, self, ()))
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
