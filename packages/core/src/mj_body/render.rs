use super::MJBody;
use crate::helper::size::Pixel;
use crate::helper::tag::Tag;
use crate::prelude::render::{Error, Header, Render, Renderable};
use std::cell::{Ref, RefCell};
use std::convert::TryFrom;
use std::rc::Rc;

struct MJBodyRender<'e, 'h> {
    header: Rc<RefCell<Header<'h>>>,
    element: &'e MJBody,
}

impl<'e, 'h> MJBodyRender<'e, 'h> {
    fn attribute(&self, name: &str) -> Option<&String> {
        self.element.attributes.get(name)
    }

    fn get_width(&self) -> Pixel {
        self.attribute("width")
            .and_then(|value| Pixel::try_from(value.as_str()).ok())
            .unwrap_or_else(|| self.header.borrow().breakpoint().clone())
    }

    fn get_body_tag(&self) -> Tag {
        self.set_body_style(Tag::new("body"))
    }

    fn get_content_div_tag(&self) -> Tag {
        self.set_body_style(Tag::new("div"))
            .maybe_add_attribute("class", self.attribute("css-class"))
    }

    fn set_body_style(&self, tag: Tag) -> Tag {
        tag.maybe_add_style("background-color", self.attribute("background-color"))
    }

    fn render_preview(&self) -> String {
        if let Some(value) = self
            .header
            .borrow()
            .head()
            .as_ref()
            .and_then(|h| h.preview())
            .map(|p| p.content())
        {
            String::from(
                r#"<div style="display:none;font-size:1px;color:#ffffff;line-height:1px;max-height:0px;max-width:0px;opacity:0;overflow:hidden;">"#,
            ) + value
                + "</div>"
        } else {
            String::default()
        }
    }

    fn render_content(&self) -> Result<String, Error> {
        let div = self.get_content_div_tag();
        let element_width = self.get_width();
        let mut children = String::default();
        for child in self.element.children.iter() {
            let mut renderer = child.renderer(Rc::clone(&self.header));
            renderer.set_container_width(Some(element_width.clone())); // TODO remove clone
            children.push_str(&renderer.render()?);
        }
        Ok(div.render(children))
    }
}

impl<'e, 'h> Render<'h> for MJBodyRender<'e, 'h> {
    fn header(&self) -> Ref<Header<'h>> {
        self.header.borrow()
    }

    fn render(&self) -> Result<String, Error> {
        let body = self.get_body_tag();
        Ok(body.render(self.render_preview() + &self.render_content()?))
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
    use crate::helper::test::compare;
    use crate::mjml::MJML;

    #[test]
    fn empty() {
        let template = include_str!("../../resources/compare/success/mj-body.mjml");
        let expected = include_str!("../../resources/compare/success/mj-body.html");
        let root = MJML::parse(template.to_string()).unwrap();
        compare(expected, root.render().unwrap().as_str());
    }
}
