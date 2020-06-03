use crate::mjml::body::prelude::*;
use crate::mjml::error::Error;
use crate::mjml::prelude::*;
use crate::util::condition::{END_CONDITIONAL_TAG, START_CONDITIONAL_TAG};
use crate::util::prelude::PropertyMap;
use crate::util::{Attributes, Context, Header, Style};
use crate::Options;
use crate::{close_tag, open_tag, to_attributes};
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
        let mut res = vec![];
        res.push(START_CONDITIONAL_TAG.into());
        res.push(open_tag!(
            "table",
            to_attributes!(
                ("border", "0"),
                ("cellpadding", "0"),
                ("cellspacing", "0"),
                ("role", "presentation")
            )
        ));
        res.push(open_tag!("tr"));
        {
            let height = self.get_size_attribute("height");
            let mut style = Style::new();
            style.set("vertical-align", "top");
            style.maybe_set(
                "height",
                height.as_ref().and_then(|size| Some(size.to_string())),
            );
            let mut attrs = Attributes::new();
            attrs.maybe_set(
                "height",
                height.as_ref().and_then(|size| Some(size.value())),
            );
            attrs.set("style", style);
            res.push(open_tag!("td", attrs.to_string()));
        }
        res.push(END_CONDITIONAL_TAG.into());
        res.push(open_tag!(
            "div",
            to_attributes!(("style", self.get_style_div().to_string()))
        ));
        res.push("&nbsp;".into());
        res.push(close_tag!("div"));
        res.push(START_CONDITIONAL_TAG.into());
        res.push(close_tag!("td"));
        res.push(close_tag!("tr"));
        res.push(close_tag!("table"));
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
