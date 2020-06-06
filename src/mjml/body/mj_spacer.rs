use crate::mjml::body::prelude::*;
use crate::mjml::error::Error;
use crate::mjml::prelude::*;
use crate::util::condition::{END_CONDITIONAL_TAG, START_CONDITIONAL_TAG};
use crate::util::{Context, Header, Tag};
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

    fn set_style_div(&self, tag: Tag) -> Tag {
        tag.maybe_set_style("height", self.get_attribute("height"))
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
        let div = self.set_style_div(Tag::div());
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
    fn set_style(&self, name: &str, tag: Tag) -> Tag {
        match name {
            "div" => self.set_style_div(tag),
            _ => tag,
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
