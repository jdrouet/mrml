use super::MJAccordionElement;
use crate::elements::body::prelude::*;
use crate::elements::body::BodyElement;
use crate::elements::error::Error;
use crate::elements::prelude::*;
use crate::parser::{Element, Node};
use crate::util::attributes::*;
use crate::util::context::Context;
use crate::util::header::Header;
use crate::util::size::Size;
use crate::util::tag::Tag;

const CHILDREN_ATTRIBUTES: [&'static str; 9] = [
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

lazy_static! {
    static ref DEFAULT_ATTRIBUTES: Attributes = Attributes::new()
        .add("border", "2px solid black")
        .add("font-family", "Ubuntu, Helvetica, Arial, sans-serif")
        .add("icon-align", "middle")
        .add("icon-position", "right")
        .add("icon-height", "32px")
        .add("icon-width", "32px")
        .add("icon-wrapped-url", "https://i.imgur.com/bIXv1bk.png")
        .add("icon-wrapped-alt", "+")
        .add("icon-unwrapped-url", "https://i.imgur.com/w4uTygT.png")
        .add("icon-unwrapped-alt", "-")
        .add("padding", "10px 25px");
}

#[derive(Clone, Debug)]
pub struct MJAccordion {
    attributes: Attributes,
    context: Option<Context>,
    children: Vec<BodyElement>,
}

impl MJAccordion {
    fn default_attributes<'a>(node: &Node<'a>, header: &Header) -> Attributes {
        header
            .default_attributes()
            .get_attributes(node, DEFAULT_ATTRIBUTES.clone())
    }

    pub fn parse<'a>(node: &Node<'a>, header: &Header) -> Result<MJAccordion, Error> {
        let mut result = MJAccordion {
            attributes: Self::default_attributes(node, header).concat(node),
            context: None,
            children: vec![],
        };
        let child_attrs = result.get_children_attributes();
        for child in node.children.iter() {
            match child {
                Element::Node(node) => match node.name.as_str() {
                    "mj-accordion-element" => {
                        let element = MJAccordionElement::parse(node, header, &child_attrs)?;
                        result
                            .children
                            .push(BodyElement::MJAccordionElement(element));
                    }
                    _ => return Err(Error::ParseError("unexpected child".into())),
                },
                _ => return Err(Error::ParseError("unexpected child".into())),
            };
        }
        Ok(result)
    }

    fn get_children_attributes(&self) -> Attributes {
        let mut res = Attributes::new();
        for key in CHILDREN_ATTRIBUTES.iter() {
            if let Some(value) = self.get_attribute(key) {
                res.set(key, value);
            }
        }
        res
    }
}

impl Component for MJAccordion {
    fn update_header(&self, header: &mut Header) {
        header.add_style(r#"
        noinput.mj-accordion-checkbox { display: block! important; }

        @media yahoo, only screen and (min-width:0) {
          .mj-accordion-element { display:block; }
          input.mj-accordion-checkbox, .mj-accordion-less { display: none !important; }
          input.mj-accordion-checkbox + * .mj-accordion-title { cursor: pointer; touch-action: manipulation; -webkit-user-select: none; -moz-user-select: none; user-select: none; }
          input.mj-accordion-checkbox + * .mj-accordion-content { overflow: hidden; display: none; }
          input.mj-accordion-checkbox + * .mj-accordion-more { display: block !important; }
          input.mj-accordion-checkbox:checked + * .mj-accordion-content { display: block; }
          input.mj-accordion-checkbox:checked + * .mj-accordion-more { display: none !important; }
          input.mj-accordion-checkbox:checked + * .mj-accordion-less { display: block !important; }
        }

        .moz-text-html input.mj-accordion-checkbox + * .mj-accordion-title { cursor: auto; touch-action: auto; -webkit-user-select: auto; -moz-user-select: auto; user-select: auto; }
        .moz-text-html input.mj-accordion-checkbox + * .mj-accordion-content { overflow: hidden; display: block; }
        .moz-text-html input.mj-accordion-checkbox + * .mj-accordion-ico { display: none; }

        @goodbye { @gmail }
        "#);
        header.maybe_add_font_families(self.get_attribute("font-family"));
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
        let mut children = vec![];
        for child in self.children.iter() {
            children.push(child.render(header)?);
        }
        let tbody = Tag::tbody().render(children.join(""));
        let table = Tag::table()
            .set_style("width", "100%")
            .set_style("border-collapse", "collapse")
            .maybe_set_style("border", self.get_attribute("border"))
            .set_style("border-bottom", "none")
            .maybe_set_style("font-family", self.get_attribute("font-family"))
            .set_attribute("cellspacing", 0)
            .set_attribute("cellpadding", 0)
            .set_class("mj-accordion")
            .render(tbody);
        Ok(table)
    }
}

impl BodyComponent for MJAccordion {
    fn attributes(&self) -> Option<&Attributes> {
        Some(&self.attributes)
    }

    fn get_children(&self) -> &Vec<BodyElement> {
        &self.children
    }

    fn get_current_width(&self) -> Option<Size> {
        self.context().and_then(|ctx| ctx.container_width())
    }

    fn set_style(&self, _name: &str, tag: Tag) -> Tag {
        tag
    }
}

#[cfg(test)]
pub mod tests {
    use crate::tests::compare_render;

    #[test]
    fn base() {
        compare_render(
            include_str!("../../../../test/mj-accordion.mjml"),
            include_str!("../../../../test/mj-accordion.html"),
        );
    }

    #[test]
    fn with_others() {
        compare_render(
            include_str!("../../../../test/mj-accordion-other.mjml"),
            include_str!("../../../../test/mj-accordion-other.html"),
        );
    }

    #[test]
    fn with_icon() {
        compare_render(
            include_str!("../../../../test/mj-accordion-icon.mjml"),
            include_str!("../../../../test/mj-accordion-icon.html"),
        );
    }

    #[test]
    fn with_font_padding() {
        compare_render(
            include_str!("../../../../test/mj-accordion-font-padding.mjml"),
            include_str!("../../../../test/mj-accordion-font-padding.html"),
        );
    }
}
