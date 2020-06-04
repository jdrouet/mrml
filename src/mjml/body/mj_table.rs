use crate::mjml::body::prelude::*;
use crate::mjml::body::raw::RawElement;
use crate::mjml::error::Error;
use crate::mjml::prelude::*;
use crate::util::prelude::PropertyMap;
use crate::util::{Attributes, Context, Header, Style};
use crate::Options;
use crate::{close_tag, open_tag};
use log::debug;
use roxmltree::Node;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct MJTable {
    attributes: HashMap<String, String>,
    context: Option<Context>,
    children: Vec<RawElement>,
}

impl MJTable {
    pub fn parse<'a, 'b>(node: Node<'a, 'b>, opts: &Options) -> Result<MJTable, Error> {
        let mut children = vec![];
        for child in node.children() {
            children.push(RawElement::conditional_parse(child, opts, true)?);
        }
        Ok(MJTable {
            attributes: get_node_attributes(&node),
            context: None,
            children,
        })
    }

    fn get_style_table(&self) -> Style {
        let mut res = Style::new();
        res.maybe_set("border", self.get_attribute("border"));
        res.maybe_set("color", self.get_attribute("color"));
        res.maybe_set("font-family", self.get_attribute("font-family"));
        res.maybe_set("font-size", self.get_attribute("font-size"));
        res.maybe_set("line-height", self.get_attribute("line-height"));
        res.maybe_set("table-layout", self.get_attribute("table-layout"));
        res.maybe_set("width", self.get_attribute("width"));
        res
    }
}

impl Component for MJTable {
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
        let mut res = vec![];
        let mut attrs = Attributes::new();
        attrs.set("border", "0");
        attrs.maybe_set("cellpadding", self.get_size_attribute("cellpadding"));
        attrs.maybe_set("cellspacing", self.get_size_attribute("cellspacing"));
        attrs.set("style", self.get_style_table());
        attrs.maybe_set("width", self.get_size_attribute("width"));
        res.push(open_tag!("table", attrs.to_string()));
        for child in self.children.iter() {
            res.push(child.render(header)?);
        }
        res.push(close_tag!("table"));
        Ok(res.join(""))
    }
}

impl BodyComponent for MJTable {}
impl BodyContainedComponent for MJTable {}
impl ComponentWithAttributes for MJTable {
    fn default_attribute(&self, key: &str) -> Option<String> {
        debug!("default_attribute {}", key);
        match key {
            "align" => Some("left".into()),
            "border" => Some("none".into()),
            "cellpadding" => Some("0".into()),
            "cellspacing" => Some("0".into()),
            "color" => Some("#000000".into()),
            "font-family" => Some("Ubuntu, Helvetica, Arial, sans-serif".into()),
            "font-size" => Some("13px".into()),
            "line-height" => Some("22px".into()),
            "padding" => Some("10px 25px".into()),
            "table-layout" => Some("auto".into()),
            "width" => Some("100%".into()),
            _ => None,
        }
    }

    fn source_attributes(&self) -> Option<&HashMap<String, String>> {
        Some(&self.attributes)
    }
}
impl ComponentWithSizeAttribute for MJTable {}

#[cfg(test)]
pub mod tests {
    use crate::tests::compare_render;

    #[test]
    fn base() {
        compare_render(
            include_str!("../../../test/mj-table.mjml"),
            include_str!("../../../test/mj-table.html"),
        );
    }

    #[test]
    fn with_text_attributes() {
        compare_render(
            include_str!("../../../test/mj-table-text.mjml"),
            include_str!("../../../test/mj-table-text.html"),
        );
    }

    #[test]
    fn with_table_attributes() {
        compare_render(
            include_str!("../../../test/mj-table-table.mjml"),
            include_str!("../../../test/mj-table-table.html"),
        );
    }

    #[test]
    fn with_other_attributes() {
        compare_render(
            include_str!("../../../test/mj-table-other.mjml"),
            include_str!("../../../test/mj-table-other.html"),
        );
    }
}
