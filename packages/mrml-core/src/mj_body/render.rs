use std::cell::{Ref, RefCell};
use std::convert::TryFrom;
use std::rc::Rc;

use super::MjBody;
use crate::helper::size::Pixel;
use crate::helper::tag::Tag;
use crate::prelude::hash::Map;
use crate::prelude::render::{Error, Header, Render, RenderOptions, Renderable};

struct MjBodyRender<'e, 'h> {
    header: Rc<RefCell<Header<'h>>>,
    element: &'e MjBody,
}

impl<'e, 'h> MjBodyRender<'e, 'h> {
    fn get_width(&self) -> Option<Pixel> {
        self.attribute("width")
            .and_then(|value| Pixel::try_from(value.as_str()).ok())
    }

    fn get_body_tag(&self) -> Tag {
        self.set_body_style(Tag::new("body").add_style("word-spacing", "normal"))
    }

    fn get_content_div_tag(&self) -> Tag {
        self.set_body_style(Tag::new("div"))
            .maybe_add_attribute("class", self.attribute("css-class"))
            .maybe_add_attribute("lang", self.header().lang().map(ToString::to_string))
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

    fn render_content(&self, opts: &RenderOptions) -> Result<String, Error> {
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

impl<'e, 'h> Render<'h> for MjBodyRender<'e, 'h> {
    fn attributes(&self) -> Option<&Map<String, String>> {
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

    fn render(&self, opts: &RenderOptions) -> Result<String, Error> {
        let body = self.get_body_tag();
        let result = body.render(self.render_preview() + &self.render_content(opts)?);
        Ok(result)
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MjBody {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'h> + 'r> {
        Box::new(MjBodyRender::<'e, 'h> {
            element: self,
            header,
        })
    }
}

#[cfg(test)]
mod tests {
    crate::should_render!(empty, "mj-body");
}
