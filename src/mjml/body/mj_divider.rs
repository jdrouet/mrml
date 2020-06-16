use crate::mjml::body::prelude::*;
use crate::mjml::error::Error;
use crate::mjml::prelude::*;
use crate::util::attributes::*;
use crate::util::condition::conditional_tag;
use crate::util::{Context, Header, Size, Tag};
use roxmltree::Node;
use std::collections::HashMap;

fn create_default_attributes() -> Attributes {
    Attributes::new()
        .add("border-color", "#000000")
        .add("border-style", "solid")
        .add("border-width", "4px")
        .add("padding", "10px 25px")
        .add("width", "100%")
}

#[derive(Clone, Debug)]
pub struct MJDivider {
    attributes: Attributes,
    context: Option<Context>,
}

impl MJDivider {
    fn default_attributes(header: &Header) -> Attributes {
        header
            .default_attributes()
            .set_element_attributes("mj-divider", create_default_attributes())
    }

    pub fn parse<'a, 'b>(node: &Node<'a, 'b>, header: &Header) -> Result<MJDivider, Error> {
        Ok(MJDivider {
            attributes: Self::default_attributes(header).concat(node),
            context: None,
        })
    }

    fn set_style_p(&self, tag: Tag) -> Tag {
        tag.set_style(
            "border-top",
            format!(
                "{} {} {}",
                self.get_attribute("border-style").unwrap(),
                self.get_attribute("border-width").unwrap(),
                self.get_attribute("border-color").unwrap()
            ),
        )
        .set_style("font-size", "1")
        .set_style("margin", "0px auto")
        .maybe_set_style("width", self.get_attribute("width"))
    }

    fn set_style_outlook(&self, tag: Tag) -> Tag {
        self.set_style_p(tag)
            .set_style("width", self.get_outlook_width())
    }

    fn get_outlook_width(&self) -> Size {
        let container_width = match self.get_container_width() {
            Some(value) => value,
            None => Size::Percent(100.0),
        };
        let padding_horizontal = self.get_padding_horizontal_width();
        let width = match self.get_size_attribute("width") {
            Some(value) => value,
            None => Size::Percent(100.0),
        };
        match width {
            Size::Percent(value) => {
                Size::Pixel((container_width.value() * value) / 100.0 - padding_horizontal.value())
            }
            Size::Pixel(value) => Size::Pixel(value),
            Size::Raw(_) => Size::Pixel(container_width.value() - padding_horizontal.value()),
        }
    }

    fn render_after(&self) -> String {
        let table = self
            .set_style_outlook(Tag::table_presentation())
            .set_attribute("align", "center")
            .set_attribute("width", self.get_outlook_width());
        let tr = Tag::new("tr");
        let td = Tag::new("td")
            .set_style("height", 0)
            .set_style("line-height", 0);
        conditional_tag(table.render(tr.render(td.render("&nbsp;"))))
    }
}

impl Component for MJDivider {
    fn context(&self) -> Option<&Context> {
        self.context.as_ref()
    }

    fn set_context(&mut self, ctx: Context) {
        self.context = Some(ctx.clone());
    }

    fn render(&self, _header: &Header) -> Result<String, Error> {
        let mut res = vec![];
        res.push(self.set_style_p(Tag::new("p")).render(""));
        res.push(self.render_after());
        Ok(res.join(""))
    }
}

impl ComponentWithAttributes for MJDivider {
    fn attributes(&self) -> Option<&HashMap<String, String>> {
        Some(self.attributes.inner())
    }
}

impl BodyComponent for MJDivider {
    fn set_style(&self, name: &str, tag: Tag) -> Tag {
        match name {
            "p" => self.set_style_p(tag),
            "outlook" => self.set_style_outlook(tag),
            _ => tag,
        }
    }
}

impl BodyContainedComponent for MJDivider {}
impl ComponentWithSizeAttribute for MJDivider {}
impl BodyComponentWithBorder for MJDivider {}
impl BodyComponentWithPadding for MJDivider {}
impl BodyComponentWithBoxWidths for MJDivider {}

#[cfg(test)]
pub mod tests {
    use crate::tests::compare_render;

    #[test]
    fn base() {
        compare_render(
            include_str!("../../../test/mj-divider.mjml"),
            include_str!("../../../test/mj-divider.html"),
        );
    }

    #[test]
    fn with_border() {
        compare_render(
            include_str!("../../../test/mj-divider-border.mjml"),
            include_str!("../../../test/mj-divider-border.html"),
        );
    }

    #[test]
    fn with_container_background_color() {
        compare_render(
            include_str!("../../../test/mj-divider-container-background-color.mjml"),
            include_str!("../../../test/mj-divider-container-background-color.html"),
        );
    }

    #[test]
    fn with_css_class() {
        compare_render(
            include_str!("../../../test/mj-divider-class.mjml"),
            include_str!("../../../test/mj-divider-class.html"),
        );
    }

    #[test]
    fn with_padding() {
        compare_render(
            include_str!("../../../test/mj-divider-padding.mjml"),
            include_str!("../../../test/mj-divider-padding.html"),
        );
    }

    #[test]
    fn with_width() {
        compare_render(
            include_str!("../../../test/mj-divider-width.mjml"),
            include_str!("../../../test/mj-divider-width.html"),
        );
    }
}
