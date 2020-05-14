use super::error::Error;
use super::prelude::{Component, ContainedComponent};
use super::Element;
use crate::util::prelude::PropertyMap;
use crate::util::{Attributes, Context, Header, Size, Style};
use crate::{close_tag, open_tag, to_attributes};
use log::debug;
use roxmltree::Node;

const ALLOWED_ATTRIBUTES: [&'static str; 16] = [
    "background-color",
    "border",
    "border-bottom",
    "border-left",
    "border-radius",
    "border-right",
    "border-top",
    "css-class",
    "direction",
    "padding",
    "padding-top",
    "padding-bottom",
    "padding-left",
    "padding-right",
    "vertical-align",
    "width",
];

#[derive(Clone, Debug)]
pub struct MJColumn<'a, 'b> {
    context: Option<Context>,
    node: Node<'a, 'b>,
    children: Vec<Element<'a, 'b>>,
}

impl MJColumn<'_, '_> {
    pub fn parse<'a, 'b>(node: Node<'a, 'b>) -> Result<MJColumn<'a, 'b>, Error> {
        let mut children = vec![];
        for child in node.children() {
            children.push(Element::parse(child)?);
        }
        Ok(MJColumn {
            context: None,
            node,
            children,
        })
    }

    fn get_column_class(&self) -> Option<(String, Size)> {
        let parsed_width = self.get_parsed_width();
        let classname = match parsed_width {
            Size::Percent(value) => format!("mj-column-per-{}", value),
            _ => format!("mj-column-px-{}", parsed_width.value()),
        };
        Some((classname.replace(".", "-"), parsed_width))
    }

    fn get_mobile_width(&self) -> Option<Size> {
        if self.get_attribute("mobile-width") != Some("mobile_width".into()) {
            return Some(Size::Percent(100.0));
        }
        let width = self
            .get_attribute("width")
            .and_then(|width| width.parse::<Size>().ok());
        if width.is_none() {
            return self
                .context()
                .and_then(|ctx| Some(ctx.non_raw_siblings()))
                .and_then(|count| Some(Size::Percent(100.0 / (count as f32))));
        }
        return width.and_then(|width| match width {
            Size::Percent(_) => Some(width),
            _ => match self.get_container_width() {
                Some(container) => Some(Size::Percent(width.value() / container.value())),
                None => None,
            },
        });
    }

    fn get_parsed_width(&self) -> Size {
        let non_raw_siblings = self
            .context()
            .and_then(|ctx| Some(ctx.non_raw_siblings()))
            .or(Some(1))
            .unwrap();
        match self
            .get_attribute("width")
            .and_then(|width| width.parse::<Size>().ok())
        {
            Some(size) => size,
            None => Size::Percent(100.0 / (non_raw_siblings as f32)),
        }
    }

    fn get_width_as_pixel(&self) -> String {
        let container_width = self.get_container_width();
        if container_width.is_none() {
            return "100%".into();
        }
        let container_width = container_width.unwrap();
        let parsed_width = self.get_parsed_width();
        let result = match parsed_width {
            Size::Percent(value) => Size::Pixel(container_width.value() * value / 100.0),
            _ => parsed_width,
        };
        result.to_string()
    }

    fn get_table_style(&self) -> Style {
        let mut res = Style::new();
        res.maybe_set("background-color", self.get_attribute("background-color"));
        res.maybe_set("border", self.get_attribute("border"));
        res.maybe_set("border-bottom", self.get_attribute("border-bottom"));
        res.maybe_set("border-left", self.get_attribute("border-left"));
        res.maybe_set("border-radius", self.get_attribute("border-radius"));
        res.maybe_set("border-right", self.get_attribute("border-right"));
        res.maybe_set("border-top", self.get_attribute("border-top"));
        res.maybe_set("vertical-align", self.get_attribute("vertical-align"));
        res
    }

    fn has_gutter(&self) -> bool {
        self.get_attribute("padding").is_some()
            || self.get_attribute("padding-bottom").is_some()
            || self.get_attribute("padding-left").is_some()
            || self.get_attribute("padding-right").is_some()
            || self.get_attribute("padding-top").is_some()
    }

    fn render_gutter(&self) -> Result<String, Error> {
        let mut res = vec![];
        res.push(open_tag!(
            "table",
            to_attributes!(
                ("border", "0"),
                ("cellpadding", "0"),
                ("cellspacing", "0"),
                ("role", "presentation"),
                ("width", "100%")
            )
        ));
        res.push(open_tag!("tbody"));
        res.push(open_tag!("tr"));
        res.push(open_tag!(
            "td",
            to_attributes!(("style", self.get_style("gutter").to_string()))
        ));
        res.push(self.render_column()?);
        res.push(close_tag!("td"));
        res.push(close_tag!("tr"));
        res.push(close_tag!("tbody"));
        res.push(close_tag!("table"));
        Ok(res.join(""))
    }

    fn render_column(&self) -> Result<String, Error> {
        let mut res = vec![];
        res.push(open_tag!(
            "table",
            to_attributes!(
                ("border", "0"),
                ("cellpadding", "0"),
                ("cellspacing", "0"),
                ("role", "presentation"),
                ("style", self.get_style("table").to_string()),
                ("width", "100%")
            )
        ));
        for child in self.children.iter() {
            match child {
                Element::Raw(_) => res.push(child.render()?),
                _ => (),
            };
        }
        res.push(close_tag!("table"));
        Ok(res.join(""))
    }
}

impl Component for MJColumn<'_, '_> {
    fn allowed_attributes() -> Option<Vec<&'static str>> {
        Some(ALLOWED_ATTRIBUTES.to_vec())
    }

    fn default_attribute(key: &str) -> Option<String> {
        debug!("default_attribute {}", key);
        match key {
            "direction" => Some("ltr".into()),
            "vertical-align" => Some("top".into()),
            _ => None,
        }
    }

    fn context(&self) -> Option<&Context> {
        self.context.as_ref()
    }

    fn to_header(&self) -> Header {
        let mut header = Header::new();
        if let Some((classname, size)) = self.get_column_class() {
            header.add_media_query(classname, size);
        }
        header
    }

    fn get_style(&self, key: &str) -> Style {
        let mut res = Style::new();
        match key {
            "div" => {
                res.set("font-size", "0px");
                res.set("text-align", "left");
                res.maybe_set("direction", self.get_attribute("direction"));
                res.set("display", "inline-block");
                res.maybe_set("vertical-align", self.get_attribute("vertical-align"));
                res.maybe_set("width", self.get_mobile_width());
            }
            "table" => {
                if self.has_gutter() {
                    res.maybe_set("background-color", self.get_attribute("background-color"));
                    res.maybe_set("border", self.get_attribute("border"));
                    res.maybe_set("border-bottom", self.get_attribute("border-bottom"));
                    res.maybe_set("border-left", self.get_attribute("border-left"));
                    res.maybe_set("border-radius", self.get_attribute("border-radius"));
                    res.maybe_set("border-right", self.get_attribute("border-right"));
                    res.maybe_set("border-top", self.get_attribute("border-top"));
                } else {
                    res.merge(&self.get_table_style());
                }
            }
            "td-outlook" => {
                res.maybe_set("vertical-align", self.get_attribute("vertical-align"));
                res.set("width", self.get_width_as_pixel());
            }
            "gutter" => {
                res.merge(&self.get_table_style());
                res.maybe_set("padding", self.get_attribute("padding"));
                res.maybe_set("padding-top", self.get_attribute("padding-top"));
                res.maybe_set("padding-right", self.get_attribute("padding-right"));
                res.maybe_set("padding-bottom", self.get_attribute("padding-bottom"));
                res.maybe_set("padding-left", self.get_attribute("padding-left"));
            }
            _ => (),
        };
        res
    }

    fn node(&self) -> Option<Node> {
        Some(self.node)
    }

    fn set_context(&mut self, ctx: Context) {
        self.context = Some(ctx);
    }

    fn render(&self) -> Result<String, Error> {
        let mut classes = vec![];
        if let Some((classname, _size)) = self.get_column_class() {
            classes.push(classname);
        }
        classes.push("mj-outlook-group-fix".into());
        if let Some(class) = self.get_attribute("css-class") {
            classes.push(class.clone());
        }
        let mut attrs = Attributes::new();
        attrs.set("class", classes.join(" "));
        attrs.set("style", self.get_style("div").to_string());
        let mut res = vec![];
        res.push(open_tag!("div", attrs.to_string()));
        if self.has_gutter() {
            res.push(self.render_gutter()?);
        } else {
            res.push(self.render_column()?);
        }
        res.push(close_tag!("div"));
        Ok(res.join(""))
    }
}

impl ContainedComponent for MJColumn<'_, '_> {}

#[cfg(test)]
pub mod tests {
    use crate::tests::compare_render;

    #[test]
    fn base() {
        compare_render(
            include_str!("../../test/mj-column.mjml"),
            include_str!("../../test/mj-column.html"),
        );
    }

    #[test]
    fn with_background_color() {
        compare_render(
            include_str!("../../test/mj-column-background-color.mjml"),
            include_str!("../../test/mj-column-background-color.html"),
        );
    }

    #[test]
    fn with_border() {
        compare_render(
            include_str!("../../test/mj-column-border.mjml"),
            include_str!("../../test/mj-column-border.html"),
        );
    }

    #[test]
    fn with_border_radius() {
        compare_render(
            include_str!("../../test/mj-column-border-radius.mjml"),
            include_str!("../../test/mj-column-border-radius.html"),
        );
    }

    #[test]
    fn with_class() {
        compare_render(
            include_str!("../../test/mj-column-class.mjml"),
            include_str!("../../test/mj-column-class.html"),
        );
    }
}
