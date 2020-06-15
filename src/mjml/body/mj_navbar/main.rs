use super::link::MJNavbarLink;
use crate::mjml::body::prelude::*;
use crate::mjml::body::BodyElement;
use crate::mjml::error::Error;
use crate::mjml::prelude::*;
use crate::util::condition::*;
use crate::util::{generate_id, Attributes, Context, Header, Size, Tag};
use crate::Options;
use roxmltree::Node;
use std::collections::HashMap;

fn create_default_attributes() -> Attributes {
    Attributes::new()
        .add("align", "center")
        .add("ico-align", "center")
        .add("ico-open", "&#9776;")
        .add("ico-close", "&#8855;")
        .add("ico-color", "#000000")
        .add("ico-font-family", "Ubuntu, Helvetica, Arial, sans-serif")
        .add("ico-font-size", "30px")
        .add("ico-text-transform", "uppercase")
        .add("ico-padding", "10px")
        .add("ico-text-decoration", "none")
        .add("ico-line-height", "30px")
}

#[derive(Clone, Debug)]
pub struct MJNavbar {
    attributes: Attributes,
    context: Option<Context>,
    children: Vec<BodyElement>,
    id: String,
}

fn create_id() -> String {
    if cfg!(test) {
        "navbar_toggle".into()
    } else {
        generate_id(8)
    }
}

impl MJNavbar {
    pub fn parse<'a, 'b>(node: &Node<'a, 'b>, opts: &Options) -> Result<MJNavbar, Error> {
        let mut attributes = create_default_attributes();
        attributes.merge_node(node);
        let mut result = MJNavbar {
            attributes,
            context: None,
            children: vec![],
            id: create_id(),
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
                let element = MJNavbarLink::parse_link(&child, opts, Some(&attrs))?;
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

    fn set_style_input(&self, tag: Tag) -> Tag {
        tag.set_style("display", "none !important")
            .set_style("max-height", "0")
            .set_style("visibility", "hidden")
    }

    fn set_style_label(&self, tag: Tag) -> Tag {
        tag.set_style("display", "block")
            .set_style("cursor", "pointer")
            .set_style("mso-hide", "all")
            .set_style("-moz-user-select", "none")
            .set_style("user-select", "none")
            .maybe_set_style("color", self.get_attribute("ico-color"))
            .maybe_set_style("font-size", self.get_attribute("ico-font-size"))
            .maybe_set_style("font-family", self.get_attribute("ico-font-family"))
            .maybe_set_style("text-transform", self.get_attribute("ico-text-transform"))
            .maybe_set_style("text-decoration", self.get_attribute("ico-text-decoration"))
            .maybe_set_style("line-height", self.get_attribute("ico-line-height"))
            .maybe_set_style("padding-top", self.get_attribute("ico-padding-top"))
            .maybe_set_style("padding-right", self.get_attribute("ico-padding-right"))
            .maybe_set_style("padding-bottom", self.get_attribute("ico-padding-bottom"))
            .maybe_set_style("padding-left", self.get_attribute("ico-padding-left"))
            .maybe_set_style("padding", self.get_attribute("ico-padding"))
    }

    fn set_style_trigger(&self, tag: Tag) -> Tag {
        tag.set_style("display", "none")
            .set_style("max-height", "0px")
            .set_style("max-width", "0px")
            .set_style("font-size", "0px")
            .set_style("overflow", "hidden")
    }

    fn set_style_ico_close(&self, tag: Tag) -> Tag {
        tag.set_style("display", "none")
            .set_style("mso-hide", "all")
    }

    fn set_style_ico_open(&self, tag: Tag) -> Tag {
        tag.set_style("mso-hide", "all")
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
        let input = self
            .set_style_input(Tag::new("input"))
            .set_class("mj-menu-checkbox")
            .set_attribute("id", self.id.as_str())
            .set_attribute("type", "checkbox");
        let div = self
            .set_style_trigger(Tag::div())
            .set_class("mj-menu-trigger");
        let label = self
            .set_style_label(Tag::new("label"))
            .maybe_set_attribute("align", self.get_attribute("ico-align"))
            .set_class("mj-menu-label")
            .set_attribute("for", self.id.as_str());
        let span_open = self
            .set_style_ico_open(Tag::new("span"))
            .set_class("mj-menu-icon-open");
        let span_close = self
            .set_style_ico_close(Tag::new("span"))
            .set_class("mj-menu-icon-close");
        let mut res: Vec<String> = vec![];
        res.push(mso_negation_conditional_tag(input.closed()));
        res.push(div.open());
        res.push(label.open());
        res.push(span_open.render(self.get_attribute("ico-open").unwrap_or(&"".into())));
        res.push(span_close.render(self.get_attribute("ico-close").unwrap_or(&"".into())));
        res.push(label.close());
        res.push(div.close());
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
        let child_base = Context::new(
            self.get_container_width(),
            self.get_siblings(),
            self.get_raw_siblings(),
            0,
        );
        for (idx, child) in self.children.iter_mut().enumerate() {
            child.set_context(child_base.clone().set_index(idx));
        }
    }

    fn render(&self, header: &Header) -> Result<String, Error> {
        let div = Tag::div().set_class("mj-inline-links");
        let table =
            Tag::table_presentation().maybe_set_attribute("align", self.get_attribute("align"));
        let tr = Tag::tr();
        let mut res: Vec<String> = vec![];
        if self.has_hamburger() {
            res.push(self.render_hamburger(header)?);
        }
        res.push(div.open());
        res.push(START_CONDITIONAL_TAG.into());
        res.push(table.open());
        res.push(tr.open());
        res.push(END_CONDITIONAL_TAG.into());
        for child in self.children.iter() {
            res.push(child.render(header)?);
        }
        res.push(START_CONDITIONAL_TAG.into());
        res.push(tr.close());
        res.push(table.close());
        res.push(END_CONDITIONAL_TAG.into());
        res.push(div.close());
        Ok(res.join(""))
    }
}

impl ComponentWithAttributes for MJNavbar {
    fn attributes(&self) -> Option<&HashMap<String, String>> {
        Some(self.attributes.inner())
    }
}

impl BodyComponent for MJNavbar {
    fn set_style(&self, _name: &str, tag: Tag) -> Tag {
        tag
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
