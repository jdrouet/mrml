use std::cell::{Ref, RefCell};
use std::rc::Rc;

use super::{MjAccordionTitle, NAME};
use crate::helper::condition::negation_conditional_tag;
use crate::helper::tag::Tag;
use crate::prelude::hash::Map;
use crate::prelude::render::{Error, Header, Render, RenderOptions, Renderable};

struct MjAccordionTitleRender<'e, 'h> {
    header: Rc<RefCell<Header<'h>>>,
    element: &'e MjAccordionTitle,
    extra: Map<String, String>,
}

impl<'e, 'h> MjAccordionTitleRender<'e, 'h> {
    fn set_style_img(&self, tag: Tag) -> Tag {
        tag.add_style("display", "none")
            .maybe_add_style("width", self.attribute("icon-width"))
            .maybe_add_style("height", self.attribute("icon-height"))
    }

    fn render_title(&self, opts: &RenderOptions) -> Result<String, Error> {
        let content = self
            .element
            .children
            .iter()
            .try_fold(String::default(), |res, child| {
                let renderer = child.renderer(Rc::clone(&self.header));
                Ok(res + &renderer.render(opts)?)
            })?;
        Ok(Tag::td()
            .add_style("width", "100%")
            .maybe_add_style("background-color", self.attribute("background-color"))
            .maybe_add_style("color", self.attribute("color"))
            .maybe_add_style("font-size", self.attribute("font-size"))
            .maybe_add_style("font-family", self.attribute("font-family"))
            .maybe_add_style("padding-top", self.attribute("padding-top"))
            .maybe_add_style("padding-right", self.attribute("padding-right"))
            .maybe_add_style("padding-bottom", self.attribute("padding-bottom"))
            .maybe_add_style("padding-left", self.attribute("padding-left"))
            .maybe_add_style("padding", self.attribute("padding"))
            .maybe_add_class(self.attribute("css-class"))
            .render(content))
    }

    fn render_icons(&self) -> String {
        let img_more = self
            .set_style_img(Tag::new("img"))
            .maybe_add_attribute("src", self.attribute("icon-wrapped-url"))
            .maybe_add_attribute("alt", self.attribute("icon-wrapped-alt"))
            .add_class("mj-accordion-more")
            .closed();
        let img_less = self
            .set_style_img(Tag::new("img"))
            .maybe_add_attribute("src", self.attribute("icon-unwrapped-url"))
            .maybe_add_attribute("alt", self.attribute("icon-unwrapped-alt"))
            .add_class("mj-accordion-less")
            .closed();
        let td = Tag::td()
            .add_style("padding", "16px")
            .maybe_add_style("background", self.attribute("background-color"))
            .maybe_add_style("vertical-align", self.attribute("icon-align"))
            .add_class("mj-accordion-ico")
            .render(img_more + &img_less);
        negation_conditional_tag(td)
    }
}

impl<'e, 'h> Render<'h> for MjAccordionTitleRender<'e, 'h> {
    fn add_extra_attribute(&mut self, key: &str, value: &str) {
        self.extra.insert(key.to_string(), value.to_string());
    }

    fn extra_attributes(&self) -> Option<&Map<String, String>> {
        Some(&self.extra)
    }

    fn default_attribute(&self, name: &str) -> Option<&str> {
        match name {
            "font-size" => Some("13px"),
            "padding" => Some("16px"),
            _ => None,
        }
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
        let mut content = [self.render_title(opts)?, self.render_icons()];
        if !self.attribute_equals("icon-position", "right") {
            content.reverse();
        }
        let content = content.join("");
        let tr = Tag::tr().render(content);
        let tbody = Tag::tbody().render(tr);
        let table = Tag::table()
            .add_attribute("cellspacing", "0")
            .add_attribute("cellpadding", "0")
            .add_style("width", "100%")
            .maybe_add_style("border-bottom", self.attribute("border"))
            .render(tbody);
        Ok(Tag::div().add_class("mj-accordion-title").render(table))
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MjAccordionTitle {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'h> + 'r> {
        Box::new(MjAccordionTitleRender::<'e, 'h> {
            element: self,
            header,
            extra: Map::new(),
        })
    }
}
