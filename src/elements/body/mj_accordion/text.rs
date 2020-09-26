use crate::elements::body::prelude::*;
use crate::elements::body::raw::RawElement;
use crate::elements::error::Error;
use crate::elements::prelude::*;
use crate::util::attributes::*;
use crate::util::{Context, Header, Tag};
use roxmltree::Node;
use std::collections::HashMap;

lazy_static! {
    static ref DEFAULT_ATTRIBUTES: Attributes = Attributes::new()
        .add("font-size", "13px")
        .add("padding", "16px");
}

#[derive(Clone, Debug)]
pub struct MJAccordionText {
    attributes: Attributes,
    context: Option<Context>,
    children: Vec<RawElement>,
}

impl MJAccordionText {
    fn default_attributes<'a, 'b>(node: &Node<'a, 'b>, header: &Header) -> Attributes {
        header
            .default_attributes()
            .get_attributes(node, DEFAULT_ATTRIBUTES.clone())
    }

    pub fn parse<'a, 'b>(
        node: &Node<'a, 'b>,
        header: &Header,
        attrs: &Attributes,
    ) -> Result<MJAccordionText, Error> {
        if node.tag_name().name() != "mj-accordion-text" {
            return Err(Error::ParseError(format!(
                "element should be 'mj-accordion-text' not '{}'",
                node.tag_name().name()
            )));
        }
        let attributes = Self::default_attributes(node, header)
            .concat(attrs)
            .concat(node);
        let mut element = MJAccordionText::new(attributes);
        for child in node.children() {
            element
                .children
                .push(RawElement::conditional_parse(&child, header, true)?);
        }
        Ok(element)
    }

    pub fn new(attributes: Attributes) -> Self {
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
        Some(self.attributes.inner())
    }
}

impl BodyComponent for MJAccordionText {}
impl BodyContainedComponent for MJAccordionText {}
impl ComponentWithSizeAttribute for MJAccordionText {}
