use super::MJAccordionElement;
use crate::mjml::body::prelude::*;
use crate::mjml::body::BodyElement;
use crate::mjml::error::Error;
use crate::mjml::prelude::*;
// use crate::util::condition::*;
use crate::util::{Context, Header, Size, Tag};
use crate::Options;
use log::debug;
use roxmltree::Node;
use std::collections::HashMap;

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

#[derive(Clone, Debug)]
pub struct MJAccordion {
    attributes: HashMap<String, String>,
    context: Option<Context>,
    children: Vec<BodyElement>,
}

impl MJAccordion {
    pub fn parse<'a, 'b>(node: &Node<'a, 'b>, opts: &Options) -> Result<MJAccordion, Error> {
        let mut result = MJAccordion {
            attributes: get_node_attributes(&node),
            context: None,
            children: vec![],
        };
        let child_attrs = result.get_children_attributes();
        for child in node.children() {
            let tag_name = child.tag_name().name();
            if tag_name == "" {
                if let Some(content) = child.text() {
                    if content.len() == 0 {
                        continue;
                    }
                }
            } else if tag_name != "mj-accordion-element" {
                return Err(Error::ParseError(format!(
                    "expect only 'mj-accordion-element', not '{}'",
                    tag_name
                )));
            } else {
                let element = MJAccordionElement::parse(&child, opts, &child_attrs)?;
                result
                    .children
                    .push(BodyElement::MJAccordionElement(element));
            }
        }
        Ok(result)
    }

    fn get_children_attributes(&self) -> HashMap<String, String> {
        let mut res = HashMap::new();
        for key in CHILDREN_ATTRIBUTES.iter() {
            if let Some(value) = self.get_attribute(key) {
                res.insert(key.to_string(), value.to_string());
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

impl ComponentWithAttributes for MJAccordion {
    fn default_attribute(&self, key: &str) -> Option<String> {
        debug!("default_attribute {}", key);
        match key {
            "border" => Some("2px solid black".into()),
            "font-family" => Some("Ubuntu, Helvetica, Arial, sans-serif".into()),
            "icon-align" => Some("middle".into()),
            "icon-position" => Some("right".into()),
            "icon-height" => Some("32px".into()),
            "icon-width" => Some("32px".into()),
            "icon-wrapped-url" => Some("https://i.imgur.com/bIXv1bk.png".into()),
            "icon-wrapped-alt" => Some("+".into()),
            "icon-unwrapped-url" => Some("https://i.imgur.com/w4uTygT.png".into()),
            "icon-unwrapped-alt" => Some("-".into()),
            "padding" => Some("10px 25px".into()),
            _ => None,
        }
    }

    fn source_attributes(&self) -> Option<&HashMap<String, String>> {
        Some(&self.attributes)
    }
}

impl BodyComponent for MJAccordion {
    fn set_style(&self, _name: &str, tag: Tag) -> Tag {
        tag
    }
}

impl ComponentWithChildren for MJAccordion {
    fn get_children(&self) -> &Vec<BodyElement> {
        &self.children
    }

    fn get_current_width(&self) -> Option<Size> {
        self.context().and_then(|ctx| ctx.container_width())
    }
}

impl BodyContainedComponent for MJAccordion {}
impl ComponentWithSizeAttribute for MJAccordion {}

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
