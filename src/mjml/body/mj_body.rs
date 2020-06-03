use super::prelude::*;
use super::BodyElement;
use crate::mjml::prelude::*;
use crate::mjml::{Component, Error};
use crate::util::prelude::PropertyMap;
use crate::util::{Attributes, Context, Header, Style};
use crate::Options;
use crate::{close_tag, open_tag, to_attributes};
use log::debug;
use roxmltree::Node;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct MJBody {
    attributes: HashMap<String, String>,
    context: Option<Context>,
    children: Vec<BodyElement>,
    exists: bool,
}

impl MJBody {
    pub fn empty<'a, 'b>(_opts: &Options) -> MJBody {
        MJBody {
            attributes: HashMap::new(),
            children: vec![],
            context: None,
            exists: false,
        }
    }

    pub fn parse<'a, 'b>(node: Node<'a, 'b>, opts: &Options) -> Result<MJBody, Error> {
        let mut children = vec![];
        for child in node.children() {
            children.push(BodyElement::parse(child, opts, None)?);
        }
        Ok(MJBody {
            attributes: get_node_attributes(&node),
            children,
            context: None,
            exists: true,
        })
    }

    pub fn render_preview(&self, header: &Header) -> String {
        let preview = match header.preview() {
            Some(value) => value,
            None => return "".into(),
        };
        let mut style = Style::new();
        style.set("display", "none");
        style.set("font-size", "1px");
        style.set("color", "#ffffff");
        style.set("line-height", "1px");
        style.set("max-height", "0px");
        style.set("max-width", "0px");
        style.set("opacity", "0");
        style.set("overflow", "hidden");
        let mut res = vec![];
        res.push(open_tag!("div", to_attributes!(("style", style.to_string()))));
        res.push(preview.clone());
        res.push(close_tag!("div"));
        res.join("")
    }
}

impl Component for MJBody {
    fn context(&self) -> Option<&Context> {
        self.context.as_ref()
    }

    fn update_header(&self, header: &mut Header) {
        for child in self.children.iter() {
            child.update_header(header);
        }
    }

    fn set_context(&mut self, ctx: Context) {
        self.context = Some(ctx.clone());
        //
        let sibling = self.children.len();
        let raw_sibling = self
            .children
            .iter()
            .filter(|item| match item {
                BodyElement::Raw(_) => true,
                _ => false,
            })
            .collect::<Vec<&BodyElement>>()
            .len();
        //
        let container_width = self.get_size_attribute("width");
        //
        for (idx, child) in self.children.iter_mut().enumerate() {
            child.set_context(Context::from(
                &ctx,
                container_width.clone(),
                sibling,
                raw_sibling,
                idx,
            ));
        }
    }

    fn render(&self, header: &Header) -> Result<String, Error> {
        debug!("render");
        let mut res: Vec<String> = vec![];
        {
            let mut attrs = Attributes::new();
            let style = self.get_style("body");
            if !style.is_empty() {
                attrs.set("style", style.to_string());
            }
            res.push(open_tag!("body", attrs.to_string()));
        }
        res.push(self.render_preview(&header));
        if self.exists {
            let mut attrs = Attributes::new();
            attrs.maybe_set("class", self.get_attribute("css-class"));
            attrs.set("style", self.get_style("div").to_string());
            res.push(open_tag!("div", attrs.to_string()));
            for child in self.children.iter() {
                res.push(child.render(header)?);
            }
            res.push(close_tag!("div"));
        }
        res.push(close_tag!("body"));
        Ok(res.join(""))
    }
}

impl ComponentWithAttributes for MJBody {
    fn source_attributes(&self) -> Option<&HashMap<String, String>> {
        Some(&self.attributes)
    }

    fn default_attribute(&self, key: &str) -> Option<String> {
        debug!("default_attribute {}", key);
        match key {
            "width" => Some("600px".into()),
            _ => None,
        }
    }
}

impl BodyComponent for MJBody {
    fn get_style(&self, key: &str) -> Style {
        let mut res = Style::new();
        match key {
            "body" | "div" => {
                res.maybe_set("background-color", self.get_attribute("background-color"));
            }
            _ => (),
        };
        res
    }
}

impl ComponentWithSizeAttribute for MJBody {}

#[cfg(test)]
pub mod tests {
    use crate::tests::{compare_render, compare_render_with_options};
    use crate::Options;

    #[test]
    fn basic() {
        compare_render(
            include_str!("../../../test/mj-body.mjml"),
            include_str!("../../../test/mj-body.html"),
        );
    }

    #[test]
    fn with_options() {
        let mut opts = Options::default();
        opts.keep_comments = false;
        compare_render_with_options(
            include_str!("../../../test/mj-body.mjml"),
            include_str!("../../../test/mj-body-without-comments.html"),
            opts,
        );
    }

    #[test]
    fn with_background_color() {
        compare_render(
            include_str!("../../../test/mj-body-background-color.mjml"),
            include_str!("../../../test/mj-body-background-color.html"),
        );
    }

    #[test]
    fn with_class() {
        compare_render(
            include_str!("../../../test/mj-body-class.mjml"),
            include_str!("../../../test/mj-body-class.html"),
        );
    }

    #[test]
    fn with_width() {
        compare_render(
            include_str!("../../../test/mj-body-width.mjml"),
            include_str!("../../../test/mj-body-width.html"),
        );
    }
}
