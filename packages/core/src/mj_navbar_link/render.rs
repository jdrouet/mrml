use super::{MJNavbarLink, NAME};
use crate::helper::condition::conditional_tag;
use crate::helper::size::Pixel;
use crate::helper::tag::Tag;
use crate::prelude::render::{Error, Header, Render, Renderable};
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

struct MJNavbarLinkRender<'e, 'h> {
    header: Rc<RefCell<Header<'h>>>,
    element: &'e MJNavbarLink,
    extra: HashMap<String, String>,
    container_width: Option<Pixel>,
}

impl<'e, 'h> MJNavbarLinkRender<'e, 'h> {
    fn set_style_a(&self, tag: Tag) -> Tag {
        tag.add_style("display", "inline-block")
            .maybe_add_style("color", self.attribute("color"))
            .maybe_add_style("font-family", self.attribute("font-family"))
            .maybe_add_style("font-size", self.attribute("font-size"))
            .maybe_add_style("font-style", self.attribute("font-style"))
            .maybe_add_style("font-weight", self.attribute("font-weight"))
            .maybe_add_style("letter-spacing", self.attribute("letter-spacing"))
            .maybe_add_style("line-height", self.attribute("line-height"))
            .maybe_add_style("text-decoration", self.attribute("text-decoration"))
            .maybe_add_style("text-transform", self.attribute("text-transform"))
            .maybe_add_style("padding", self.attribute("padding"))
            .maybe_add_style("padding-top", self.attribute("padding-top"))
            .maybe_add_style("padding-right", self.attribute("padding-right"))
            .maybe_add_style("padding-bottom", self.attribute("padding-bottom"))
            .maybe_add_style("padding-left", self.attribute("padding-left"))
    }

    fn set_style_td(&self, tag: Tag) -> Tag {
        tag.maybe_add_style("padding", self.attribute("padding"))
            .maybe_add_style("padding-top", self.attribute("padding-top"))
            .maybe_add_style("padding-right", self.attribute("padding-right"))
            .maybe_add_style("padding-bottom", self.attribute("padding-bottom"))
            .maybe_add_style("padding-left", self.attribute("padding-left"))
    }

    fn get_link(&self) -> Option<String> {
        self.attribute("href").as_ref().and_then(|href| {
            self.attribute("navbar-base-url")
                .map(move |base| format!("{}{}", base, href))
                .or_else(|| Some(href.to_string()))
        })
    }

    fn render_content(&self) -> Result<String, Error> {
        let link = self
            .set_style_a(Tag::new("a"))
            .add_class("mj-link")
            .maybe_add_class(self.attribute("css-class"))
            .maybe_add_attribute("href", self.get_link())
            .maybe_add_attribute("rel", self.attribute("rel"))
            .maybe_add_attribute("target", self.attribute("target"))
            .maybe_add_attribute("name", self.attribute("name"));
        let content = self
            .element
            .children
            .iter()
            .try_fold(String::default(), |res, child| {
                let renderer = child.renderer(Rc::clone(&self.header));
                Ok(res + &renderer.render()?)
            })?;
        Ok(link.render(content))
    }
}

impl<'e, 'h> Render<'h> for MJNavbarLinkRender<'e, 'h> {
    fn default_attribute(&self, key: &str) -> Option<&str> {
        match key {
            "color" => Some("#000000"),
            "font-family" => Some("Ubuntu, Helvetica, Arial, sans-serif"),
            "font-size" => Some("13px"),
            "font-weight" => Some("normal"),
            "line-height" => Some("22px"),
            "padding" => Some("15px 10px"),
            "target" => Some("_blank"),
            "text-decoration" => Some("none"),
            "text-transform" => Some("uppercase"),
            _ => None,
        }
    }

    fn add_extra_attribute(&mut self, key: &str, value: &str) {
        self.extra.insert(key.to_string(), value.to_string());
    }

    fn extra_attributes(&self) -> Option<&HashMap<String, String>> {
        Some(&self.extra)
    }

    fn attributes(&self) -> Option<&HashMap<String, String>> {
        Some(&self.element.attributes)
    }

    fn tag(&self) -> Option<&str> {
        Some(NAME)
    }

    fn set_container_width(&mut self, width: Option<Pixel>) {
        self.container_width = width;
    }

    fn header(&self) -> Ref<Header<'h>> {
        self.header.borrow()
    }

    fn render(&self) -> Result<String, Error> {
        let td = self
            .set_style_td(Tag::td())
            .maybe_add_suffixed_class(self.attribute("css-class"), "outlook");
        Ok(conditional_tag(td.open()) + &self.render_content()? + &conditional_tag(td.close()))
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MJNavbarLink {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'h> + 'r> {
        Box::new(MJNavbarLinkRender::<'e, 'h> {
            element: self,
            header,
            extra: HashMap::new(),
            container_width: None,
        })
    }
}
