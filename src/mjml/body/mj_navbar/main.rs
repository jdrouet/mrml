use super::link::MJNavbarLink;
use crate::mjml::body::prelude::*;
use crate::mjml::body::BodyElement;
use crate::mjml::error::Error;
use crate::mjml::prelude::*;
use crate::util::condition::*;
use crate::util::prelude::PropertyMap;
use crate::util::{Attributes, Context, Header, Size, Style};
use crate::Options;
use crate::{close_tag, closed_tag, open_tag, to_attributes};
use log::debug;
use roxmltree::Node;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct MJNavbar {
    attributes: HashMap<String, String>,
    context: Option<Context>,
    children: Vec<BodyElement>,
    id: String,
}

impl MJNavbar {
    pub fn parse<'a, 'b>(node: Node<'a, 'b>, opts: &Options) -> Result<MJNavbar, Error> {
        let mut result = MJNavbar {
            attributes: get_node_attributes(&node),
            context: None,
            children: vec![],
            id: String::from("navbar_toggle"),
        };
        let attrs = result.get_children_attributes();
        for child in node.children() {
            let tag_name = child.tag_name().name();
            if tag_name == "" {
                if let Some(content) = child.text() {
                    if content.len() == 0 {
                        continue;
                    }
                }
            } else if tag_name != "mj-navbar-link" {
                return Err(Error::ParseError(format!(
                    "expect only 'mj-navbar-link', not '{}'",
                    tag_name
                )));
            } else {
                let element = MJNavbarLink::parse_link(child, opts, Some(&attrs))?;
                result.children.push(BodyElement::MJNavbarLink(element));
            }
        }
        Ok(result)
    }

    fn get_children_attributes(&self) -> Attributes {
        let mut attrs = Attributes::new();
        attrs.maybe_set("navbar-base-url", self.get_attribute("base-url"));
        attrs
    }

    fn get_style_input(&self) -> Style {
        let mut res = Style::new();
        res.set("display", "none !important");
        res.set("max-height", "0");
        res.set("visibility", "hidden");
        res
    }

    fn get_style_label(&self) -> Style {
        let mut res = Style::new();
        res.set("display", "block");
        res.set("cursor", "pointer");
        res.set("mso-hide", "all");
        res.set("-moz-user-select", "none");
        res.set("user-select", "none");
        res.maybe_set("color", self.get_attribute("ico-color"));
        res.maybe_set("font-size", self.get_attribute("ico-font-size"));
        res.maybe_set("font-family", self.get_attribute("ico-font-family"));
        res.maybe_set("text-transform", self.get_attribute("ico-text-transform"));
        res.maybe_set("text-decoration", self.get_attribute("ico-text-decoration"));
        res.maybe_set("line-height", self.get_attribute("ico-line-height"));
        res.maybe_set("padding-top", self.get_attribute("ico-padding-top"));
        res.maybe_set("padding-right", self.get_attribute("ico-padding-right"));
        res.maybe_set("padding-bottom", self.get_attribute("ico-padding-bottom"));
        res.maybe_set("padding-left", self.get_attribute("ico-padding-left"));
        res.maybe_set("padding", self.get_attribute("ico-padding"));
        res
    }

    fn get_style_trigger(&self) -> Style {
        let mut res = Style::new();
        res.set("display", "none");
        res.set("max-height", "0px");
        res.set("max-width", "0px");
        res.set("font-size", "0px");
        res.set("overflow", "hidden");
        res
    }

    fn get_style_ico_close(&self) -> Style {
        let mut res = Style::new();
        res.set("display", "none");
        res.set("mso-hide", "all");
        res
    }

    fn get_style_ico_open(&self) -> Style {
        let mut res = Style::new();
        res.set("mso-hide", "all");
        res
    }

    fn has_hamburger(&self) -> bool {
        self.get_attribute("hamburger")
            .and_then(|value| {
                if value == "hamburger" {
                    Some(true)
                } else {
                    None
                }
            })
            .is_some()
    }

    fn render_hamburger(&self, _header: &Header) -> Result<String, Error> {
        let mut res: Vec<String> = vec![];
        res.push(START_MSO_NEGATION_CONDITIONAL_TAG.into());
        res.push(closed_tag!(
            "input",
            to_attributes!(
                ("class", "mj-menu-checkbox"),
                ("id", self.id),
                ("style", self.get_style_input().to_string()),
                ("type", "checkbox")
            )
        ));
        res.push(END_NEGATION_CONDITIONAL_TAG.into());
        res.push(open_tag!(
            "div",
            to_attributes!(
                ("class", "mj-menu-trigger"),
                ("style", self.get_style_trigger().to_string())
            )
        ));
        {
            let mut attrs = Attributes::new();
            attrs.maybe_set("align", self.get_attribute("ico-align"));
            attrs.set("class", "mj-menu-label");
            attrs.set("for", self.id.clone());
            attrs.set("style", self.get_style_label());
            res.push(open_tag!("label", attrs.to_string()));
        }
        res.push(open_tag!(
            "span",
            to_attributes!(
                ("class", "mj-menu-icon-open"),
                ("style", self.get_style_ico_open().to_string())
            )
        ));
        if let Some(value) = self.get_attribute("ico-open") {
            res.push(value);
        }
        res.push(close_tag!("span"));
        res.push(open_tag!(
            "span",
            to_attributes!(
                ("class", "mj-menu-icon-close"),
                ("style", self.get_style_ico_close().to_string())
            )
        ));
        if let Some(value) = self.get_attribute("ico-close") {
            res.push(value);
        }
        res.push(close_tag!("span"));
        res.push(close_tag!("label"));
        res.push(close_tag!("div"));
        Ok(res.join(""))
    }
}

impl Component for MJNavbar {
    fn update_header(&self, header: &mut Header) {
        header.add_style(format!(r#"
        noinput.mj-menu-checkbox {{ display:block!important; max-height:none!important; visibility:visible!important; }}
        @media only screen and (max-width:{}) {{
          .mj-menu-checkbox[type="checkbox"] ~ .mj-inline-links {{ display:none!important; }}
          .mj-menu-checkbox[type="checkbox"]:checked ~ .mj-inline-links,
          .mj-menu-checkbox[type="checkbox"] ~ .mj-menu-trigger {{ display:block!important; max-width:none!important; max-height:none!important; font-size:inherit!important; }}
          .mj-menu-checkbox[type="checkbox"] ~ .mj-inline-links > a {{ display:block!important; }}
          .mj-menu-checkbox[type="checkbox"]:checked ~ .mj-menu-trigger .mj-menu-icon-close {{ display:block!important; }}
          .mj-menu-checkbox[type="checkbox"]:checked ~ .mj-menu-trigger .mj-menu-icon-open {{ display:none!important; }}
        }}
        "#, header.breakpoint().to_string()));
        for child in self.children.iter() {
            child.update_header(header);
        }
    }

    fn context(&self) -> Option<&Context> {
        self.context.as_ref()
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

    fn render(&self, header: &Header) -> Result<String, Error> {
        let mut res: Vec<String> = vec![];
        if self.has_hamburger() {
            res.push(self.render_hamburger(header)?);
        }
        res.push(open_tag!(
            "div",
            to_attributes!(("class", "mj-inline-links"))
        ));
        res.push(START_CONDITIONAL_TAG.into());
        {
            let mut attrs = Attributes::new();
            attrs.set("border", "0");
            attrs.set("cellpadding", "0");
            attrs.set("cellspacing", "0");
            attrs.set("role", "presentation");
            attrs.maybe_set("align", self.get_attribute("align"));
            res.push(open_tag!("table", attrs.to_string()));
        }
        res.push(open_tag!("tr"));
        res.push(END_CONDITIONAL_TAG.into());
        for child in self.children.iter() {
            res.push(child.render(header)?);
        }
        res.push(START_CONDITIONAL_TAG.into());
        res.push(close_tag!("tr"));
        res.push(close_tag!("table"));
        res.push(END_CONDITIONAL_TAG.into());
        res.push(close_tag!("div"));
        Ok(res.join(""))
    }
}

impl ComponentWithAttributes for MJNavbar {
    fn default_attribute(&self, key: &str) -> Option<String> {
        debug!("default_attribute {}", key);
        match key {
            "align" => Some("center".into()),
            "ico-align" => Some("center".into()),
            "ico-open" => Some("&#9776;".into()),
            "ico-close" => Some("&#8855;".into()),
            "ico-color" => Some("#000000".into()),
            "ico-font-family" => Some("Ubuntu, Helvetica, Arial, sans-serif".into()),
            "ico-font-size" => Some("30px".into()),
            "ico-text-transform" => Some("uppercase".into()),
            "ico-padding" => Some("10px".into()),
            "ico-text-decoration" => Some("none".into()),
            "ico-line-height" => Some("30px".into()),
            _ => None,
        }
    }

    fn source_attributes(&self) -> Option<&HashMap<String, String>> {
        Some(&self.attributes)
    }
}

impl BodyComponent for MJNavbar {
    fn get_style(&self, name: &str) -> Style {
        match name {
            _ => Style::new(),
        }
    }
}

impl ComponentWithChildren for MJNavbar {
    fn get_children(&self) -> &Vec<BodyElement> {
        &self.children
    }

    fn get_current_width(&self) -> Option<Size> {
        self.context().and_then(|ctx| ctx.container_width())
    }
}

impl BodyContainedComponent for MJNavbar {}

#[cfg(test)]
pub mod tests {
    use crate::tests::compare_render;

    #[test]
    fn base() {
        compare_render(
            include_str!("../../../../test/mj-navbar.mjml"),
            include_str!("../../../../test/mj-navbar.html"),
        );
    }

    #[test]
    fn with_align_and_class() {
        compare_render(
            include_str!("../../../../test/mj-navbar-align-class.mjml"),
            include_str!("../../../../test/mj-navbar-align-class.html"),
        );
    }

    #[test]
    fn with_ico_and_link() {
        compare_render(
            include_str!("../../../../test/mj-navbar-ico.mjml"),
            include_str!("../../../../test/mj-navbar-ico.html"),
        );
    }
}
