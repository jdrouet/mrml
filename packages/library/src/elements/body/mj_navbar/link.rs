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
        .add("color", "#000000")
        .add("font-family", "Ubuntu, Helvetica, Arial, sans-serif")
        .add("font-size", "13px")
        .add("font-weight", "normal")
        .add("line-height", "22px")
        .add("padding", "15px 10px")
        .add("target", "_blank")
        .add("text-decoration", "none")
        .add("text-transform", "uppercase");
}

#[derive(Clone, Debug)]
pub struct MJNavbarLink {
    attributes: Attributes,
    context: Option<Context>,
    content: Option<String>,
}

impl MJNavbarLink {
    fn default_attributes<'a>(node: &Node<'a>, header: &Header) -> Attributes {
        header
            .default_attributes()
            .get_attributes(node, DEFAULT_ATTRIBUTES.clone())
    }

    pub fn parse_link<'a>(
        node: &Node<'a>,
        header: &Header,
        extra: Option<&Attributes>,
    ) -> Result<MJNavbarLink, Error> {
        if node.name.as_str() != "mj-navbar-link" {
            return Err(Error::UnexpectedElement(node.name.as_str().into()));
        }
        let content = node
            .children
            .iter()
            .filter_map(|child| child.as_text())
            .map(|child| child.as_str())
            .collect::<String>();
        let mut attributes = Self::default_attributes(node, header);
        if let Some(extra) = extra {
            attributes.merge(extra);
        }
        attributes.merge(node);
        Ok(MJNavbarLink {
            attributes,
            context: None,
            content: if content.is_empty() {
                None
            } else {
                Some(content)
            },
        })
    }

    pub fn parse<'a>(
        node: &Node<'a>,
        header: &Header,
        extra: Option<&Attributes>,
    ) -> Result<MJNavbarLink, Error> {
        let mut attrs = match extra {
            Some(value) => value.into(),
            None => Attributes::default(),
        };
        if attrs.get("text-padding").is_none() {
            attrs.set("text-padding", "4px 4px 4px 0");
        }
        Self::parse_link(node, header, Some(&attrs))
    }

    fn set_style_a(&self, tag: Tag) -> Tag {
        tag.set_style("display", "inline-block")
            .maybe_set_style("color", self.get_attribute("color"))
            .maybe_set_style("font-family", self.get_attribute("font-family"))
            .maybe_set_style("font-size", self.get_attribute("font-size"))
            .maybe_set_style("font-style", self.get_attribute("font-style"))
            .maybe_set_style("font-weight", self.get_attribute("font-weight"))
            .maybe_set_style("letter-spacing", self.get_attribute("letter-spacing"))
            .maybe_set_style("line-height", self.get_attribute("line-height"))
            .maybe_set_style("text-decoration", self.get_attribute("text-decoration"))
            .maybe_set_style("text-transform", self.get_attribute("text-transform"))
            .maybe_set_style("padding", self.get_attribute("padding"))
            .maybe_set_style("padding-top", self.get_attribute("padding-top"))
            .maybe_set_style("padding-right", self.get_attribute("padding-right"))
            .maybe_set_style("padding-bottom", self.get_attribute("padding-bottom"))
            .maybe_set_style("padding-left", self.get_attribute("padding-left"))
    }

    fn set_style_td(&self, tag: Tag) -> Tag {
        tag.maybe_set_style("padding", self.get_attribute("padding"))
            .maybe_set_style("padding-top", self.get_attribute("padding-top"))
            .maybe_set_style("padding-right", self.get_attribute("padding-right"))
            .maybe_set_style("padding-bottom", self.get_attribute("padding-bottom"))
            .maybe_set_style("padding-left", self.get_attribute("padding-left"))
    }

    fn get_link(&self) -> Option<String> {
        self.get_attribute("href").as_ref().and_then(|href| {
            self.get_attribute("navbar-base-url")
                .map(move |base| format!("{}{}", base, href))
                .or_else(|| Some(href.to_string()))
        })
    }

    fn render_content(&self, _header: &Header) -> Result<String, Error> {
        let link = self
            .set_style_a(Tag::new("a"))
            .set_class("mj-link")
            .maybe_set_class(self.get_attribute("css-class"))
            .maybe_set_attribute("href", self.get_link())
            .maybe_set_attribute("rel", self.get_attribute("rel"))
            .maybe_set_attribute("target", self.get_attribute("target"))
            .maybe_set_attribute("name", self.get_attribute("name"));
        Ok(link.render(self.content.as_deref().unwrap_or("")))
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
        self.context = Some(ctx);
    }

    fn render(&self, header: &Header) -> Result<String, Error> {
        let td = self
            .set_style_td(Tag::td())
            .maybe_set_class(suffix_css_classes(
                self.get_attribute("css-class"),
                "outlook",
            ));
        let mut res: Vec<String> = vec![];
        res.push(conditional_tag(td.open()));
        res.push(self.render_content(header)?);
        res.push(conditional_tag(td.close()));
        Ok(res.join(""))
    }
}

impl BodyComponent for MJNavbarLink {
    fn attributes(&self) -> Option<&Attributes> {
        Some(&self.attributes)
    }

    fn get_current_width(&self) -> Option<Size> {
        None
    }

    fn get_children(&self) -> &Vec<BodyElement> {
        &EMPTY_CHILDREN
    }
    fn set_style(&self, name: &str, tag: Tag) -> Tag {
        match name {
            "a" => self.set_style_a(tag),
            "td" => self.set_style_td(tag),
            _ => tag,
        }
    }
}
