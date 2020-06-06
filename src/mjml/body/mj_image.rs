use crate::mjml::body::prelude::*;
use crate::mjml::error::Error;
use crate::mjml::prelude::*;
use crate::util::prelude::*;
use crate::util::{Context, Header, Size, Style, Tag};
use crate::Options;
use log::debug;
use roxmltree::Node;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct MJImage {
    options: Options,
    attributes: HashMap<String, String>,
    context: Option<Context>,
}

impl MJImage {
    pub fn parse<'a, 'b>(node: Node<'a, 'b>, opts: &Options) -> Result<MJImage, Error> {
        Ok(MJImage {
            options: opts.clone(),
            attributes: get_node_attributes(&node),
            context: None,
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

    fn get_style_img(&self) -> Style {
        let mut res = Style::new();
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
        res
    }

    fn get_style_td(&self) -> Style {
        let mut res = Style::new();
        if !self.is_full_width() {
            res.maybe_set("width", self.get_content_width());
        }
        res
    }

    fn get_style_table(&self) -> Style {
        let mut res = Style::new();
        if self.is_full_width() {
            res.set("min-width", "100%");
            res.set("max-width", "100%");
            res.maybe_set("width", self.get_content_width());
        }
        res.set("border-collapse", "collapse");
        res.set("border-spacing", "0px");
        res
    }

    fn render_image(&self) -> String {
        Tag::new("img")
            .maybe_set_attribute("alt", self.get_attribute("alt"))
            .set_attribute(
                "height",
                self.get_size_attribute("height")
                    .and_then(|size| Some(size.value().to_string()))
                    .unwrap_or("auto".into()),
            )
            .maybe_set_attribute("src", self.get_attribute("src"))
            .maybe_set_attribute("srcset", self.get_attribute("srcset"))
            .insert_style(self.get_style_img().inner())
            .maybe_set_attribute("title", self.get_attribute("title"))
            .maybe_set_attribute(
                "width",
                self.get_content_width().and_then(|size| Some(size.value())),
            )
            .maybe_set_attribute("usemap", self.get_attribute("usemap"))
            .closed()
    }

    fn render_link(&self) -> String {
        Tag::new("a")
            .maybe_set_attribute("href", self.get_attribute("href"))
            .maybe_set_attribute("name", self.get_attribute("name"))
            .maybe_set_attribute("rel", self.get_attribute("rel"))
            .maybe_set_attribute("target", self.get_attribute("target"))
            .render(self.render_image())
    }
}

impl Component for MJImage {
    fn update_header(&self, header: &mut Header) {
        let mut style = format!(
            "@media only screen and (max-width:{}) {{\n",
            header.breakpoint().to_string(),
        );
        style.push_str("table.mj-full-width-mobile { width: 100% !important; }\n");
        style.push_str("td.mj-full-width-mobile { width: auto !important; }\n");
        style.push_str("}\n");
        header.add_style(style);
    }

    fn context(&self) -> Option<&Context> {
        self.context.as_ref()
    }

    fn set_context(&mut self, ctx: Context) {
        self.context = Some(ctx.clone());
    }

    fn render(&self, _header: &Header) -> Result<String, Error> {
        let table = Tag::new("table")
            .set_attribute("border", 0)
            .set_attribute("cellpadding", 0)
            .set_attribute("cellspacing", 0)
            .set_attribute("role", "presentation")
            .maybe_set_class(if self.is_fluid_on_mobile() {
                Some("mj-full-width-mobile")
            } else {
                None
            })
            .insert_style(self.get_style_table().inner());
        let tbody = Tag::new("tbody");
        let tr = Tag::new("tr");
        let td = Tag::new("td")
            .insert_style(self.get_style_td().inner())
            .maybe_set_class(if self.is_fluid_on_mobile() {
                Some("mj-full-width-mobile")
            } else {
                None
            });
        let content = if self.get_attribute("href").is_some() {
            self.render_link()
        } else {
            self.render_image()
        };
        Ok(table.render(tbody.render(tr.render(td.render(content)))))
    }
}

impl ComponentWithAttributes for MJImage {
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

    fn source_attributes(&self) -> Option<&HashMap<String, String>> {
        Some(&self.attributes)
    }
}

impl BodyComponent for MJImage {
    fn get_style(&self, name: &str) -> Style {
        match name {
            "img" => self.get_style_img(),
            "td" => self.get_style_td(),
            "table" => self.get_style_table(),
            _ => Style::new(),
        }
    }
}

impl BodyContainedComponent for MJImage {}
impl ComponentWithSizeAttribute for MJImage {}
impl BodyComponentWithBorder for MJImage {}
impl BodyComponentWithPadding for MJImage {}
impl BodyComponentWithBoxWidths for MJImage {}

#[cfg(test)]
pub mod tests {
    use crate::tests::compare_render;

    #[test]
    fn base() {
        compare_render(
            include_str!("../../../test/mj-image.mjml"),
            include_str!("../../../test/mj-image.html"),
        );
    }

    #[test]
    fn with_align() {
        compare_render(
            include_str!("../../../test/mj-image-align.mjml"),
            include_str!("../../../test/mj-image-align.html"),
        );
    }

    #[test]
    fn with_border() {
        compare_render(
            include_str!("../../../test/mj-image-border.mjml"),
            include_str!("../../../test/mj-image-border.html"),
        );
    }

    #[test]
    fn with_border_radius() {
        compare_render(
            include_str!("../../../test/mj-image-border-radius.mjml"),
            include_str!("../../../test/mj-image-border-radius.html"),
        );
    }

    #[test]
    fn with_container_background_color() {
        compare_render(
            include_str!("../../../test/mj-image-container-background-color.mjml"),
            include_str!("../../../test/mj-image-container-background-color.html"),
        );
    }

    #[test]
    fn with_css_class() {
        compare_render(
            include_str!("../../../test/mj-image-class.mjml"),
            include_str!("../../../test/mj-image-class.html"),
        );
    }

    #[test]
    fn with_fluid_on_mobile() {
        compare_render(
            include_str!("../../../test/mj-image-fluid-on-mobile.mjml"),
            include_str!("../../../test/mj-image-fluid-on-mobile.html"),
        );
    }

    #[test]
    fn with_height() {
        compare_render(
            include_str!("../../../test/mj-image-height.mjml"),
            include_str!("../../../test/mj-image-height.html"),
        );
    }

    #[test]
    fn with_href() {
        compare_render(
            include_str!("../../../test/mj-image-href.mjml"),
            include_str!("../../../test/mj-image-href.html"),
        );
    }

    #[test]
    fn with_padding() {
        compare_render(
            include_str!("../../../test/mj-image-padding.mjml"),
            include_str!("../../../test/mj-image-padding.html"),
        );
    }

    #[test]
    fn with_rel() {
        compare_render(
            include_str!("../../../test/mj-image-rel.mjml"),
            include_str!("../../../test/mj-image-rel.html"),
        );
    }

    #[test]
    fn with_title() {
        compare_render(
            include_str!("../../../test/mj-image-title.mjml"),
            include_str!("../../../test/mj-image-title.html"),
        );
    }
}
