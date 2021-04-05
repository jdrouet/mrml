use super::MJBody;
use crate::helper::size::Pixel;
use crate::helper::tag::Tag;
use crate::prelude::render::{Error, Header, Options, Render, Renderable};
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::rc::Rc;

struct MJBodyRender<'e, 'h> {
    header: Rc<RefCell<Header<'h>>>,
    element: &'e MJBody,
}

impl<'e, 'h> MJBodyRender<'e, 'h> {
    fn get_width(&self) -> Option<Pixel> {
        self.attribute("width")
            .and_then(|value| Pixel::try_from(value.as_str()).ok())
    }

    fn get_body_tag(&self) -> Tag {
        self.set_body_style(Tag::new("body"))
            .add_style("word-spacing", "normal")
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

    fn render_content(&self, opts: &Options) -> Result<String, Error> {
        let div = self.get_content_div_tag();
        let element_width = self.get_width();
        let mut children = String::default();
        let raw_siblings = self
            .element
            .children
            .iter()
            .filter(|item| item.is_raw())
            .count();
        for (index, child) in self.element.children.iter().enumerate() {
            let mut renderer = child.renderer(Rc::clone(&self.header));
            renderer.set_container_width(element_width.clone());
            renderer.set_index(index);
            renderer.set_raw_siblings(raw_siblings);
            renderer.set_siblings(self.element.children.len());
            children.push_str(&renderer.render(opts)?);
        }
        Ok(div.render(children))
    }
}

impl<'e, 'h> Render<'h> for MJBodyRender<'e, 'h> {
    fn attributes(&self) -> Option<&HashMap<String, String>> {
        Some(&self.element.attributes)
    }

    fn default_attribute(&self, key: &str) -> Option<&str> {
        match key {
            "width" => Some("600px"),
            _ => None,
        }
    }

    fn header(&self) -> Ref<Header<'h>> {
        self.header.borrow()
    }

    fn render(&self, opts: &Options) -> Result<String, Error> {
        let body = self.get_body_tag();
        Ok(body.render(self.render_preview() + &self.render_content(opts)?))
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
    use crate::prelude::render::Options;

    #[test]
    fn empty() {
        let opts = Options::default();
        let template = include_str!("../../resources/compare/success/mj-body.mjml");
        let expected = include_str!("../../resources/compare/success/mj-body.html");
        let root = MJML::parse(template.to_string()).unwrap();
        compare(expected, root.render(&opts).unwrap().as_str());
    }
}
