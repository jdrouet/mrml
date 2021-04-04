use super::{MJAccordion, NAME};
use crate::helper::size::{Pixel, Size};
use crate::helper::tag::Tag;
use crate::prelude::render::{Error, Header, Render, Renderable};
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

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

struct MJAccordionRender<'e, 'h> {
    header: Rc<RefCell<Header<'h>>>,
    element: &'e MJAccordion,
    container_width: Option<Pixel>,
    siblings: usize,
    raw_siblings: usize,
}

impl<'e, 'h> MJAccordionRender<'e, 'h> {
    fn render_style(&self) -> String {
        r#"
        noinput.mj-accordion-checkbox { display: block! important; }

        @media yahoo, only screen and (min-width:0) {
          .mj-accordion-element { display:block; }
          input.mj-accordion-checkbox, .mj-accordion-less { display: none !important; }
          input.mj-accordion-checkbox + * .mj-accordion-title { cursor: pointer; touch-action: manipulation; -webkit-user-select: none; -moz-user-select: none; user-select: none; }
          input.mj-accordion-checkbox + * .mj-accordion-content { overflow: hidden; display: none; }
          input.mj-accordion-checkbox + * .mj-accordion-more { display: block !important; }
          input.mj-accordion-checkbox:checked + * .mj-accordion-content { display: block; }
          input.mj-accordion-checkbox:checked + * .mj-accordion-more { display: none !important; }
          input.mj-accordion-checkbox:checked + * .mj-accordion-less { display: block !important; }
        }

        .moz-text-html input.mj-accordion-checkbox + * .mj-accordion-title { cursor: auto; touch-action: auto; -webkit-user-select: auto; -moz-user-select: auto; user-select: auto; }
        .moz-text-html input.mj-accordion-checkbox + * .mj-accordion-content { overflow: hidden; display: block; }
        .moz-text-html input.mj-accordion-checkbox + * .mj-accordion-ico { display: none; }

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

impl<'e, 'h> Render<'h> for MJAccordionRender<'e, 'h> {
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

    fn attributes(&self) -> Option<&HashMap<String, String>> {
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

    fn render(&self) -> Result<String, Error> {
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
                Ok(res + &renderer.render()?)
            })?;
        let tbody = Tag::tbody().render(children);
        Ok(Tag::table()
            .add_style("width", "100%")
            .add_style("border-collapse", "collapse")
            .maybe_add_style("border", self.attribute("border"))
            .add_style("border-bottom", "none")
            .maybe_add_style("font-family", self.attribute("font-family"))
            .add_attribute("cell-spacing", 0)
            .add_attribute("cell-padding", 0)
            .add_class("mj-accordion")
            .render(tbody))
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MJAccordion {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'h> + 'r> {
        Box::new(MJAccordionRender::<'e, 'h> {
            element: self,
            header,
            container_width: None,
            siblings: 1,
            raw_siblings: 0,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::helper::test::compare;
    use crate::mjml::MJML;

    #[test]
    fn basic() {
        let template = include_str!("../../resources/compare/success/mj-accordion.mjml");
        let expected = include_str!("../../resources/compare/success/mj-accordion.html");
        let root = MJML::parse(template.to_string()).unwrap();
        let result = root.render().unwrap();
        compare(expected, result.as_str());
    }

    #[test]
    fn font_padding() {
        let template =
            include_str!("../../resources/compare/success/mj-accordion-font-padding.mjml");
        let expected =
            include_str!("../../resources/compare/success/mj-accordion-font-padding.html");
        let root = MJML::parse(template.to_string()).unwrap();
        let result = root.render().unwrap();
        compare(expected, result.as_str());
    }

    #[test]
    fn icon() {
        let template = include_str!("../../resources/compare/success/mj-accordion-icon.mjml");
        let expected = include_str!("../../resources/compare/success/mj-accordion-icon.html");
        let root = MJML::parse(template.to_string()).unwrap();
        let result = root.render().unwrap();
        compare(expected, result.as_str());
    }

    #[test]
    fn other() {
        let template = include_str!("../../resources/compare/success/mj-accordion-other.mjml");
        let expected = include_str!("../../resources/compare/success/mj-accordion-other.html");
        let root = MJML::parse(template.to_string()).unwrap();
        let result = root.render().unwrap();
        compare(expected, result.as_str());
    }
}
