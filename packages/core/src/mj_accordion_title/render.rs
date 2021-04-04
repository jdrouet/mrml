use super::{MJAccordionTitle, NAME};
use crate::helper::condition::negation_conditional_tag;
use crate::helper::tag::Tag;
use crate::prelude::render::{Error, Header, Render, Renderable};
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

struct MJAccordionTitleRender<'e, 'h> {
    header: Rc<RefCell<Header<'h>>>,
    element: &'e MJAccordionTitle,
    extra: HashMap<String, String>,
}

impl<'e, 'h> MJAccordionTitleRender<'e, 'h> {
    fn set_style_img(&self, tag: Tag) -> Tag {
        tag.add_style("display", "none")
            .maybe_add_style("height", self.attribute("icon-height"))
            .maybe_add_style("width", self.attribute("icon-width"))
    }

    fn render_title(&self) -> Result<String, Error> {
        let content = self
            .element
            .children
            .iter()
            .try_fold(String::default(), |res, child| {
                let renderer = child.renderer(Rc::clone(&self.header));
                Ok(res + &renderer.render()?)
            })?;
        Ok(Tag::td()
            .maybe_add_style("background-color", self.attribute("background-color"))
            .maybe_add_style("color", self.attribute("color"))
            .maybe_add_style("font-size", self.attribute("font-size"))
            .maybe_add_style("font-family", self.attribute("font-family"))
            .maybe_add_style("padding", self.attribute("padding"))
            .maybe_add_style("padding-top", self.attribute("padding-top"))
            .maybe_add_style("padding-right", self.attribute("padding-right"))
            .maybe_add_style("padding-bottom", self.attribute("padding-bottom"))
            .maybe_add_style("padding-left", self.attribute("padding-left"))
            .add_style("width", "100%")
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

impl<'e, 'h> Render<'h> for MJAccordionTitleRender<'e, 'h> {
    fn add_extra_attribute(&mut self, key: &str, value: &str) {
        self.extra.insert(key.to_string(), value.to_string());
    }

    fn extra_attributes(&self) -> Option<&HashMap<String, String>> {
        Some(&self.extra)
    }

    fn default_attribute(&self, name: &str) -> Option<&str> {
        match name {
            "font-size" => Some("13px"),
            "padding" => Some("16px"),
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

    fn render(&self) -> Result<String, Error> {
        let font_families = self.attribute("font-family");
        self.header
            .borrow_mut()
            .maybe_add_font_families(font_families);
        let mut content = vec![self.render_title()?, self.render_icons()];
        if !self.attribute_equals("icon-position", "right") {
            content.reverse();
        }
        let content = content.join("");
        let tr = Tag::tr().render(content);
        let tbody = Tag::tbody().render(tr);
        let table = Tag::table()
            .add_attribute("cell-spacing", 0)
            .add_attribute("cell-padding", 0)
            .add_style("width", "100%")
            .maybe_add_style("border-bottom", self.attribute("border"))
            .render(tbody);
        Ok(Tag::div().add_class("mj-accordion-title").render(table))
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MJAccordionTitle {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'h> + 'r> {
        Box::new(MJAccordionTitleRender::<'e, 'h> {
            element: self,
            header,
            extra: HashMap::new(),
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
