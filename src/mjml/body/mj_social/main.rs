use super::element::MJSocialElement;
use crate::mjml::body::prelude::*;
use crate::mjml::body::BodyElement;
use crate::mjml::error::Error;
use crate::mjml::prelude::*;
use crate::util::condition::*;
use crate::util::prelude::*;
use crate::util::{Attributes, Context, Header, Size, Style, Tag};
use crate::Options;
use log::debug;
use roxmltree::Node;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct MJSocial {
    attributes: HashMap<String, String>,
    context: Option<Context>,
    children: Vec<BodyElement>,
}

impl MJSocial {
    pub fn parse<'a, 'b>(node: Node<'a, 'b>, opts: &Options) -> Result<MJSocial, Error> {
        let mut result = MJSocial {
            attributes: get_node_attributes(&node),
            context: None,
            children: vec![],
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
            } else if tag_name != "mj-social-element" {
                return Err(Error::ParseError(format!(
                    "expect only 'mj-social-element', not '{}'",
                    tag_name
                )));
            } else {
                let element = MJSocialElement::parse_social_child(child, opts, Some(&attrs))?;
                result.children.push(BodyElement::MJSocialElement(element));
            }
        }
        Ok(result)
    }

    fn get_style_table_vertical(&self) -> Style {
        let mut res = Style::new();
        res.set("margin", "0px");
        res
    }

    fn get_children_attributes(&self) -> Attributes {
        let mut attrs = Attributes::new();
        attrs.maybe_set("padding", self.get_attribute("inner-padding"));
        attrs.maybe_set("border-radius", self.get_attribute("border-radius"));
        attrs.maybe_set("color", self.get_attribute("color"));
        attrs.maybe_set("font-family", self.get_attribute("font-family"));
        attrs.maybe_set("font-size", self.get_attribute("font-size"));
        attrs.maybe_set("font-weight", self.get_attribute("font-weight"));
        attrs.maybe_set("font-style", self.get_attribute("font-style"));
        attrs.maybe_set("icon-size", self.get_attribute("icon-size"));
        attrs.maybe_set("icon-height", self.get_attribute("icon-height"));
        attrs.maybe_set("icon-padding", self.get_attribute("icon-padding"));
        attrs.maybe_set("text-padding", self.get_attribute("text-padding"));
        attrs.maybe_set("line-height", self.get_attribute("line-height"));
        attrs.maybe_set("text-decoration", self.get_attribute("text-decoration"));
        attrs
    }

    fn is_horizontal(&self) -> bool {
        self.get_attribute("mode")
            .and_then(|mode| Some(mode == "horizontal"))
            .unwrap_or(true)
    }

    fn render_horizontal(&self, header: &Header) -> Result<String, Error> {
        let table =
            Tag::table_presentation().maybe_set_attribute("align", self.get_attribute("align"));
        let tr = Tag::tr();
        let td = Tag::td();
        let inner_table = Tag::table_presentation()
            .maybe_set_attribute("align", self.get_attribute("align"))
            .set_style("display", "inline-table")
            .set_style("float", "none");
        let mut res = vec![];
        res.push(START_CONDITIONAL_TAG.into());
        res.push(table.open());
        res.push(tr.open());
        res.push(END_CONDITIONAL_TAG.into());
        for child in self.children.iter() {
            res.push(conditional_tag(td.open()));
            res.push(inner_table.render(child.render(header)?));
            res.push(conditional_tag(td.close()));
        }
        res.push(START_CONDITIONAL_TAG.into());
        res.push(tr.close());
        res.push(table.close());
        res.push(END_CONDITIONAL_TAG.into());
        Ok(res.join(""))
    }

    fn render_vertical(&self, header: &Header) -> Result<String, Error> {
        let table = Tag::table_presentation().insert_style(self.get_style_table_vertical().inner());
        let mut res = vec![];
        for child in self.children.iter() {
            // TODO set child attributes
            res.push(child.render(header)?);
        }
        Ok(table.render(res.join("")))
    }
}

impl Component for MJSocial {
    fn update_header(&self, header: &mut Header) {
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
        if self.is_horizontal() {
            self.render_horizontal(header)
        } else {
            self.render_vertical(header)
        }
    }
}

impl ComponentWithAttributes for MJSocial {
    fn default_attribute(&self, key: &str) -> Option<String> {
        debug!("default_attribute {}", key);
        match key {
            "align" => Some("center".into()),
            "border-radius" => Some("3px".into()),
            "color" => Some("#333333".into()),
            "font-family" => Some("Ubuntu, Helvetica, Arial, sans-serif".into()),
            "font-size" => Some("13px".into()),
            "icon-size" => Some("20px".into()),
            "line-height" => Some("22px".into()),
            "mode" => Some("horizontal".into()),
            "padding" => Some("10px 25px".into()),
            "text-decoration" => Some("none".into()),
            _ => None,
        }
    }

    fn source_attributes(&self) -> Option<&HashMap<String, String>> {
        Some(&self.attributes)
    }
}

impl BodyComponent for MJSocial {
    fn get_style(&self, name: &str) -> Style {
        match name {
            "table-vertical" => self.get_style_table_vertical(),
            _ => Style::new(),
        }
    }
}

impl ComponentWithChildren for MJSocial {
    fn get_children(&self) -> &Vec<BodyElement> {
        &self.children
    }

    fn get_current_width(&self) -> Option<Size> {
        self.context().and_then(|ctx| ctx.container_width())
    }
}

impl BodyContainedComponent for MJSocial {}

#[cfg(test)]
pub mod tests {
    use crate::tests::compare_render;

    #[test]
    fn base() {
        compare_render(
            include_str!("../../../../test/mj-social.mjml"),
            include_str!("../../../../test/mj-social.html"),
        );
    }

    #[test]
    fn with_align() {
        compare_render(
            include_str!("../../../../test/mj-social-align.mjml"),
            include_str!("../../../../test/mj-social-align.html"),
        );
    }

    #[test]
    fn with_border_radius() {
        compare_render(
            include_str!("../../../../test/mj-social-border-radius.mjml"),
            include_str!("../../../../test/mj-social-border-radius.html"),
        );
    }

    #[test]
    fn with_color() {
        compare_render(
            include_str!("../../../../test/mj-social-color.mjml"),
            include_str!("../../../../test/mj-social-color.html"),
        );
    }

    #[test]
    fn with_class() {
        compare_render(
            include_str!("../../../../test/mj-social-class.mjml"),
            include_str!("../../../../test/mj-social-class.html"),
        );
    }

    #[test]
    fn with_container_background_color() {
        compare_render(
            include_str!("../../../../test/mj-social-container-background-color.mjml"),
            include_str!("../../../../test/mj-social-container-background-color.html"),
        );
    }

    #[test]
    fn with_font_family() {
        compare_render(
            include_str!("../../../../test/mj-social-font-family.mjml"),
            include_str!("../../../../test/mj-social-font-family.html"),
        );
    }

    #[test]
    fn with_font() {
        compare_render(
            include_str!("../../../../test/mj-social-font.mjml"),
            include_str!("../../../../test/mj-social-font.html"),
        );
    }

    #[test]
    fn with_icon() {
        compare_render(
            include_str!("../../../../test/mj-social-icon.mjml"),
            include_str!("../../../../test/mj-social-icon.html"),
        );
    }

    #[test]
    fn with_text() {
        compare_render(
            include_str!("../../../../test/mj-social-text.mjml"),
            include_str!("../../../../test/mj-social-text.html"),
        );
    }

    #[test]
    fn with_padding() {
        compare_render(
            include_str!("../../../../test/mj-social-padding.mjml"),
            include_str!("../../../../test/mj-social-padding.html"),
        );
    }

    #[test]
    fn with_mode() {
        compare_render(
            include_str!("../../../../test/mj-social-mode.mjml"),
            include_str!("../../../../test/mj-social-mode.html"),
        );
    }
}
