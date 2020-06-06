use crate::mjml::body::prelude::*;
use crate::mjml::error::Error;
use crate::mjml::prelude::*;
use crate::util::condition::{END_CONDITIONAL_TAG, START_CONDITIONAL_TAG};
use crate::util::prelude::*;
use crate::util::{Context, Header, Style, Tag};
use crate::Options;
use log::debug;
use roxmltree::Node;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct MJSpacer {
    options: Options,
    attributes: HashMap<String, String>,
    context: Option<Context>,
}

impl MJSpacer {
    pub fn parse<'a, 'b>(node: Node<'a, 'b>, opts: &Options) -> Result<MJSpacer, Error> {
        Ok(MJSpacer {
            options: opts.clone(),
            attributes: get_node_attributes(&node),
            context: None,
        })
    }

    fn get_style_div(&self) -> Style {
        let mut res = Style::new();
        res.maybe_set("height", self.get_attribute("height"));
        res
    }
}

impl Component for MJSpacer {
    fn context(&self) -> Option<&Context> {
        self.context.as_ref()
    }

    fn set_context(&mut self, ctx: Context) {
        self.context = Some(ctx.clone());
    }

    fn render(&self, _header: &Header) -> Result<String, Error> {
        let height = self.get_size_attribute("height");
        let table = Tag::table();
        let tr = Tag::tr();
        let td = Tag::td()
            .set_style("vertical-align", "top")
            .maybe_set_style("height", height.clone())
            .maybe_set_attribute("height", height.and_then(|h| Some(h.value())));
        let div = Tag::div().insert_style(self.get_style_div().inner());
        let mut res = vec![];
        res.push(START_CONDITIONAL_TAG.into());
        res.push(table.open());
        res.push(tr.open());
        res.push(td.open());
        res.push(END_CONDITIONAL_TAG.into());
        res.push(div.render("&nbsp;"));
        res.push(START_CONDITIONAL_TAG.into());
        res.push(td.close());
        res.push(tr.close());
        res.push(table.close());
        res.push(END_CONDITIONAL_TAG.into());
        Ok(res.join(""))
    }
}

impl ComponentWithAttributes for MJSpacer {
    fn default_attribute(&self, key: &str) -> Option<String> {
        debug!("default_attribute {}", key);
        match key {
            "height" => Some("20px".into()),
            _ => None,
        }
    }

    fn source_attributes(&self) -> Option<&HashMap<String, String>> {
        Some(&self.attributes)
    }
}

impl BodyComponent for MJSpacer {
    fn get_style(&self, name: &str) -> Style {
        match name {
            "div" => self.get_style_div(),
            _ => Style::new(),
        }
    }
}

impl BodyContainedComponent for MJSpacer {}
impl ComponentWithSizeAttribute for MJSpacer {}
impl BodyComponentWithBorder for MJSpacer {}
impl BodyComponentWithPadding for MJSpacer {}
impl BodyComponentWithBoxWidths for MJSpacer {}

#[cfg(test)]
pub mod tests {
    use crate::tests::compare_render;

    #[test]
    fn base() {
        compare_render(
            include_str!("../../../test/mj-spacer.mjml"),
            include_str!("../../../test/mj-spacer.html"),
        );
    }
}
