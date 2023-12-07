use std::cell::{Ref, RefCell};
use std::rc::Rc;

use super::{MjAccordionText, NAME};
use crate::helper::tag::Tag;
use crate::prelude::hash::Map;
use crate::prelude::render::{Error, Header, Render, RenderOptions, Renderable};

struct MjAccordionTextRender<'e, 'h> {
    header: Rc<RefCell<Header<'h>>>,
    element: &'e MjAccordionText,
    extra: Map<String, String>,
}

impl<'e, 'h> MjAccordionTextRender<'e, 'h> {
    fn render_children(&self, opts: &RenderOptions) -> Result<String, Error> {
        let content = self
            .element
            .children
            .iter()
            .try_fold(String::default(), |res, child| {
                let renderer = child.renderer(Rc::clone(&self.header));
                Ok(res + &renderer.render(opts)?)
            })?;
        Ok(Tag::td()
            .maybe_add_class(self.attribute("css-class"))
            .maybe_add_style("background", self.attribute("background-color"))
            .maybe_add_style("font-size", self.attribute("font-size"))
            .maybe_add_style("font-family", self.attribute("font-family"))
            .maybe_add_style("line-height", self.attribute("line-height"))
            .maybe_add_style("color", self.attribute("color"))
            .maybe_add_style("padding-top", self.attribute("padding-top"))
            .maybe_add_style("padding-right", self.attribute("padding-right"))
            .maybe_add_style("padding-bottom", self.attribute("padding-bottom"))
            .maybe_add_style("padding-left", self.attribute("padding-left"))
            .maybe_add_style("padding", self.attribute("padding"))
            .render(content))
    }
}

impl<'e, 'h> Render<'h> for MjAccordionTextRender<'e, 'h> {
    fn default_attribute(&self, name: &str) -> Option<&str> {
        match name {
            "line-height" => Some("1"),
            "font-size" => Some("13px"),
            "padding" => Some("16px"),
            _ => None,
        }
    }

    fn add_extra_attribute(&mut self, key: &str, value: &str) {
        self.extra.insert(key.to_string(), value.to_string());
    }

    fn extra_attributes(&self) -> Option<&Map<String, String>> {
        Some(&self.extra)
    }

    fn attributes(&self) -> Option<&Map<String, String>> {
        Some(&self.element.attributes)
    }

    fn tag(&self) -> Option<&str> {
        Some(NAME)
    }

    fn header(&self) -> Ref<Header<'h>> {
        self.header.borrow()
    }

    fn render(&self, opts: &RenderOptions) -> Result<String, Error> {
        let font_families = self.attribute("font-family");
        self.header
            .borrow_mut()
            .maybe_add_font_families(font_families);
        let tr = Tag::tr().render(self.render_children(opts)?);
        let tbody = Tag::tbody().render(tr);
        let table = Tag::table()
            .add_attribute("cellspacing", "0")
            .add_attribute("cellpadding", "0")
            .add_style("width", "100%")
            .maybe_add_style("border-bottom", self.attribute("border"))
            .render(tbody);
        let div = Tag::div().add_class("mj-accordion-content").render(table);
        Ok(div)
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MjAccordionText {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'h> + 'r> {
        Box::new(MjAccordionTextRender::<'e, 'h> {
            element: self,
            header,
            extra: Map::default(),
        })
    }
}
