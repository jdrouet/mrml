use super::{MJButton, NAME};
use crate::helper::size::Pixel;
use crate::helper::tag::Tag;
use crate::prelude::render::{Error, Header, Render, Renderable};
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

struct MJButtonRender<'e, 'h> {
    header: Rc<RefCell<Header<'h>>>,
    element: &'e MJButton,
}

impl<'e, 'h> MJButtonRender<'e, 'h> {
    fn content_width(&self) -> Option<String> {
        if let Some(width) = self.attribute_as_pixel("width") {
            let pad_left = self
                .attribute_as_pixel("inner-padding-left")
                .map(|pad| pad.value())
                .or_else(|| {
                    self.attribute_as_spacing("inner-padding")
                        .map(|pad| pad.left().value())
                })
                .unwrap_or(0.0);
            let pad_right = self
                .attribute_as_pixel("inner-padding-right")
                .map(|pad| pad.value())
                .or_else(|| {
                    self.attribute_as_spacing("inner-padding")
                        .map(|pad| pad.right().value())
                })
                .unwrap_or(0.0);
            Some(Pixel::new(width.value() - pad_left - pad_right).to_string())
        } else {
            None
        }
    }

    fn render_children(&self) -> Result<String, Error> {
        self.element
            .children
            .iter()
            .try_fold(String::default(), |res, child| {
                let renderer = child.renderer(Rc::clone(&self.header));
                Ok(res + &renderer.render()?)
            })
    }

    fn set_style_table(&self, tag: Tag) -> Tag {
        tag.add_style("border-collapse", "separate")
            .maybe_add_style("width", self.attribute("width"))
            .add_style("line-height", "100%")
    }

    fn set_style_td(&self, tag: Tag) -> Tag {
        tag.maybe_add_style("background", self.attribute("background-color"))
            .maybe_add_style("border", self.attribute("border"))
            .maybe_add_style("border-top", self.attribute("border-top"))
            .maybe_add_style("border-right", self.attribute("border-right"))
            .maybe_add_style("border-bottom", self.attribute("border-bottom"))
            .maybe_add_style("border-left", self.attribute("border-left"))
            .maybe_add_style("border-radius", self.attribute("border-radius"))
            .add_style("cursor", "auto")
            .maybe_add_style("font-style", self.attribute("font-style"))
            .maybe_add_style("height", self.attribute("height"))
            .maybe_add_style("mso-padding-alt", self.attribute("inner-padding"))
            .maybe_add_style("text-align", self.attribute("text-align"))
    }

    fn set_style_content(&self, tag: Tag) -> Tag {
        tag.add_style("display", "inline-block")
            .maybe_add_style("width", self.content_width())
            .maybe_add_style("background", self.attribute("background-color"))
            .maybe_add_style("color", self.attribute("color"))
            .maybe_add_style("font-family", self.attribute("font-family"))
            .maybe_add_style("font-size", self.attribute("font-size"))
            .maybe_add_style("font-style", self.attribute("font-style"))
            .maybe_add_style("font-weight", self.attribute("font-weight"))
            .maybe_add_style("line-height", self.attribute("line-height"))
            .maybe_add_style("line-spacing", self.attribute("line-spacing"))
            .add_style("margin", "0")
            .maybe_add_style("text-decoration", self.attribute("text-decoration"))
            .maybe_add_style("text-transform", self.attribute("text-transform"))
            .maybe_add_style("padding", self.attribute("inner-padding"))
            .add_style("mso-padding-alt", "0px")
            .maybe_add_style("border-radius", self.attribute("border-radius"))
    }
}

impl<'e, 'h> Render<'h> for MJButtonRender<'e, 'h> {
    fn default_attribute(&self, key: &str) -> Option<&str> {
        match key {
            "align" => Some("center"),
            "background-color" => Some("#414141"),
            "border" => Some("none"),
            "border-radius" => Some("3px"),
            "color" => Some("#ffffff"),
            "font-family" => Some("Ubuntu, Helvetica, Arial, sans-serif"),
            "font-size" => Some("13px"),
            "font-weight" => Some("normal"),
            "inner-padding" => Some("10px 25px"),
            "line-height" => Some("120%"),
            "padding" => Some("10px 25px"),
            "target" => Some("_blank"),
            "text-decoration" => Some("none"),
            "text-transform" => Some("none"),
            "vertical-align" => Some("middle"),
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
        let table = self.set_style_table(Tag::table_presentation());
        let tr = Tag::tr();
        let td = self
            .set_style_td(Tag::td())
            .add_attribute("align", "center")
            .maybe_add_attribute("bgcolor", self.attribute("background-color"))
            .add_attribute("role", "presentation")
            .maybe_add_attribute("valign", self.attribute("vertical-align"));
        let link = Tag::new(self.attribute("href").map(|_| "a").unwrap_or("p"))
            .maybe_add_attribute("href", self.attribute("href"))
            .maybe_add_attribute("rel", self.attribute("rel"))
            .maybe_add_attribute("name", self.attribute("name"))
            .maybe_add_attribute(
                "target",
                self.attribute("href")
                    .and_then(|_v| self.attribute("target")),
            );
        let link = self.set_style_content(link);

        Ok(table.render(tr.render(td.render(link.render(self.render_children()?)))))
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MJButton {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'h> + 'r> {
        Box::new(MJButtonRender::<'e, 'h> {
            element: self,
            header,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::helper::test::compare;
    use crate::mjml::MJML;

    #[test]
    fn basic() {
        let template = include_str!("../../resources/compare/success/mj-button.mjml");
        let expected = include_str!("../../resources/compare/success/mj-button.html");
        let root = MJML::parse(template.to_string()).unwrap();
        let result = root.render().unwrap();
        compare(expected, result.as_str());
    }

    #[test]
    fn align() {
        let template = include_str!("../../resources/compare/success/mj-button-align.mjml");
        let expected = include_str!("../../resources/compare/success/mj-button-align.html");
        let root = MJML::parse(template.to_string()).unwrap();
        let result = root.render().unwrap();
        compare(expected, result.as_str());
    }

    #[test]
    fn background() {
        let template = include_str!("../../resources/compare/success/mj-button-background.mjml");
        let expected = include_str!("../../resources/compare/success/mj-button-background.html");
        let root = MJML::parse(template.to_string()).unwrap();
        let result = root.render().unwrap();
        compare(expected, result.as_str());
    }

    #[test]
    fn border_radius() {
        let template = include_str!("../../resources/compare/success/mj-button-border-radius.mjml");
        let expected = include_str!("../../resources/compare/success/mj-button-border-radius.html");
        let root = MJML::parse(template.to_string()).unwrap();
        let result = root.render().unwrap();
        compare(expected, result.as_str());
    }

    #[test]
    fn border() {
        let template = include_str!("../../resources/compare/success/mj-button-border.mjml");
        let expected = include_str!("../../resources/compare/success/mj-button-border.html");
        let root = MJML::parse(template.to_string()).unwrap();
        let result = root.render().unwrap();
        compare(expected, result.as_str());
    }

    #[test]
    fn class() {
        let template = include_str!("../../resources/compare/success/mj-button-class.mjml");
        let expected = include_str!("../../resources/compare/success/mj-button-class.html");
        let root = MJML::parse(template.to_string()).unwrap();
        let result = root.render().unwrap();
        compare(expected, result.as_str());
    }

    #[test]
    fn color() {
        let template = include_str!("../../resources/compare/success/mj-button-color.mjml");
        let expected = include_str!("../../resources/compare/success/mj-button-color.html");
        let root = MJML::parse(template.to_string()).unwrap();
        let result = root.render().unwrap();
        compare(expected, result.as_str());
    }

    #[test]
    fn container_background_color() {
        let template = include_str!(
            "../../resources/compare/success/mj-button-container-background-color.mjml"
        );
        let expected = include_str!(
            "../../resources/compare/success/mj-button-container-background-color.html"
        );
        let root = MJML::parse(template.to_string()).unwrap();
        let result = root.render().unwrap();
        compare(expected, result.as_str());
    }

    #[test]
    fn example() {
        let template = include_str!("../../resources/compare/success/mj-button-example.mjml");
        let expected = include_str!("../../resources/compare/success/mj-button-example.html");
        let root = MJML::parse(template.to_string()).unwrap();
        let result = root.render().unwrap();
        compare(expected, result.as_str());
    }

    #[test]
    fn font_family() {
        let template = include_str!("../../resources/compare/success/mj-button-font-family.mjml");
        let expected = include_str!("../../resources/compare/success/mj-button-font-family.html");
        let root = MJML::parse(template.to_string()).unwrap();
        let result = root.render().unwrap();
        compare(expected, result.as_str());
    }

    #[test]
    fn font_size() {
        let template = include_str!("../../resources/compare/success/mj-button-font-size.mjml");
        let expected = include_str!("../../resources/compare/success/mj-button-font-size.html");
        let root = MJML::parse(template.to_string()).unwrap();
        let result = root.render().unwrap();
        compare(expected, result.as_str());
    }

    #[test]
    fn font_style() {
        let template = include_str!("../../resources/compare/success/mj-button-font-style.mjml");
        let expected = include_str!("../../resources/compare/success/mj-button-font-style.html");
        let root = MJML::parse(template.to_string()).unwrap();
        let result = root.render().unwrap();
        compare(expected, result.as_str());
    }

    #[test]
    fn font_weight() {
        let template = include_str!("../../resources/compare/success/mj-button-font-weight.mjml");
        let expected = include_str!("../../resources/compare/success/mj-button-font-weight.html");
        let root = MJML::parse(template.to_string()).unwrap();
        let result = root.render().unwrap();
        compare(expected, result.as_str());
    }

    #[test]
    fn height() {
        let template = include_str!("../../resources/compare/success/mj-button-height.mjml");
        let expected = include_str!("../../resources/compare/success/mj-button-height.html");
        let root = MJML::parse(template.to_string()).unwrap();
        let result = root.render().unwrap();
        compare(expected, result.as_str());
    }

    #[test]
    fn href() {
        let template = include_str!("../../resources/compare/success/mj-button-href.mjml");
        let expected = include_str!("../../resources/compare/success/mj-button-href.html");
        let root = MJML::parse(template.to_string()).unwrap();
        let result = root.render().unwrap();
        compare(expected, result.as_str());
    }

    #[test]
    fn inner_padding() {
        let template = include_str!("../../resources/compare/success/mj-button-inner-padding.mjml");
        let expected = include_str!("../../resources/compare/success/mj-button-inner-padding.html");
        let root = MJML::parse(template.to_string()).unwrap();
        let result = root.render().unwrap();
        compare(expected, result.as_str());
    }

    #[test]
    fn line_height() {
        let template = include_str!("../../resources/compare/success/mj-button-line-height.mjml");
        let expected = include_str!("../../resources/compare/success/mj-button-line-height.html");
        let root = MJML::parse(template.to_string()).unwrap();
        let result = root.render().unwrap();
        compare(expected, result.as_str());
    }

    #[test]
    fn padding() {
        let template = include_str!("../../resources/compare/success/mj-button-padding.mjml");
        let expected = include_str!("../../resources/compare/success/mj-button-padding.html");
        let root = MJML::parse(template.to_string()).unwrap();
        let result = root.render().unwrap();
        compare(expected, result.as_str());
    }

    #[test]
    fn text_decoration() {
        let template =
            include_str!("../../resources/compare/success/mj-button-text-decoration.mjml");
        let expected =
            include_str!("../../resources/compare/success/mj-button-text-decoration.html");
        let root = MJML::parse(template.to_string()).unwrap();
        let result = root.render().unwrap();
        compare(expected, result.as_str());
    }

    #[test]
    fn text_transform() {
        let template =
            include_str!("../../resources/compare/success/mj-button-text-transform.mjml");
        let expected =
            include_str!("../../resources/compare/success/mj-button-text-transform.html");
        let root = MJML::parse(template.to_string()).unwrap();
        let result = root.render().unwrap();
        compare(expected, result.as_str());
    }

    #[test]
    fn vertical_align() {
        let template =
            include_str!("../../resources/compare/success/mj-button-vertical-align.mjml");
        let expected =
            include_str!("../../resources/compare/success/mj-button-vertical-align.html");
        let root = MJML::parse(template.to_string()).unwrap();
        let result = root.render().unwrap();
        compare(expected, result.as_str());
    }

    #[test]
    fn width() {
        let template = include_str!("../../resources/compare/success/mj-button-width.mjml");
        let expected = include_str!("../../resources/compare/success/mj-button-width.html");
        let root = MJML::parse(template.to_string()).unwrap();
        let result = root.render().unwrap();
        compare(expected, result.as_str());
    }
}
