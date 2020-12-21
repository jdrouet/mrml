mod parser;

use crate::elements::body::mj_accordion_text::MJAccordionText;
use crate::elements::body::mj_accordion_title::MJAccordionTitle;
use crate::elements::body::prelude::*;
use crate::elements::body::BodyElement;
use crate::elements::error::Error;
use crate::elements::prelude::*;
use crate::util::attributes::*;
use crate::util::condition::*;
use crate::util::context::Context;
use crate::util::header::Header;
use crate::util::size::Size;
use crate::util::tag::Tag;

const CHILDREN_ATTR: [&str; 9] = [
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
    attributes: Attributes,
    context: Option<Context>,
    title: Option<MJAccordionTitle>,
    text: Option<MJAccordionText>,
}

impl MJAccordionElement {
    fn get_children_attributes(&self) -> Attributes {
        let mut result = Attributes::default();
        for key in CHILDREN_ATTR.iter() {
            if let Some(value) = self.get_attribute(key) {
                result.set(key, value);
            }
        }
        result
    }

    fn render_text(&self, header: &Header, attributes: &Attributes) -> Result<String, Error> {
        match self.text.as_ref() {
            Some(content) => content.render(header),
            None => MJAccordionText::new(attributes.clone()).render(header),
        }
    }

    fn render_title(&self, header: &Header, attributes: &Attributes) -> Result<String, Error> {
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
        self.context = Some(ctx);
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

impl BodyComponent for MJAccordionElement {
    fn attributes(&self) -> Option<&Attributes> {
        Some(&self.attributes)
    }
    fn get_children(&self) -> &Vec<BodyElement> {
        &EMPTY_CHILDREN
    }
    fn get_current_width(&self) -> Option<Size> {
        None
    }
}
