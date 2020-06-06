use crate::mjml::body::prelude::*;
use crate::mjml::error::Error;
use crate::mjml::prelude::*;
use crate::util::condition::conditional_tag;
use crate::util::prelude::*;
use crate::util::{Context, Header, Size, Style, Tag};
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
    pub fn parse<'a, 'b>(node: Node<'a, 'b>, opts: &Options) -> Result<MJDivider, Error> {
        Ok(MJDivider {
            options: opts.clone(),
            attributes: get_node_attributes(&node),
            context: None,
        })
    }

    fn get_style_p(&self) -> Style {
        let mut res = Style::new();
        res.set(
            "border-top",
            format!(
                "{} {} {}",
                self.get_attribute("border-style").unwrap(),
                self.get_attribute("border-width").unwrap(),
                self.get_attribute("border-color").unwrap()
            ),
        );
        res.set("font-size", "1");
        res.set("margin", "0px auto");
        res.maybe_set("width", self.get_attribute("width"));
        res
    }

    fn get_style_outlook(&self) -> Style {
        let mut res = self.get_style_p();
        res.set("width", self.get_outlook_width().to_string());
        res
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
        let table = Tag::new("table")
            .set_attribute("align", "center")
            .set_attribute("border", 0)
            .set_attribute("cellpadding", 0)
            .set_attribute("cellspacing", 0)
            .insert_style(self.get_style_outlook().inner())
            .set_attribute("role", "presentation")
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
        res.push(
            Tag::new("p")
                .insert_style(self.get_style_p().inner())
                .render(""),
        );
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
    fn get_style(&self, name: &str) -> Style {
        match name {
            "p" => self.get_style_p(),
            "outlook" => self.get_style_outlook(),
            _ => Style::new(),
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
