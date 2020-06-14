use super::{MJAccordionText, MJAccordionTitle};
use crate::mjml::body::prelude::*;
use crate::mjml::error::Error;
use crate::mjml::prelude::*;
use crate::util::condition::*;
use crate::util::{Context, Header, Tag};
use crate::Options;
use log::debug;
use roxmltree::Node;
use std::collections::HashMap;

const CHILDREN_ATTR: [&'static str; 9] = [
    "border",
    "icon-align",
    "icon-width",
    "icon-height",
    "icon-position",
    "icon-wrapped-url",
    "icon-wrapped-alt",
    "icon-unwrapped-url",
    "icon-unwrapped-alt",
];

#[derive(Clone, Debug)]
pub struct MJAccordionElement {
    attributes: HashMap<String, String>,
    context: Option<Context>,
    title: Option<MJAccordionTitle>,
    text: Option<MJAccordionText>,
}

impl MJAccordionElement {
    pub fn parse<'a, 'b>(
        node: &Node<'a, 'b>,
        opts: &Options,
        attrs: &HashMap<String, String>,
    ) -> Result<MJAccordionElement, Error> {
        if node.tag_name().name() != "mj-accordion-element" {
            return Err(Error::ParseError(format!(
                "element should be 'mj-accordion-element' no '{}'",
                node.tag_name().name()
            )));
        }
        let mut attributes = attrs.clone();
        add_node_attributes(&mut attributes, &node);
        let mut element = MJAccordionElement {
            attributes,
            context: None,
            title: None,
            text: None,
        };
        let children_attr = element.get_children_attributes();
        for child in node.children() {
            match child.tag_name().name() {
                "mj-accordion-title" => {
                    element.title = Some(MJAccordionTitle::parse(&child, opts, &children_attr)?);
                }
                "mj-accordion-text" => {
                    element.text = Some(MJAccordionText::parse(&child, opts, &children_attr)?);
                }
                _ => (),
            };
        }
        Ok(element)
    }

    fn get_children_attributes(&self) -> HashMap<String, String> {
        let mut result: HashMap<String, String> = HashMap::new();
        for key in CHILDREN_ATTR.iter() {
            if let Some(value) = self.get_attribute(key) {
                result.insert(key.to_string(), value.to_string());
            }
        }
        result
    }

    fn render_text(
        &self,
        header: &Header,
        attributes: &HashMap<String, String>,
    ) -> Result<String, Error> {
        match self.text.as_ref() {
            Some(content) => content.render(header),
            None => MJAccordionText::new(attributes.clone()).render(header),
        }
    }

    fn render_title(
        &self,
        header: &Header,
        attributes: &HashMap<String, String>,
    ) -> Result<String, Error> {
        match self.title.as_ref() {
            Some(content) => content.render(header),
            None => MJAccordionTitle::new(attributes.clone()).render(header),
        }
    }

    fn render_children(&self, header: &Header) -> Result<String, Error> {
        let children_attr = self.get_children_attributes();
        let title = self.render_title(header, &children_attr)?;
        let text = self.render_text(header, &children_attr)?;
        Ok(title + &text)
    }
}

impl Component for MJAccordionElement {
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
        let input = negation_conditional_tag(
            Tag::new("input")
                .set_attribute("type", "checkbox")
                .set_class("mj-accordion-checkbox")
                .set_style("display", "none")
                .closed(),
        );
        let div = Tag::div().render(self.render_children(header)?);
        let label = Tag::new("label")
            .set_class("mj-accordion-element")
            .set_style("font-size", "13px")
            .maybe_set_style("font-family", self.get_attribute("font-family"))
            .render(input + &div);
        let td = Tag::td()
            .set_style("padding", "0px")
            .maybe_set_style("background-color", self.get_attribute("background-color"))
            .render(label);
        let tr = Tag::tr()
            .maybe_set_class(self.get_attribute("css-class"))
            .render(td);
        Ok(tr)
    }
}

impl ComponentWithAttributes for MJAccordionElement {
    fn default_attribute(&self, key: &str) -> Option<String> {
        debug!("default_attribute {}", key);
        match key {
            _ => None,
        }
    }

    fn source_attributes(&self) -> Option<&HashMap<String, String>> {
        Some(&self.attributes)
    }
}

impl BodyComponent for MJAccordionElement {}
impl BodyContainedComponent for MJAccordionElement {}
impl ComponentWithSizeAttribute for MJAccordionElement {}
