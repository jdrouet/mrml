use super::element::MJSocialElement;
use crate::elements::body::prelude::*;
use crate::elements::body::BodyElement;
use crate::elements::error::Error;
use crate::elements::prelude::*;
use crate::parser::Node;
use crate::util::attributes::*;
use crate::util::condition::*;
use crate::util::context::Context;
use crate::util::header::Header;
use crate::util::size::Size;
use crate::util::tag::Tag;

lazy_static! {
    static ref DEFAULT_ATTRIBUTES: Attributes = Attributes::default()
        .add("align", "center")
        .add("border-radius", "3px")
        .add("color", "#333333")
        .add("font-family", "Ubuntu, Helvetica, Arial, sans-serif")
        .add("font-size", "13px")
        .add("icon-size", "20px")
        .add("line-height", "22px")
        .add("mode", "horizontal")
        .add("padding", "10px 25px")
        .add("text-decoration", "none");
}

#[derive(Clone, Debug)]
pub struct MJSocial {
    attributes: Attributes,
    context: Option<Context>,
    children: Vec<BodyElement>,
}

impl MJSocial {
    fn default_attributes<'a>(node: &Node<'a>, header: &Header) -> Attributes {
        header
            .default_attributes()
            .get_attributes(node, DEFAULT_ATTRIBUTES.clone())
    }

    pub fn parse<'a>(node: &Node<'a>, header: &Header) -> Result<MJSocial, Error> {
        let mut result = MJSocial {
            attributes: Self::default_attributes(node, header).concat(node),
            context: None,
            children: vec![],
        };
        let attrs = result.get_children_attributes();
        for child in node.children.iter() {
            if let Some(child_node) = child.as_node() {
                let tag_name = child_node.name.as_str();
                if tag_name != "mj-social-element" {
                    return Err(Error::UnexpectedElement(tag_name.into()));
                } else {
                    let element =
                        MJSocialElement::parse_social_child(&child_node, header, Some(&attrs))?;
                    result.children.push(BodyElement::MJSocialElement(element));
                }
            }
        }
        Ok(result)
    }

    fn set_style_table_vertical(&self, tag: Tag) -> Tag {
        tag.set_style("margin", "0px")
    }

    fn get_children_attributes(&self) -> Attributes {
        let mut attrs = Attributes::default();
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
            .map(|mode| mode == "horizontal")
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
        let table = self.set_style_table_vertical(Tag::table_presentation());
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
        self.context = Some(ctx);
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
        if self.is_horizontal() {
            self.render_horizontal(header)
        } else {
            self.render_vertical(header)
        }
    }
}

impl BodyComponent for MJSocial {
    fn get_children(&self) -> &Vec<BodyElement> {
        &self.children
    }

    fn get_current_width(&self) -> Option<Size> {
        self.context().and_then(|ctx| ctx.container_width())
    }

    fn attributes(&self) -> Option<&Attributes> {
        Some(&self.attributes)
    }

    fn set_style(&self, name: &str, tag: Tag) -> Tag {
        match name {
            "table-vertical" => self.set_style_table_vertical(tag),
            _ => tag,
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::tests::{compare_render, compare_render_with_options};

    #[test]
    fn base() {
        compare_render(
            include_str!("../../../../test/mj-social.mjml"),
            include_str!("../../../../test/mj-social.html"),
        );
    }

    #[test]
    fn different_origin() {
        let result = include_str!("../../../../test/mj-social.html").replace(
            "https://www.mailjet.com/images/theme/v1/icons/ico-social/",
            "http://my.origin.rust/",
        );
        let mut opts = crate::Options::default();
        opts.social_icon_origin = String::from("http://my.origin.rust/");
        compare_render_with_options(
            include_str!("../../../../test/mj-social.mjml"),
            result.as_str(),
            opts,
        );
    }

    #[test]
    fn link() {
        compare_render(
            include_str!("../../../../test/mj-social-link.mjml"),
            include_str!("../../../../test/mj-social-link.html"),
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
