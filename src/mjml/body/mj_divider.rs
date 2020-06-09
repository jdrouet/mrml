use crate::mjml::body::prelude::*;
use crate::mjml::error::Error;
use crate::mjml::prelude::*;
use crate::util::condition::conditional_tag;
use crate::util::{Context, Header, Size, Tag};
use crate::Options;
use log::debug;
use roxmltree::Node;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct MJDivider {
    options: Options,
    attributes: HashMap<String, String>,
    context: Option<Context>,
}

impl MJDivider {
    pub fn parse<'a, 'b>(node: &Node<'a, 'b>, opts: &Options) -> Result<MJDivider, Error> {
        Ok(MJDivider {
            options: opts.clone(),
            attributes: get_node_attributes(&node),
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
    fn default_attribute(&self, key: &str) -> Option<String> {
        debug!("default_attribute {}", key);
        match key {
            "border-color" => Some("#000000".into()),
            "border-style" => Some("solid".into()),
            "border-width" => Some("4px".into()),
            "padding" => Some("10px 25px".into()),
            "width" => Some("100%".into()),
            _ => None,
        }
    }

    fn source_attributes(&self) -> Option<&HashMap<String, String>> {
        Some(&self.attributes)
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
