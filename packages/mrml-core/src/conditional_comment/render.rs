use super::ConditionalComment;
use crate::prelude::render::*;

impl<'root> Render<'root> for Renderer<'root, ConditionalComment, ()> {
    fn context(&self) -> &'root RenderContext<'root> {
        self.context
    }

    fn render(&self, cursor: &mut RenderCursor) -> Result<(), Error> {
        cursor.buffer.push_str(self.element.inner_str());
        Ok(())
    }
}

impl<'render, 'root: 'render> Renderable<'render, 'root> for ConditionalComment {
    fn renderer(
        &'root self,
        context: &'root RenderContext<'root>,
    ) -> Box<dyn Render<'root> + 'render> {
        Box::new(Renderer::new(context, self, ()))
    }
}

#[cfg(all(test, feature = "parse"))]
mod tests {
    use crate::conditional_comment::ConditionalComment;
    use crate::mj_head::MjHead;
    use crate::mjml::Mjml;
    use crate::prelude::render::{Header, RenderContext, RenderCursor, RenderOptions, Renderable};

    #[test]
    fn render_fails_without_mj_raw() {
        let result = Mjml::parse(
            r#"<mjml><mj-body><!--[if mso]><span>SpanContent</span><![endif]--></mj-body></mjml>"#,
        );
        assert!(
            matches!(
                result,
                Err(crate::prelude::parser::Error::UnexpectedToken { .. })
            ),
            "Expected UnexpectedToken error (conditional comment not being inside mj-raw)",
        );
    }

    #[test]
    fn render_when_inside_mj_raw() {
        let opts = RenderOptions::default();
        let root = Mjml::parse(r#"<mjml><mj-body><mj-raw><!--[if mso]><span>SpanContent</span><![endif]--></mj-raw></mj-body></mjml>"#).unwrap();
        let result = root.element.render(&opts).unwrap();
        assert!(result.contains("<!--[if mso]><span>SpanContent</span><![endif]-->"));
    }

    #[test]
    fn render_inner_comments_are_removed_when_disabled() {
        let opts = RenderOptions {
            disable_comments: true,
            ..Default::default()
        };
        let root = Mjml::parse(r#"<mjml><mj-body><mj-raw><!--[if mso]><!-- Comment --><span>SpanContent</span><![endif]--></mj-raw></mj-body></mjml>"#).unwrap();
        let result = root.element.render(&opts).unwrap();
        assert!(result.contains("<!--[if mso]><span>SpanContent</span><![endif]-->"));
    }

    #[test]
    fn render_context() {
        let opts = RenderOptions::default();
        let mj_head = Some(MjHead::default());
        let header = Header::new(mj_head.as_ref(), None);
        let context = RenderContext::new(&opts, header);

        let content = "<!--[if mso]><span>Test Content</span><![endif]-->";
        let elt = ConditionalComment::from(content);
        let renderer = elt.renderer(&context);

        assert!(std::ptr::eq(renderer.context(), &context));

        let mut cursor = RenderCursor::default();
        renderer.render(&mut cursor).unwrap();

        assert_eq!(cursor.buffer.as_ref(), content);
    }
}
