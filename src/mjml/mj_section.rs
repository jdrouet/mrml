use super::error::Error;
use super::prelude::Component;
use crate::util::condition::*;
use crate::util::{suffix_css_classes, suffix_unit, Properties};
use crate::{close_tag, closed_tag, open_tag};
use log::debug;
use roxmltree::Node;

const ALLOWED_ATTRIBUTES: [&str; 20] = [
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

pub struct MJSection<'a, 'b> {
    context: Option<Properties>,
    node: Node<'a, 'b>,
}

impl MJSection<'_, '_> {
    fn default_attribute(key: &str) -> Option<&str> {
        debug!("default_attribute {}", key);
        match key {
            "background-repeat" => Some("repeat"),
            "background-size" => Some("auto"),
            "direction" => Some("ltr"),
            "padding" => Some("20px 0"),
            "text-align" => Some("center"),
            "text-padding" => Some("4px 4px 4px 0"),
            _ => None,
        }
    }

    pub fn parse<'a, 'b>(node: Node<'a, 'b>) -> Result<MJSection<'a, 'b>, Error> {
        Ok(MJSection {
            context: None,
            node,
        })
    }

    fn get_attribute(&self, key: &str) -> Option<String> {
        if !ALLOWED_ATTRIBUTES.contains(&key) {
            return None;
        }
        self.node
            .attribute(key)
            .or_else(|| MJSection::default_attribute(key))
            .map(|v| v.to_string())
    }

    fn get_context(&self, name: &str) -> Option<String> {
        self.context.as_ref().and_then(|ctx| ctx.get(name))
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

    fn get_background_style(&self) -> Properties {
        let mut res = Properties::new();
        if self.get_attribute("background-url").is_some() {
            res.maybe_set("background", self.get_background());
        } else {
            res.maybe_set("background", self.get_attribute("background-color"));
            res.maybe_set("background-color", self.get_attribute("background-color"));
        }
        res
    }

    fn get_style(&self, name: &str) -> Properties {
        let mut res = Properties::new();
        let bg_style = self.get_background_style();
        match name {
            "div" => {
                if !self.is_full_width() {
                    res.merge(&bg_style);
                }
                res.set("margin", "0px auto");
                res.maybe_set(
                    "max-width",
                    suffix_unit(self.get_context("container-width"), "px"),
                );
                res.maybe_set("border-radius", self.get_attribute("border-radius"));
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
            let mut attr = Properties::new();
            attr.set("align", "center");
            attr.set("border", "0");
            attr.set("cellpadding", "0");
            attr.set("cellspacing", "0");
            attr.maybe_set(
                "style",
                self.get_context("container-width")
                    .and_then(|v| Some(format!("width:{}px;", v))),
            );
            attr.maybe_set("width", self.get_context("container-width"));
            attr.maybe_set(
                "class",
                suffix_css_classes(self.get_attribute("css-class"), "outlook"),
            );
            res.push(open_tag!("table", attr.as_attributes()));
        };
        res.push(open_tag!("tr"));
        res.push(open_tag!(
            "td",
            (
                "style",
                "font-size:0px;line-height:0px;mso-line-height-rule:exactly;"
            )
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
        unimplemented!();
    }

    fn render_wrapped_children(&self) -> Result<String, Error> {
        let mut res = vec![];
        res.push(START_CONDITIONAL_TAG.into());
        res.push(open_tag!("tr"));
        res.push(END_CONDITIONAL_TAG.into());
        // TODO: render children
        res.push(START_CONDITIONAL_TAG.into());
        res.push(close_tag!("tr"));
        res.push(END_CONDITIONAL_TAG.into());
        Ok(res.join(""))
    }

    fn render_section(&self) -> Result<String, Error> {
        let has_bg = self.has_background();
        let mut res = vec![];
        {
            let mut attr = Properties::new();
            if !self.is_full_width() {
                attr.maybe_set("class", self.get_attribute("css-class"));
            }
            attr.set("style", self.get_style("div").as_style());
            res.push(open_tag!("div", attr.as_attributes()));
        };
        if has_bg {
            res.push(open_tag!(
                "div",
                ("style", self.get_style("inner-div").as_style())
            ));
        }
        {
            // TABLE
            let mut attrs = Properties::new();
            attrs.set("align", "center");
            if !self.is_full_width() {
                attrs.maybe_set("background", self.get_attribute("background-url"));
            }
            attrs.set("border", 0);
            attrs.set("cellpadding", 0);
            attrs.set("cellspacing", 0);
            attrs.set("role", "presentation");
            attrs.set("style", self.get_style("table").as_style());
            res.push(open_tag!("table", attrs.as_attributes()));
        };
        res.push(open_tag!("tbody"));
        res.push(open_tag!("tr"));
        res.push(open_tag!("td", ("style", self.get_style("td").as_style())));
        res.push(START_CONDITIONAL_TAG.into());
        res.push(open_tag!(
            "table",
            ("role", "presentation"),
            ("border", 0),
            ("cellpadding", 0),
            ("cellspacing", 0)
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
            let mut attrs = Properties::new();
            if self.is_full_width() {
                attrs.set("mso-width-percent", "1000");
            } else {
                attrs.maybe_set(
                    "style",
                    self.get_context("container-width")
                        .and_then(|v| Some(format!("width:{}px;", v))),
                );
            }
            attrs.set("xmlns:v", "urn:schemas-microsoft-com:vml");
            attrs.set("fill", "true");
            attrs.set("stroke", "false");
            res.push(open_tag!("v:rect", attrs.as_attributes()));
        };
        {
            let mut attrs = Properties::new();
            attrs.set("origin", "0.5, 0");
            attrs.set("position", "0.5, 0");
            attrs.maybe_set("src", self.get_attribute("background-url"));
            attrs.maybe_set("color", self.get_attribute("background-color"));
            attrs.set("type", "tile");
            res.push(closed_tag!("v:fill", attrs.as_attributes()));
        };
        res.push(open_tag!(
            "v:textbox",
            ("style", "mso-fit-shape-to-text:true"),
            ("inset", "0,0,0,0")
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
    fn default_attribute(key: &str) -> Option<String> {
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

    fn node(&self) -> Option<Node> {
        Some(self.node)
    }

    fn set_context(&mut self, ctx: Properties) {
        self.context = Some(ctx);
    }

    fn render(&self) -> Result<String, Error> {
        if self.is_full_width() {
            self.render_full_width()
        } else {
            self.render_simple()
        }
    }
}
