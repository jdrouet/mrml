use crate::mjml::body::prelude::*;
use crate::mjml::body::raw::RawElement;
use crate::mjml::error::Error;
use crate::mjml::prelude::*;
use crate::util::{Context, Header, Tag};
use roxmltree::Node;
use std::collections::HashMap;

fn create_default_attributes() -> HashMap<String, String> {
    let mut res = HashMap::new();
    res.insert("font-size".into(), "13px".into());
    res.insert("padding".into(), "16px".into());
    res
}

fn append_attributes(target: &mut HashMap<String, String>, attrs: &HashMap<String, String>) {
    for (key, value) in attrs.iter() {
        target.insert(key.clone(), value.clone());
    }
}

#[derive(Clone, Debug)]
pub struct MJAccordionText {
    attributes: HashMap<String, String>,
    context: Option<Context>,
    children: Vec<RawElement>,
}

impl MJAccordionText {
    pub fn parse<'a, 'b>(
        node: &Node<'a, 'b>,
        header: &Header,
        attrs: &HashMap<String, String>,
    ) -> Result<MJAccordionText, Error> {
        if node.tag_name().name() != "mj-accordion-text" {
            return Err(Error::ParseError(format!(
                "element should be 'mj-accordion-text' not '{}'",
                node.tag_name().name()
            )));
        }
        let mut attributes = create_default_attributes();
        append_attributes(&mut attributes, attrs);
        add_node_attributes(&mut attributes, &node);
        let mut element = MJAccordionText::new(attributes);
        for child in node.children() {
            element
                .children
                .push(RawElement::conditional_parse(&child, header, true)?);
        }
        Ok(element)
    }

    pub fn new(attributes: HashMap<String, String>) -> Self {
        MJAccordionText {
            attributes,
            context: None,
            children: vec![],
        }
    }

    fn render_children(&self, header: &Header) -> Result<String, Error> {
        let mut res = vec![];
        for child in self.children.iter() {
            res.push(child.render(header)?);
        }
        Ok(res.join(""))
    }

    fn render_content(&self, header: &Header) -> Result<String, Error> {
        Ok(Tag::td()
            .maybe_set_class(self.get_attribute("css-class"))
            .maybe_set_style("background", self.get_attribute("background-color"))
            .maybe_set_style("color", self.get_attribute("color"))
            .maybe_set_style("font-family", self.get_attribute("font-family"))
            .maybe_set_style("font-size", self.get_attribute("font-size"))
            .maybe_set_style("padding", self.get_attribute("padding"))
            .maybe_set_style("padding-top", self.get_attribute("padding-top"))
            .maybe_set_style("padding-right", self.get_attribute("padding-right"))
            .maybe_set_style("padding-bottom", self.get_attribute("padding-bottom"))
            .maybe_set_style("padding-left", self.get_attribute("padding-left"))
            .render(self.render_children(header)?))
    }
}

impl Component for MJAccordionText {
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
        let tr = Tag::tr().render(self.render_content(header)?);
        let tbody = Tag::tbody().render(tr);
        let table = Tag::table()
            .set_attribute("cellspacing", 0)
            .set_attribute("cellpadding", 0)
            .set_style("width", "100%")
            .maybe_set_style("border-bottom", self.get_attribute("border"))
            .render(tbody);
        let div = Tag::div().set_class("mj-accordion-content").render(table);
        Ok(div)
    }
}

impl ComponentWithAttributes for MJAccordionText {
    fn attributes(&self) -> Option<&HashMap<String, String>> {
        Some(&self.attributes)
    }
}

impl BodyComponent for MJAccordionText {}
impl BodyContainedComponent for MJAccordionText {}
impl ComponentWithSizeAttribute for MJAccordionText {}
