use super::{MJAccordionText, NAME};
use crate::helper::tag::Tag;
use crate::prelude::render::{Error, Header, Render, Renderable};
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

struct MJAccordionTextRender<'e, 'h> {
    header: Rc<RefCell<Header<'h>>>,
    element: &'e MJAccordionText,
    extra: HashMap<String, String>,
}

impl<'e, 'h> MJAccordionTextRender<'e, 'h> {
    fn render_children(&self) -> Result<String, Error> {
        let content = self
            .element
            .children
            .iter()
            .try_fold(String::default(), |res, child| {
                let renderer = child.renderer(Rc::clone(&self.header));
                Ok(res + &renderer.render()?)
            })?;
        Ok(Tag::td()
            .maybe_add_class(self.attribute("css-class"))
            .maybe_add_style("background", self.attribute("background-color"))
            .maybe_add_style("color", self.attribute("color"))
            .maybe_add_style("font-family", self.attribute("font-family"))
            .maybe_add_style("font-size", self.attribute("font-size"))
            .maybe_add_style("line-height", self.attribute("line-height"))
            .maybe_add_style("padding", self.attribute("padding"))
            .maybe_add_style("padding-top", self.attribute("padding-top"))
            .maybe_add_style("padding-right", self.attribute("padding-right"))
            .maybe_add_style("padding-bottom", self.attribute("padding-bottom"))
            .maybe_add_style("padding-left", self.attribute("padding-left"))
            .render(content))
    }
}

impl<'e, 'h> Render<'h> for MJAccordionTextRender<'e, 'h> {
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

    fn extra_attributes(&self) -> Option<&HashMap<String, String>> {
        Some(&self.extra)
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

    fn render(&self) -> Result<String, Error> {
        let font_families = self.attribute("font-family");
        self.header
            .borrow_mut()
            .maybe_add_font_families(font_families);
        let tr = Tag::tr().render(self.render_children()?);
        let tbody = Tag::tbody().render(tr);
        let table = Tag::table()
            .add_attribute("cell-spacing", 0)
            .add_attribute("cell-padding", 0)
            .add_style("width", "100%")
            .maybe_add_style("border-bottom", self.attribute("border"))
            .render(tbody);
        let div = Tag::div().add_class("mj-accordion-content").render(table);
        Ok(div)
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MJAccordionText {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'h> + 'r> {
        Box::new(MJAccordionTextRender::<'e, 'h> {
            element: self,
            header,
            extra: HashMap::default(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::helper::test::compare;
    use crate::mjml::MJML;

    #[test]
    fn basic() {
        let template = include_str!("../../resources/compare/success/mj-carousel.mjml");
        let expected = include_str!("../../resources/compare/success/mj-carousel.html");
        let root = MJML::parse(template.to_string()).unwrap();
        let result = root.render().unwrap();
        compare(expected, result.as_str());
    }

    #[test]
    fn align_border_radius_class() {
        let template = include_str!(
            "../../resources/compare/success/mj-carousel-align-border-radius-class.mjml"
        );
        let expected = include_str!(
            "../../resources/compare/success/mj-carousel-align-border-radius-class.html"
        );
        let root = MJML::parse(template.to_string()).unwrap();
        let result = root.render().unwrap();
        compare(expected, result.as_str());
    }

    #[test]
    fn icon() {
        let template = include_str!("../../resources/compare/success/mj-carousel-icon.mjml");
        let expected = include_str!("../../resources/compare/success/mj-carousel-icon.html");
        let root = MJML::parse(template.to_string()).unwrap();
        let result = root.render().unwrap();
        compare(expected, result.as_str());
    }

    #[test]
    fn tb() {
        let template = include_str!("../../resources/compare/success/mj-carousel-tb.mjml");
        let expected = include_str!("../../resources/compare/success/mj-carousel-tb.html");
        let root = MJML::parse(template.to_string()).unwrap();
        let result = root.render().unwrap();
        compare(expected, result.as_str());
    }

    #[test]
    fn thumbnails() {
        let template = include_str!("../../resources/compare/success/mj-carousel-thumbnails.mjml");
        let expected = include_str!("../../resources/compare/success/mj-carousel-thumbnails.html");
        let root = MJML::parse(template.to_string()).unwrap();
        let result = root.render().unwrap();
        compare(expected, result.as_str());
    }
}
