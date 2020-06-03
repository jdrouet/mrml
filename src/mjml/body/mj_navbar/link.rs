use crate::mjml::body::prelude::*;
use crate::mjml::error::Error;
use crate::mjml::prelude::*;
use crate::util::condition::*;
use crate::util::prelude::PropertyMap;
use crate::util::{suffix_css_classes, Attributes, Context, Header, Style};
use crate::Options;
use crate::{close_tag, open_tag};
use log::debug;
use roxmltree::Node;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct MJNavbarLink {
    attributes: HashMap<String, String>,
    context: Option<Context>,
    content: Option<String>,
}

impl MJNavbarLink {
    pub fn parse_link<'a, 'b>(
        node: Node<'a, 'b>,
        _opts: &Options,
        extra: Option<&Attributes>,
    ) -> Result<MJNavbarLink, Error> {
        if node.tag_name().name() != "mj-navbar-link" {
            return Err(Error::ParseError(format!(
                "element should be 'mj-navbar-link' no '{}'",
                node.tag_name().name()
            )));
        }
        let content: Vec<&str> = node
            .children()
            .filter(|child| child.is_text())
            .map(|child| child.text())
            .filter(|child| child.is_some())
            .map(|child| child.unwrap())
            .collect();
        let content = if content.len() == 0 {
            None
        } else {
            Some(content.join(""))
        };
        let mut attributes = match extra {
            Some(value) => value.inner().clone(),
            None => HashMap::new(),
        };
        add_node_attributes(&mut attributes, &node);
        Ok(MJNavbarLink {
            attributes,
            context: None,
            content,
        })
    }

    pub fn parse<'a, 'b>(
        node: Node<'a, 'b>,
        opts: &Options,
        extra: Option<&Attributes>,
    ) -> Result<MJNavbarLink, Error> {
        let mut attrs = match extra {
            Some(value) => value.into(),
            None => Attributes::new(),
        };
        if attrs.get("text-padding").is_none() {
            attrs.set("text-padding", "4px 4px 4px 0");
        }
        Self::parse_link(node, opts, Some(&attrs))
    }

    fn get_style_a(&self) -> Style {
        let mut res = Style::new();
        res.set("display", "inline-block");
        res.maybe_set("color", self.get_attribute("color"));
        res.maybe_set("font-family", self.get_attribute("font-family"));
        res.maybe_set("font-size", self.get_attribute("font-size"));
        res.maybe_set("font-style", self.get_attribute("font-style"));
        res.maybe_set("font-weight", self.get_attribute("font-weight"));
        res.maybe_set("letter-spacing", self.get_attribute("letter-spacing"));
        res.maybe_set("line-height", self.get_attribute("line-height"));
        res.maybe_set("text-decoration", self.get_attribute("text-decoration"));
        res.maybe_set("text-transform", self.get_attribute("text-transform"));
        res.maybe_set("padding", self.get_attribute("padding"));
        res.maybe_set("padding-top", self.get_attribute("padding-top"));
        res.maybe_set("padding-right", self.get_attribute("padding-right"));
        res.maybe_set("padding-bottom", self.get_attribute("padding-bottom"));
        res.maybe_set("padding-left", self.get_attribute("padding-left"));
        res
    }

    fn get_style_td(&self) -> Style {
        let mut res = Style::new();
        res.maybe_set("padding", self.get_attribute("padding"));
        res.maybe_set("padding-top", self.get_attribute("padding-top"));
        res.maybe_set("padding-right", self.get_attribute("padding-right"));
        res.maybe_set("padding-bottom", self.get_attribute("padding-bottom"));
        res.maybe_set("padding-left", self.get_attribute("padding-left"));
        res
    }

    fn get_link(&self) -> Option<String> {
        self.get_attribute("href").and_then(|href| {
            self.get_attribute("navbar-base-url")
                .and_then(move |base| Some(format!("{}{}", base, href)))
        })
    }

    fn render_content(&self, _header: &Header) -> Result<String, Error> {
        let mut res = vec![];
        let mut classes = vec![];
        classes.push("mj-link".into());
        if let Some(css_class) = self.get_attribute("css-class") {
            classes.push(css_class);
        }
        let classes = classes.join(" ");
        let mut attrs = Attributes::new();
        attrs.set("class", classes);
        attrs.set("style", self.get_style_a().to_string());
        attrs.maybe_set("href", self.get_link());
        attrs.maybe_set("rel", self.get_attribute("rel"));
        attrs.maybe_set("target", self.get_attribute("target"));
        attrs.maybe_set("name", self.get_attribute("name"));
        res.push(open_tag!("a", attrs.to_string()));
        if let Some(content) = self.content.as_ref() {
            res.push(content.clone());
        }
        res.push(close_tag!("a"));
        Ok(res.join(""))
    }
}

impl Component for MJNavbarLink {
    fn update_header(&self, header: &mut Header) {
        header.maybe_add_font_families(self.get_attribute("font-family"));
    }

    fn context(&self) -> Option<&Context> {
        self.context.as_ref()
    }

    fn set_context(&mut self, ctx: Context) {
        self.context = Some(ctx.clone());
    }

    fn render(&self, header: &Header) -> Result<String, Error> {
        let mut res: Vec<String> = vec![];
        res.push(START_CONDITIONAL_TAG.into());
        {
            let mut attrs = Attributes::new();
            attrs.set("style", self.get_style_td());
            attrs.maybe_set(
                "class",
                suffix_css_classes(self.get_attribute("css-class"), "outlook"),
            );
            res.push(open_tag!("td", attrs.to_string()));
        }
        res.push(END_CONDITIONAL_TAG.into());
        res.push(self.render_content(header)?);
        res.push(START_CONDITIONAL_TAG.into());
        res.push(close_tag!("td"));
        res.push(END_CONDITIONAL_TAG.into());
        Ok(res.join(""))
    }
}

impl ComponentWithAttributes for MJNavbarLink {
    fn default_attribute(&self, key: &str) -> Option<String> {
        debug!("default_attribute {}", key);
        match key {
            "color" => Some("#000000".into()),
            "font-family" => Some("Ubuntu, Helvetica, Arial, sans-serif".into()),
            "font-size" => Some("13px".into()),
            "font-weight" => Some("normal".into()),
            "line-height" => Some("22px".into()),
            "padding" => Some("15px 10px".into()),
            "target" => Some("_blank".into()),
            "text-decoration" => Some("none".into()),
            "text-transform" => Some("uppercase".into()),
            _ => None,
        }
    }

    fn source_attributes(&self) -> Option<&HashMap<String, String>> {
        Some(&self.attributes)
    }
}

impl BodyComponent for MJNavbarLink {
    fn get_style(&self, name: &str) -> Style {
        match name {
            "a" => self.get_style_a(),
            "td" => self.get_style_td(),
            _ => Style::new(),
        }
    }
}

impl BodyContainedComponent for MJNavbarLink {}
impl ComponentWithSizeAttribute for MJNavbarLink {}
