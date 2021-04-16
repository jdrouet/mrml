use super::Comment;
use crate::prelude::render::{Error, Header, Options, Render, Renderable};
use std::cell::{Ref, RefCell};
use std::rc::Rc;

struct CommentRender<'e, 'h> {
    header: Rc<RefCell<Header<'h>>>,
    element: &'e Comment,
}

impl<'e, 'h> Render<'h> for CommentRender<'e, 'h> {
    fn header(&self) -> Ref<Header<'h>> {
        self.header.borrow()
    }

    fn render(&self, opts: &Options) -> Result<String, Error> {
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

#[cfg(test)]
mod tests {
    use crate::mjml::MJML;
    use crate::prelude::render::Options;

    #[test]
    fn render_enabled() {
        let opts = Options::default();
        let root = MJML::parse(r#"<mjml><mj-body><!-- Hello World! --></mj-body></mjml>"#).unwrap();
        let result = root.render(&opts).unwrap();
        assert!(result.contains("Hello World!"));
    }

    #[test]
    fn render_disabled() {
        let mut opts = Options::default();
        opts.disable_comments = true;
        let root = MJML::parse(r#"<mjml><mj-body><!-- Hello World! --></mj-body></mjml>"#).unwrap();
        let result = root.render(&opts).unwrap();
        assert!(!result.contains("Hello World!"));
    }

    #[test]
    fn render_with_is_raw() {
        let opts = Options::default();
        let root = MJML::parse(r#"<mjml><mj-body><mj-section><mj-column><!-- Hello World! --></mj-column></mj-section></mj-body></mjml>"#).unwrap();
        let result = root.render(&opts).unwrap();
        assert!(result.contains("Hello World!"));
    }
}
