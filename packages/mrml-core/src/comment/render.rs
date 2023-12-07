use std::cell::{Ref, RefCell};
use std::rc::Rc;

use super::Comment;
use crate::prelude::render::{Error, Header, Render, RenderOptions, Renderable};

struct CommentRender<'e, 'h> {
    header: Rc<RefCell<Header<'h>>>,
    element: &'e Comment,
}

impl<'e, 'h> Render<'h> for CommentRender<'e, 'h> {
    fn header(&self) -> Ref<Header<'h>> {
        self.header.borrow()
    }

    fn render(&self, opts: &RenderOptions) -> Result<String, Error> {
        if opts.disable_comments {
            Ok(String::default())
        } else {
            Ok(String::from("<!--") + &self.element.children + "-->")
        }
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for Comment {
    fn is_raw(&'e self) -> bool {
        true
    }

    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'h> + 'r> {
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
