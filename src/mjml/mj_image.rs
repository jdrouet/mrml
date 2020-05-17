use super::error::Error;
use super::prelude::*;
// use super::Element;
// use crate::util::condition::*;
use crate::util::prelude::PropertyMap;
use crate::util::{Attributes, Context, Header, Size, Style};
use crate::{close_tag, closed_tag, open_tag, with_tag};
use log::debug;
use roxmltree::Node;

const ALLOWED_ATTRIBUTES: [&'static str; 29] = [
    "css-class",
    "align",
    "alt",
    "href",
    "name",
    "src",
    "srcset",
    "title",
    "rel",
    "enum",
    "border",
    "border-top",
    "border-right",
    "border-bottom",
    "border-left",
    "border-radius",
    "container-background-color",
    "fluid-on-mobile",
    "padding",
    "padding-top",
    "padding-right",
    "padding-bottom",
    "padding-left",
    "target",
    "width",
    "height",
    "max-height",
    "font-size",
    "usemap",
];

#[derive(Clone, Debug)]
pub struct MJImage<'a, 'b> {
    context: Option<Context>,
    node: Node<'a, 'b>,
}

impl MJImage<'_, '_> {
    pub fn parse<'a, 'b>(node: Node<'a, 'b>) -> Result<MJImage<'a, 'b>, Error> {
        Ok(MJImage {
            context: None,
            node,
        })
    }

    fn is_fluid_on_mobile(&self) -> bool {
        match self
            .get_attribute("fluid-on-mobile")
            .and_then(|value| value.parse::<bool>().ok())
        {
            Some(value) => value,
            None => false,
        }
    }

    fn is_full_width(&self) -> bool {
        self.get_attribute("full-width").is_some()
    }

    fn get_content_width(&self) -> Option<Size> {
        self.get_size_attribute("width")
            .and_then(|width| match self.get_box_widths() {
                Some(box_size) => {
                    if width.value() < box_size.value() {
                        Some(width)
                    } else {
                        Some(box_size)
                    }
                }
                None => Some(width),
            })
            // when no width given
            .or_else(|| self.get_box_widths())
    }

    fn render_image(&self) -> String {
        let mut attrs = Attributes::new();
        attrs.maybe_set("alt", self.get_attribute("alt"));
        match self.get_size_attribute("height") {
            Some(height) => {
                attrs.set("height", height.value());
            }
            None => {
                attrs.set("height", "auto");
            }
        };
        attrs.maybe_set("src", self.get_attribute("src"));
        attrs.maybe_set("srcset", self.get_attribute("srcset"));
        attrs.set("style", self.get_style("img").to_string());
        attrs.maybe_set("title", self.get_attribute("title"));
        attrs.maybe_set(
            "width",
            self.get_content_width()
                .and_then(|width| Some(width.value())),
        );
        attrs.maybe_set("usemap", self.get_attribute("usemap"));
        closed_tag!("img", attrs.to_string())
    }

    fn render_link(&self) -> String {
        let mut attrs = Attributes::new();
        attrs.maybe_set("href", self.get_attribute("href"));
        attrs.maybe_set("target", self.get_attribute("target"));
        attrs.maybe_set("rel", self.get_attribute("rel"));
        attrs.maybe_set("name", self.get_attribute("name"));
        with_tag!("a", attrs.to_string(), self.render_image())
    }
}

impl Component for MJImage<'_, '_> {
    fn allowed_attributes(&self) -> Option<Vec<&'static str>> {
        Some(ALLOWED_ATTRIBUTES.to_vec())
    }

    fn default_attribute(&self, key: &str) -> Option<String> {
        debug!("default_attribute {}", key);
        match key {
            "align" => Some("center".into()),
            "border" => Some("0".into()),
            "height" => Some("auto".into()),
            "padding" => Some("10px 25px".into()),
            "target" => Some("_blank".into()),
            "font-size" => Some("13px".into()),
            _ => None,
        }
    }

    fn to_header(&self) -> Header {
        let mut header = Header::new();
        if let Some(ctx) = self.context() {
            let mut style = format!(
                "@media only screen and (max-width:{}) {{\n",
                ctx.options().breakpoint.to_string(),
            );
            style.push_str("table.mj-full-width-mobile { width: 100% !important; }\n");
            style.push_str("td.mj-full-width-mobile { width: auto !important; }\n");
            style.push_str("}\n");
            header.add_style(style);
        }
        header
    }

    fn get_style(&self, name: &str) -> Style {
        let mut res = Style::new();
        match name {
            "img" => {
                res.maybe_set("border", self.get_attribute("border"));
                res.maybe_set("border-left", self.get_attribute("left"));
                res.maybe_set("border-right", self.get_attribute("right"));
                res.maybe_set("border-top", self.get_attribute("top"));
                res.maybe_set("border-bottom", self.get_attribute("bottom"));
                res.maybe_set("border-radius", self.get_attribute("border-radius"));
                res.set("display", "block");
                res.set("outline", "none");
                res.set("text-decoration", "none");
                res.maybe_set("height", self.get_attribute("height"));
                res.maybe_set("max-height", self.get_attribute("max-height"));
                res.set("width", "100%");
                if self.is_full_width() {
                    res.set("min-width", "100%");
                    res.set("max-width", "100%");
                }
                res.maybe_set("font-size", self.get_attribute("font-size"));
            }
            "td" => {
                if !self.is_full_width() {
                    res.maybe_set("width", self.get_content_width());
                }
            }
            "table" => {
                if self.is_full_width() {
                    res.set("min-width", "100%");
                    res.set("max-width", "100%");
                    res.maybe_set("width", self.get_content_width());
                }
                res.set("border-collapse", "collapse");
                res.set("border-spacing", "0px");
            }
            _ => (),
        };
        res
    }

    fn context(&self) -> Option<&Context> {
        self.context.as_ref()
    }

    fn node(&self) -> Option<Node> {
        Some(self.node)
    }

    fn set_context(&mut self, ctx: Context) {
        self.context = Some(ctx.clone());
    }

    fn render(&self) -> Result<String, Error> {
        let mut res = vec![];
        {
            let mut attrs = Attributes::new();
            attrs.set("border", "0");
            attrs.set("cellpadding", "0");
            attrs.set("cellspacing", "0");
            attrs.set("role", "presentation");
            attrs.set("style", self.get_style("table"));
            if self.is_fluid_on_mobile() {
                attrs.set("class", "mj-full-width-mobile");
            }
            res.push(open_tag!("table", attrs.to_string()));
        };
        res.push(open_tag!("tbody"));
        res.push(open_tag!("tr"));
        {
            let mut attrs = Attributes::new();
            attrs.set("style", self.get_style("td"));
            if self.is_fluid_on_mobile() {
                attrs.set("class", "mj-full-width-mobile");
            }
            res.push(open_tag!("td", attrs.to_string()));
        };
        if self.get_attribute("href").is_some() {
            res.push(self.render_link());
        } else {
            res.push(self.render_image());
        }
        res.push(close_tag!("td"));
        res.push(close_tag!("tr"));
        res.push(close_tag!("tbody"));
        res.push(close_tag!("table"));
        Ok(res.join(""))
    }
}

impl ContainedComponent for MJImage<'_, '_> {}
impl ComponentWithSizeAttribute for MJImage<'_, '_> {}
impl ComponentWithBorder for MJImage<'_, '_> {}
impl ComponentWithPadding for MJImage<'_, '_> {}
impl ComponentWithBoxWidths for MJImage<'_, '_> {}

#[cfg(test)]
pub mod tests {
    use crate::tests::compare_render;

    #[test]
    fn base() {
        compare_render(
            include_str!("../../test/mj-image.mjml"),
            include_str!("../../test/mj-image.html"),
        );
    }

    #[test]
    fn with_align() {
        compare_render(
            include_str!("../../test/mj-image-align.mjml"),
            include_str!("../../test/mj-image-align.html"),
        );
    }

    #[test]
    fn with_border() {
        compare_render(
            include_str!("../../test/mj-image-border.mjml"),
            include_str!("../../test/mj-image-border.html"),
        );
    }

    #[test]
    fn with_border_radius() {
        compare_render(
            include_str!("../../test/mj-image-border-radius.mjml"),
            include_str!("../../test/mj-image-border-radius.html"),
        );
    }

    #[test]
    fn with_container_background_color() {
        compare_render(
            include_str!("../../test/mj-image-container-background-color.mjml"),
            include_str!("../../test/mj-image-container-background-color.html"),
        );
    }

    #[test]
    fn with_css_class() {
        compare_render(
            include_str!("../../test/mj-image-class.mjml"),
            include_str!("../../test/mj-image-class.html"),
        );
    }

    #[test]
    fn with_fluid_on_mobile() {
        compare_render(
            include_str!("../../test/mj-image-fluid-on-mobile.mjml"),
            include_str!("../../test/mj-image-fluid-on-mobile.html"),
        );
    }

    #[test]
    fn with_height() {
        compare_render(
            include_str!("../../test/mj-image-height.mjml"),
            include_str!("../../test/mj-image-height.html"),
        );
    }

    #[test]
    fn with_href() {
        compare_render(
            include_str!("../../test/mj-image-href.mjml"),
            include_str!("../../test/mj-image-href.html"),
        );
    }

    #[test]
    fn with_padding() {
        compare_render(
            include_str!("../../test/mj-image-padding.mjml"),
            include_str!("../../test/mj-image-padding.html"),
        );
    }

    #[test]
    fn with_rel() {
        compare_render(
            include_str!("../../test/mj-image-rel.mjml"),
            include_str!("../../test/mj-image-rel.html"),
        );
    }

    #[test]
    fn with_title() {
        compare_render(
            include_str!("../../test/mj-image-title.mjml"),
            include_str!("../../test/mj-image-title.html"),
        );
    }
}
