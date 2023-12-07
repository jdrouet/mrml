use std::cell::{Ref, RefCell};
use std::rc::Rc;

use super::{MjAccordion, MjAccordionChild, NAME};
use crate::helper::size::{Pixel, Size};
use crate::helper::tag::Tag;
use crate::prelude::hash::Map;
use crate::prelude::render::{Error, Header, Render, RenderOptions, Renderable};

const CHILDREN_ATTRIBUTES: [&str; 9] = [
    "border",
    "icon-align",
    "icon-width",
    "icon-height",
    "icon-position",
    "icon-wrapped-url",
    "icon-wrapped-alt",
    "icon-unwrapped-url",
    "icon-unwrapped-alt",
];

struct MjAccordionRender<'e, 'h> {
    header: Rc<RefCell<Header<'h>>>,
    element: &'e MjAccordion,
    container_width: Option<Pixel>,
    siblings: usize,
    raw_siblings: usize,
}

impl<'e, 'h> MjAccordionRender<'e, 'h> {
    fn render_style(&self) -> String {
        r#"
        noinput.mj-accordion-checkbox { display: block! important; }

        @media yahoo, only screen and (min-width:0) {
          .mj-accordion-element { display:block; }
          input.mj-accordion-checkbox, .mj-accordion-less { display: none !important; }
          input.mj-accordion-checkbox+* .mj-accordion-title { cursor: pointer; touch-action: manipulation; -webkit-user-select: none; -moz-user-select: none; user-select: none; }
          input.mj-accordion-checkbox+* .mj-accordion-content { overflow: hidden; display: none; }
          input.mj-accordion-checkbox+* .mj-accordion-more { display: block !important; }
          input.mj-accordion-checkbox:checked+* .mj-accordion-content { display: block; }
          input.mj-accordion-checkbox:checked+* .mj-accordion-more { display: none !important; }
          input.mj-accordion-checkbox:checked+* .mj-accordion-less { display: block !important; }
        }

        .moz-text-html input.mj-accordion-checkbox+* .mj-accordion-title { cursor: auto; touch-action: auto; -webkit-user-select: auto; -moz-user-select: auto; user-select: auto; }
        .moz-text-html input.mj-accordion-checkbox+* .mj-accordion-content { overflow: hidden; display: block; }
        .moz-text-html input.mj-accordion-checkbox+* .mj-accordion-ico { display: none; }

        @goodbye { @gmail }
        "#.to_string()
    }

    fn update_header(&self) {
        let style = self.render_style();
        let font_families = self.attribute("font-family");
        self.header
            .borrow_mut()
            .maybe_add_font_families(font_families);
        self.header.borrow_mut().add_style(style);
    }
}

impl<'e, 'h> Render<'h> for MjAccordionRender<'e, 'h> {
    fn default_attribute(&self, name: &str) -> Option<&str> {
        match name {
            "border" => Some("2px solid black"),
            "font-family" => Some("Ubuntu, Helvetica, Arial, sans-serif"),
            "icon-align" => Some("middle"),
            "icon-position" => Some("right"),
            "icon-height" => Some("32px"),
            "icon-width" => Some("32px"),
            "icon-wrapped-url" => Some("https://i.imgur.com/bIXv1bk.png"),
            "icon-wrapped-alt" => Some("+"),
            "icon-unwrapped-url" => Some("https://i.imgur.com/w4uTygT.png"),
            "icon-unwrapped-alt" => Some("-"),
            "padding" => Some("10px 25px"),
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

    fn get_width(&self) -> Option<Size> {
        self.container_width
            .as_ref()
            .map(|w| Size::Pixel(w.clone()))
    }

    fn set_container_width(&mut self, width: Option<Pixel>) {
        self.container_width = width;
    }

    fn set_siblings(&mut self, value: usize) {
        self.siblings = value;
    }

    fn set_raw_siblings(&mut self, value: usize) {
        self.raw_siblings = value;
    }

    fn render(&self, opts: &RenderOptions) -> Result<String, Error> {
        self.update_header();
        let children = self
            .element
            .children
            .iter()
            .try_fold(String::default(), |res, child| {
                let mut renderer = child.renderer(Rc::clone(&self.header));
                CHILDREN_ATTRIBUTES.iter().for_each(|key| {
                    renderer.maybe_add_extra_attribute(key, self.attribute(key));
                });
                Ok(res + &renderer.render(opts)?)
            })?;
        let tbody = Tag::tbody().render(children);
        Ok(Tag::table()
            .add_style("width", "100%")
            .add_style("border-collapse", "collapse")
            .maybe_add_style("border", self.attribute("border"))
            .add_style("border-bottom", "none")
            .maybe_add_style("font-family", self.attribute("font-family"))
            .add_attribute("cellspacing", "0")
            .add_attribute("cellpadding", "0")
            .add_class("mj-accordion")
            .render(tbody))
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MjAccordion {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'h> + 'r> {
        Box::new(MjAccordionRender::<'e, 'h> {
            element: self,
            header,
            container_width: None,
            siblings: 1,
            raw_siblings: 0,
        })
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MjAccordionChild {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'h> + 'r> {
        match self {
            Self::MjAccordionElement(elt) => elt.renderer(header),
            Self::Comment(elt) => elt.renderer(header),
        }
    }
}

#[cfg(test)]
mod tests {
    crate::should_render!(basic, "mj-accordion");
    crate::should_render!(font_padding, "mj-accordion-font-padding");
    crate::should_render!(icon, "mj-accordion-icon");
    crate::should_render!(other, "mj-accordion-other");
}
