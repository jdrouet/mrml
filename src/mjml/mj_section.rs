use super::error::Error;
use super::prelude::*;
use super::Element;
use crate::util::condition::*;
use crate::util::prelude::PropertyMap;
use crate::util::{suffix_css_classes, Attributes, Context, Header, Size, Style};
use crate::{close_tag, closed_tag, open_tag, to_attributes};
use log::debug;
use roxmltree::Node;

const ALLOWED_ATTRIBUTES: [&'static str; 20] = [
    "background-color",
    "background-url",
    "background-repeat",
    "background-size",
    "css-class",
    "border",
    "border-bottom",
    "border-left",
    "border-radius",
    "border-right",
    "border-top",
    "direction",
    "full-width",
    "padding",
    "padding-top",
    "padding-bottom",
    "padding-left",
    "padding-right",
    "text-align",
    "text-padding",
];

#[derive(Clone, Debug)]
pub struct MJSection<'a, 'b> {
    context: Option<Context>,
    node: Node<'a, 'b>,
    children: Vec<Element<'a, 'b>>,
}

impl MJSection<'_, '_> {
    pub fn parse<'a, 'b>(node: Node<'a, 'b>) -> Result<MJSection<'a, 'b>, Error> {
        let mut children = vec![];
        for child in node.children() {
            children.push(Element::parse(child)?);
        }
        Ok(MJSection {
            context: None,
            node,
            children,
        })
    }

    fn get_background(&self) -> Option<String> {
        let mut res = vec![];
        if let Some(color) = self.get_attribute("background-color") {
            res.push(color);
        }
        if let Some(url) = self.get_attribute("background-url") {
            res.push(format!("url({})", url));
            // has default value
            res.push(format!(
                "top center / {}",
                self.get_attribute("background-size").unwrap()
            ));
            // has default value
            res.push(self.get_attribute("background-repeat").unwrap());
        }
        if res.len() > 0 {
            Some(res.join(" "))
        } else {
            None
        }
    }

    fn get_background_style(&self) -> Style {
        let mut res = Style::new();
        if self.get_attribute("background-url").is_some() {
            res.maybe_set("background", self.get_background());
        } else {
            res.maybe_set("background", self.get_attribute("background-color"));
            res.maybe_set("background-color", self.get_attribute("background-color"));
        }
        res
    }

    fn has_background(&self) -> bool {
        self.node.has_attribute("background-url")
    }

    fn is_full_width(&self) -> bool {
        self.node.has_attribute("full-width")
    }

    fn render_before(&self) -> Result<String, Error> {
        let mut res = vec![];
        res.push(START_CONDITIONAL_TAG.into());
        {
            let mut attr = Attributes::new();
            attr.set("align", "center");
            attr.set("border", "0");
            attr.set("cellpadding", "0");
            attr.set("cellspacing", "0");
            attr.maybe_set(
                "style",
                self.get_container_width()
                    .and_then(|v| Some(format!("width:{};", v.to_string()))),
            );
            attr.maybe_set("width", self.get_container_width_value());
            attr.maybe_set(
                "class",
                suffix_css_classes(self.get_attribute("css-class"), "outlook"),
            );
            res.push(open_tag!("table", attr.to_string()));
        };
        res.push(open_tag!("tr"));
        res.push(open_tag!(
            "td",
            to_attributes!((
                "style",
                "font-size:0px;line-height:0px;mso-line-height-rule:exactly;"
            ))
        ));
        res.push(END_CONDITIONAL_TAG.into());
        Ok(res.join(""))
    }

    fn render_after(&self) -> Result<String, Error> {
        let mut res = vec![];
        res.push(START_CONDITIONAL_TAG.into());
        res.push(close_tag!("td"));
        res.push(close_tag!("tr"));
        res.push(close_tag!("table"));
        res.push(END_CONDITIONAL_TAG.into());
        Ok(res.join(""))
    }

    fn render_full_width(&self) -> Result<String, Error> {
        let mut content: Vec<String> = vec![];
        content.push(self.render_before()?);
        content.push(self.render_section()?);
        content.push(self.render_after()?);
        let content = if self.has_background() {
            self.render_with_background(content.join(""))?
        } else {
            content.join("")
        };
        let mut res: Vec<String> = vec![];
        {
            let mut attrs = Attributes::new();
            attrs.set("align", "center");
            attrs.maybe_set("class", self.get_attribute("css-class"));
            attrs.maybe_set("background", self.get_attribute("background-url"));
            attrs.set("border", "0");
            attrs.set("cellpadding", "0");
            attrs.set("cellspacing", "0");
            attrs.set("role", "presentation");
            attrs.set("style", self.get_style("table-full-width").to_string());
            res.push(open_tag!("table", attrs.to_string()));
        };
        res.push(open_tag!("tbody"));
        res.push(open_tag!("tr"));
        res.push(open_tag!("td"));
        res.push(content);
        res.push(close_tag!("td"));
        res.push(close_tag!("tr"));
        res.push(close_tag!("tbody"));
        res.push(close_tag!("table"));
        Ok(res.join(""))
    }

    fn render_wrapped_children(&self) -> Result<String, Error> {
        let mut res = vec![];
        res.push(START_CONDITIONAL_TAG.into());
        res.push(open_tag!("tr"));
        res.push(END_CONDITIONAL_TAG.into());
        for child in self.children.iter() {
            match child {
                Element::Raw(element) => res.push(element.render()?),
                _ => {
                    let mut attrs = Attributes::new();
                    attrs.maybe_set("align", child.get_attribute("align"));
                    attrs.maybe_set(
                        "class",
                        suffix_css_classes(child.get_attribute("css-class"), "outlook"),
                    );
                    attrs.set("style", child.get_style("td-outlook").to_string());
                    res.push(START_CONDITIONAL_TAG.into());
                    res.push(open_tag!("td", attrs.to_string()));
                    res.push(END_CONDITIONAL_TAG.into());
                    res.push(child.render()?);
                    res.push(START_CONDITIONAL_TAG.into());
                    res.push(close_tag!("td"));
                    res.push(END_CONDITIONAL_TAG.into());
                }
            }
        }

        res.push(START_CONDITIONAL_TAG.into());
        res.push(close_tag!("tr"));
        res.push(END_CONDITIONAL_TAG.into());
        Ok(res.join(""))
    }

    fn render_section(&self) -> Result<String, Error> {
        let has_bg = self.has_background();
        let mut res = vec![];
        {
            let mut attr = Attributes::new();
            if !self.is_full_width() {
                attr.maybe_set("class", self.get_attribute("css-class"));
            }
            attr.set("style", self.get_style("div").to_string());
            res.push(open_tag!("div", attr.to_string()));
        };
        if has_bg {
            res.push(open_tag!(
                "div",
                to_attributes!(("style", self.get_style("inner-div").to_string()))
            ));
        }
        {
            // TABLE
            let mut attrs = Attributes::new();
            attrs.set("align", "center");
            if !self.is_full_width() {
                attrs.maybe_set("background", self.get_attribute("background-url"));
            }
            attrs.set("border", 0);
            attrs.set("cellpadding", 0);
            attrs.set("cellspacing", 0);
            attrs.set("role", "presentation");
            attrs.set("style", self.get_style("table").to_string());
            res.push(open_tag!("table", attrs.to_string()));
        };
        res.push(open_tag!("tbody"));
        res.push(open_tag!("tr"));
        res.push(open_tag!(
            "td",
            to_attributes!(("style", self.get_style("td").to_string()))
        ));
        res.push(START_CONDITIONAL_TAG.into());
        res.push(open_tag!(
            "table",
            to_attributes!(
                ("role", "presentation"),
                ("border", 0),
                ("cellpadding", 0),
                ("cellspacing", 0)
            )
        ));
        res.push(END_CONDITIONAL_TAG.into());
        // renderWrappedChildren()
        res.push(self.render_wrapped_children()?);
        //
        res.push(START_CONDITIONAL_TAG.into());
        res.push(close_tag!("table"));
        res.push(END_CONDITIONAL_TAG.into());
        res.push(close_tag!("td"));
        res.push(close_tag!("tr"));
        res.push(close_tag!("tbody"));
        res.push(close_tag!("table"));
        if has_bg {
            res.push(close_tag!("div"));
        }
        res.push(close_tag!("div"));
        Ok(res.join(""))
    }

    fn render_with_background(&self, content: String) -> Result<String, Error> {
        let mut res = vec![];
        res.push(START_CONDITIONAL_TAG.into());
        {
            let mut attrs = Attributes::new();
            if self.is_full_width() {
                attrs.set("mso-width-percent", "1000");
            } else {
                attrs.maybe_set(
                    "style",
                    self.get_container_width()
                        .and_then(|width| Some(format!("width:{};", width.to_string()))),
                );
            }
            attrs.set("xmlns:v", "urn:schemas-microsoft-com:vml");
            attrs.set("fill", "true");
            attrs.set("stroke", "false");
            res.push(open_tag!("v:rect", attrs.to_string()));
        };
        {
            let mut attrs = Attributes::new();
            attrs.set("origin", "0.5, 0");
            attrs.set("position", "0.5, 0");
            attrs.maybe_set("src", self.get_attribute("background-url"));
            attrs.maybe_set("color", self.get_attribute("background-color"));
            attrs.set("type", "tile");
            res.push(closed_tag!("v:fill", attrs.to_string()));
        };
        res.push(open_tag!(
            "v:textbox",
            to_attributes!(
                ("style", "mso-fit-shape-to-text:true"),
                ("inset", "0,0,0,0")
            )
        ));
        res.push(END_CONDITIONAL_TAG.into());
        res.push(content);
        res.push(START_CONDITIONAL_TAG.into());
        res.push(close_tag!("v:textbox"));
        res.push(close_tag!("v:rect"));
        res.push(END_CONDITIONAL_TAG.into());
        Ok(res.join(""))
    }

    fn render_simple(&self) -> Result<String, Error> {
        let section = self.render_section()?;

        let mut res = vec![];
        res.push(self.render_before()?);
        if self.has_background() {
            res.push(self.render_with_background(section)?);
        } else {
            res.push(section);
        }
        res.push(self.render_after()?);
        Ok(res.join(""))
    }
}

impl Component for MJSection<'_, '_> {
    fn allowed_attributes(&self) -> Option<Vec<&'static str>> {
        Some(ALLOWED_ATTRIBUTES.to_vec())
    }

    fn default_attribute(&self, key: &str) -> Option<String> {
        debug!("default_attribute {}", key);
        match key {
            "background-repeat" => Some("repeat".into()),
            "background-size" => Some("auto".into()),
            "direction" => Some("ltr".into()),
            "padding" => Some("20px 0".into()),
            "text-align" => Some("center".into()),
            "text-padding" => Some("4px 4px 4px 0".into()),
            _ => None,
        }
    }

    fn is_raw(&self) -> bool {
        false
    }

    fn to_header(&self) -> Header {
        let mut header = Header::new();
        for child in self.children.iter() {
            header.merge(&child.to_header());
        }
        header
    }

    fn get_style(&self, name: &str) -> Style {
        let mut res = Style::new();
        let bg_style = self.get_background_style();
        match name {
            "div" => {
                if !self.is_full_width() {
                    res.merge(&bg_style);
                }
                res.set("margin", "0px auto");
                res.maybe_set("border-radius", self.get_attribute("border-radius"));
                res.maybe_set("max-width", self.get_container_width_str());
            }
            "inner-div" => {
                res.set("line-height", 0);
                res.set("font-size", 0);
            }
            "table-full-width" => {
                if self.is_full_width() {
                    res.merge(&bg_style);
                }
                res.maybe_set("border-radius", self.get_attribute("border-radius"));
                res.set("width", "100%");
            }
            "table" => {
                if !self.is_full_width() {
                    res.merge(&bg_style);
                }
                res.maybe_set("border-radius", self.get_attribute("border-radius"));
                res.set("width", "100%");
            }
            "td" => {
                res.maybe_set("border", self.get_attribute("border"));
                res.maybe_set("border-bottom", self.get_attribute("border-bottom"));
                res.maybe_set("border-left", self.get_attribute("border-left"));
                res.maybe_set("border-right", self.get_attribute("border-right"));
                res.maybe_set("border-top", self.get_attribute("border-top"));
                res.maybe_set("direction", self.get_attribute("direction"));
                res.set("font-size", "0px");
                res.maybe_set("padding", self.get_attribute("padding"));
                res.maybe_set("padding-bottom", self.get_attribute("padding-bottom"));
                res.maybe_set("padding-left", self.get_attribute("padding-left"));
                res.maybe_set("padding-right", self.get_attribute("padding-right"));
                res.maybe_set("padding-top", self.get_attribute("padding-top"));
                res.maybe_set("text-align", self.get_attribute("text-align"));
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
        let sibling = self.get_siblings();
        let raw_sibling = self.get_raw_siblings();
        let container_width = self.get_container_width();
        for (idx, child) in self.children.iter_mut().enumerate() {
            let mut child_ctx =
                Context::from(&ctx, container_width.clone(), sibling, raw_sibling, idx);
            child_ctx.set("index", idx);
            child.set_context(child_ctx);
        }
    }

    fn render(&self) -> Result<String, Error> {
        if self.is_full_width() {
            self.render_full_width()
        } else {
            self.render_simple()
        }
    }
}

impl ComponentWithChildren for MJSection<'_, '_> {
    fn get_children(&self) -> &Vec<Element> {
        &self.children
    }

    fn get_current_width(&self) -> Option<Size> {
        self.context().and_then(|ctx| ctx.container_width())
    }
}

impl ContainedComponent for MJSection<'_, '_> {}

#[cfg(test)]
pub mod tests {
    use crate::tests::compare_render;

    #[test]
    fn with_body_width() {
        compare_render(
            include_str!("../../test/mj-section-body-width.mjml"),
            include_str!("../../test/mj-section-body-width.html"),
        );
    }

    #[test]
    fn base() {
        compare_render(
            include_str!("../../test/mj-section.mjml"),
            include_str!("../../test/mj-section.html"),
        );
    }

    #[test]
    fn with_background_color() {
        compare_render(
            include_str!("../../test/mj-section-background-color.mjml"),
            include_str!("../../test/mj-section-background-color.html"),
        );
    }

    #[test]
    fn with_background_url() {
        compare_render(
            include_str!("../../test/mj-section-background-url.mjml"),
            include_str!("../../test/mj-section-background-url.html"),
        );
    }

    #[test]
    fn with_background_url_full() {
        compare_render(
            include_str!("../../test/mj-section-background-url-full.mjml"),
            include_str!("../../test/mj-section-background-url-full.html"),
        );
    }

    #[test]
    fn with_border() {
        compare_render(
            include_str!("../../test/mj-section-border.mjml"),
            include_str!("../../test/mj-section-border.html"),
        );
    }

    #[test]
    fn with_border_radius() {
        compare_render(
            include_str!("../../test/mj-section-border-radius.mjml"),
            include_str!("../../test/mj-section-border-radius.html"),
        );
    }

    #[test]
    fn with_css_class() {
        compare_render(
            include_str!("../../test/mj-section-class.mjml"),
            include_str!("../../test/mj-section-class.html"),
        );
    }

    #[test]
    fn with_direction() {
        compare_render(
            include_str!("../../test/mj-section-direction.mjml"),
            include_str!("../../test/mj-section-direction.html"),
        );
    }

    #[test]
    fn with_full_width() {
        compare_render(
            include_str!("../../test/mj-section-full-width.mjml"),
            include_str!("../../test/mj-section-full-width.html"),
        );
    }

    #[test]
    fn with_padding() {
        compare_render(
            include_str!("../../test/mj-section-padding.mjml"),
            include_str!("../../test/mj-section-padding.html"),
        );
    }

    #[test]
    fn with_text_align() {
        compare_render(
            include_str!("../../test/mj-section-text-align.mjml"),
            include_str!("../../test/mj-section-text-align.html"),
        );
    }
}
