use super::{MJText, NAME};
use crate::helper::condition::conditional_tag;
use crate::helper::tag::Tag;
use crate::prelude::render::{Error, Header, Options, Render, Renderable};
use std::cell::{Ref, RefCell};
use crate::prelude::hash::Map;
use std::rc::Rc;

struct MJTextRender<'e, 'h> {
    header: Rc<RefCell<Header<'h>>>,
    element: &'e MJText,
}

impl<'e, 'h> MJTextRender<'e, 'h> {
    fn set_style_text(&self, tag: Tag) -> Tag {
        tag.maybe_add_style("font-family", self.attribute("font-family"))
            .maybe_add_style("font-size", self.attribute("font-size"))
            .maybe_add_style("font-style", self.attribute("font-style"))
            .maybe_add_style("font-weight", self.attribute("font-weight"))
            .maybe_add_style("letter-spacing", self.attribute("letter-spacing"))
            .maybe_add_style("line-height", self.attribute("line-height"))
            .maybe_add_style("text-align", self.attribute("align"))
            .maybe_add_style("text-decoration", self.attribute("text-decoration"))
            .maybe_add_style("text-transform", self.attribute("text-transform"))
            .maybe_add_style("color", self.attribute("color"))
            .maybe_add_style("height", self.attribute("height"))
    }

    fn render_content(&self, opts: &Options) -> Result<String, Error> {
        let res = self
            .element
            .children
            .iter()
            .try_fold(String::default(), |res, child| {
                let renderer = child.renderer(Rc::clone(&self.header));
                Ok(res + &renderer.render(opts)?)
            })?;
        Ok(self.set_style_text(Tag::div()).render(res))
    }

    fn render_with_height(&self, height: &str, opts: &Options) -> Result<String, Error> {
        let table = Tag::table_presentation();
        let tr = Tag::tr();
        let td = Tag::td()
            .add_attribute("height", height)
            .add_style("vertical-align", "top")
            .add_style("height", height);
        Ok(conditional_tag(
            table.render(tr.render(td.render(self.render_content(opts)?))),
        ))
    }
}

impl<'e, 'h> Render<'h> for MJTextRender<'e, 'h> {
    fn default_attribute(&self, key: &str) -> Option<&str> {
        match key {
            "align" => Some("left"),
            "color" => Some("#000000"),
            "font-family" => Some("Ubuntu, Helvetica, Arial, sans-serif"),
            "font-size" => Some("13px"),
            "line-height" => Some("1"),
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

    fn render(&self, opts: &Options) -> Result<String, Error> {
        let font_family = self.attribute("font-family");
        self.header
            .borrow_mut()
            .maybe_add_font_families(font_family);
        if let Some(ref height) = self.attribute("height") {
            self.render_with_height(height, opts)
        } else {
            self.render_content(opts)
        }
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MJText {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'h> + 'r> {
        Box::new(MJTextRender::<'e, 'h> {
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
    fn basic() {
        let opts = Options::default();
        let template = include_str!("../../resources/compare/success/mj-text.mjml");
        let expected = include_str!("../../resources/compare/success/mj-text.html");
        let root = MJML::parse(template.to_string()).unwrap();
        let result = root.render(&opts).unwrap();
        compare(expected, result.as_str());
    }

    #[test]
    fn align() {
        let opts = Options::default();
        let template = include_str!("../../resources/compare/success/mj-text-align.mjml");
        let expected = include_str!("../../resources/compare/success/mj-text-align.html");
        let root = MJML::parse(template.to_string()).unwrap();
        let result = root.render(&opts).unwrap();
        compare(expected, result.as_str());
    }

    #[test]
    fn class() {
        let opts = Options::default();
        let template = include_str!("../../resources/compare/success/mj-text-class.mjml");
        let expected = include_str!("../../resources/compare/success/mj-text-class.html");
        let root = MJML::parse(template.to_string()).unwrap();
        let result = root.render(&opts).unwrap();
        compare(expected, result.as_str());
    }

    #[test]
    fn color() {
        let opts = Options::default();
        let template = include_str!("../../resources/compare/success/mj-text-color.mjml");
        let expected = include_str!("../../resources/compare/success/mj-text-color.html");
        let root = MJML::parse(template.to_string()).unwrap();
        let result = root.render(&opts).unwrap();
        compare(expected, result.as_str());
    }

    #[test]
    fn container_background_color() {
        let opts = Options::default();
        let template =
            include_str!("../../resources/compare/success/mj-text-container-background-color.mjml");
        let expected =
            include_str!("../../resources/compare/success/mj-text-container-background-color.html");
        let root = MJML::parse(template.to_string()).unwrap();
        let result = root.render(&opts).unwrap();
        compare(expected, result.as_str());
    }

    #[test]
    fn example() {
        let opts = Options::default();
        let template = include_str!("../../resources/compare/success/mj-text-example.mjml");
        let expected = include_str!("../../resources/compare/success/mj-text-example.html");
        let root = MJML::parse(template.to_string()).unwrap();
        let result = root.render(&opts).unwrap();
        compare(expected, result.as_str());
    }

    #[test]
    fn font_family() {
        let opts = Options::default();
        let template = include_str!("../../resources/compare/success/mj-text-font-family.mjml");
        let expected = include_str!("../../resources/compare/success/mj-text-font-family.html");
        let root = MJML::parse(template.to_string()).unwrap();
        let result = root.render(&opts).unwrap();
        compare(expected, result.as_str());
    }

    #[test]
    fn font_size() {
        let opts = Options::default();
        let template = include_str!("../../resources/compare/success/mj-text-font-size.mjml");
        let expected = include_str!("../../resources/compare/success/mj-text-font-size.html");
        let root = MJML::parse(template.to_string()).unwrap();
        let result = root.render(&opts).unwrap();
        compare(expected, result.as_str());
    }

    #[test]
    fn font_style() {
        let opts = Options::default();
        let template = include_str!("../../resources/compare/success/mj-text-font-style.mjml");
        let expected = include_str!("../../resources/compare/success/mj-text-font-style.html");
        let root = MJML::parse(template.to_string()).unwrap();
        let result = root.render(&opts).unwrap();
        compare(expected, result.as_str());
    }

    #[test]
    fn font_weight() {
        let opts = Options::default();
        let template = include_str!("../../resources/compare/success/mj-text-font-weight.mjml");
        let expected = include_str!("../../resources/compare/success/mj-text-font-weight.html");
        let root = MJML::parse(template.to_string()).unwrap();
        let result = root.render(&opts).unwrap();
        compare(expected, result.as_str());
    }

    #[test]
    fn height() {
        let opts = Options::default();
        let template = include_str!("../../resources/compare/success/mj-text-height.mjml");
        let expected = include_str!("../../resources/compare/success/mj-text-height.html");
        let root = MJML::parse(template.to_string()).unwrap();
        let result = root.render(&opts).unwrap();
        compare(expected, result.as_str());
    }

    #[test]
    fn line_height() {
        let opts = Options::default();
        let template = include_str!("../../resources/compare/success/mj-text-line-height.mjml");
        let expected = include_str!("../../resources/compare/success/mj-text-line-height.html");
        let root = MJML::parse(template.to_string()).unwrap();
        let result = root.render(&opts).unwrap();
        compare(expected, result.as_str());
    }

    #[test]
    fn padding() {
        let opts = Options::default();
        let template = include_str!("../../resources/compare/success/mj-text-padding.mjml");
        let expected = include_str!("../../resources/compare/success/mj-text-padding.html");
        let root = MJML::parse(template.to_string()).unwrap();
        let result = root.render(&opts).unwrap();
        compare(expected, result.as_str());
    }
}
