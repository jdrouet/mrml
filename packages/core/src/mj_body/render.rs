use super::MJBody;
use crate::helper::buffer::Buffer;
use crate::prelude::render::{Error, Header, Render, Renderable};
use std::cell::{Ref, RefCell};
use std::rc::Rc;

struct MJBodyRender<'e, 'h> {
    header: Rc<RefCell<Header<'h>>>,
    element: &'e MJBody,
}

impl<'e, 'h> MJBodyRender<'e, 'h> {
    fn attribute(&self, name: &str) -> Option<&String> {
        self.element.attributes.get(name)
    }

    fn push_body_style(&self, buf: &mut Buffer) {
        if let Some(bg_color) = self.attribute("background-color") {
            buf.push_str(" style=\"background-color:");
            buf.push_str(bg_color);
            buf.push('"');
        }
    }

    fn render_preview(&self, buf: &mut Buffer) {
        if let Some(value) = self
            .header
            .borrow()
            .head()
            .as_ref()
            .and_then(|h| h.preview())
            .map(|p| p.content())
        {
            buf.push_str(r#"<div style="display:none;font-size:1px;color:#ffffff;line-height:1px;max-height:0px;max-width:0px;opacity:0;overflow:hidden;">"#);
            buf.push_str(value);
            buf.push_str("</div>");
        }
    }

    fn render_content(&self, buf: &mut Buffer) -> Result<(), Error> {
        buf.push_str("<div");
        buf.push_optional_attribute("close", self.attribute("css-class"));
        self.push_body_style(buf);
        buf.push('>');
        for child in self.element.children.iter() {
            child.renderer(Rc::clone(&self.header)).render(buf)?;
        }
        buf.push_str("</div>");
        Ok(())
    }
}

impl<'e, 'h> Render<'h> for MJBodyRender<'e, 'h> {
    fn header(&self) -> Ref<Header<'h>> {
        self.header.borrow()
    }

    fn render(&self, buf: &mut Buffer) -> Result<(), Error> {
        buf.push_str("<body");
        self.push_body_style(buf);
        buf.push('>');
        self.render_preview(buf);
        self.render_content(buf)?;
        buf.push_str("</body>");
        Ok(())
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MJBody {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'h> + 'r> {
        Box::new(MJBodyRender::<'e, 'h> {
            element: self,
            header,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::helper::test::cleanup;
    use crate::mjml::MJML;

    #[test]
    fn empty() {
        let template = include_str!("../../resources/compare/success/mj-body.mjml");
        let expected = include_str!("../../resources/compare/success/mj-body.html");
        let root = MJML::parse(template.to_string()).unwrap();
        let result = cleanup(root.render().unwrap().as_str());
        let expected = cleanup(expected);
        assert_eq!(result, expected);
    }
}
