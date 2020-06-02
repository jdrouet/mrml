use super::BodyElement;
use crate::mjml::body::prelude::*;
use crate::mjml::error::Error;
use crate::mjml::prelude::*;
use crate::util::condition::*;
use crate::util::prelude::PropertyMap;
use crate::util::{Attributes, Context, Header, Size, Style};
use crate::Options;
use crate::{close_tag, open_tag, to_attributes};
use log::debug;
use roxmltree::Node;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct MJGroup {
    attributes: HashMap<String, String>,
    context: Option<Context>,
    children: Vec<BodyElement>,
}

impl MJGroup {
    pub fn parse<'a, 'b>(
        node: Node<'a, 'b>,
        opts: &Options,
        _extra: Option<&Attributes>,
    ) -> Result<MJGroup, Error> {
        let mut children = vec![];
        let mut attrs = Attributes::new();
        attrs.set("mobile-width", "mobile-width");
        for child in node.children() {
            children.push(BodyElement::parse(child, opts, Some(&attrs))?);
        }
        Ok(MJGroup {
            attributes: get_node_attributes(&node),
            context: None,
            children,
        })
    }

    fn get_width(&self) -> Option<Size> {
        self.get_current_width().and_then(|width| {
            if width.is_percent() {
                self.get_container_width().and_then(|container| {
                    Some(Size::Pixel(container.value() * width.value() / 100.0))
                })
            } else {
                Some(width)
            }
        })
    }

    fn get_style_div(&self) -> Style {
        let mut res = Style::new();
        res.set("font-size", "0");
        res.set("line-height", "0");
        res.set("text-align", "left");
        res.set("display", "inline-block");
        res.set("width", "100%");
        res.maybe_set("background-color", self.get_attribute("background-color"));
        res.maybe_set("direction", self.get_attribute("direction"));
        res.maybe_set("vertical-align", self.get_attribute("vertical-align"));
        res
    }

    fn get_style_td_outlook(&self) -> Style {
        let mut res = Style::new();
        res.maybe_set("vertical-align", self.get_attribute("vertical-align"));
        res.maybe_set("width", self.get_width());
        res
    }

    fn get_parsed_width(&self) -> Size {
        let non_raw_siblings = self
            .context()
            .and_then(|ctx| Some(ctx.non_raw_siblings()))
            .or(Some(1))
            .unwrap();
        match self.get_size_attribute("width") {
            Some(size) => size,
            None => Size::Percent(100.0 / (non_raw_siblings as f32)),
        }
    }

    fn get_column_class(&self) -> (String, Size) {
        let parsed_width = self.get_parsed_width();
        let classname = match parsed_width {
            Size::Percent(value) => format!("mj-column-per-{}", value),
            _ => format!("mj-column-px-{}", parsed_width.value()),
        };
        (classname.replace(".", "-"), parsed_width)
    }

    fn render_child(&self, header: &Header, child: &BodyElement) -> Result<String, Error> {
        let mut res = vec![];
        res.push(START_CONDITIONAL_TAG.into());
        {
            let mut style = Style::new();
            style.maybe_set("align", child.get_attribute("align"));
            style.maybe_set("vertical-align", child.get_attribute("vertical-align"));
            style.maybe_set(
                "width",
                child.get_width().or_else(|| {
                    child
                        .get_attribute("width")
                        .and_then(|value| Size::from_str(value.as_str()).ok())
                }),
            );
            res.push(open_tag!(
                "td",
                to_attributes!(("style", style.to_string()))
            ));
        }
        res.push(END_CONDITIONAL_TAG.into());
        res.push(child.render(header)?);
        res.push(START_CONDITIONAL_TAG.into());
        res.push(close_tag!("td"));
        res.push(END_CONDITIONAL_TAG.into());
        Ok(res.join(""))
    }

    fn render_children(&self, header: &Header) -> Result<String, Error> {
        let mut res = vec![];
        for child in self.children.iter() {
            if child.is_raw() {
                res.push(child.render(header)?);
            } else {
                res.push(self.render_child(header, child)?);
            }
        }
        Ok(res.join(""))
    }
}

impl Component for MJGroup {
    fn context(&self) -> Option<&Context> {
        self.context.as_ref()
    }

    fn update_header(&self, header: &mut Header) {
        let (classname, size) = self.get_column_class();
        header.add_media_query(classname, size);
        for child in self.children.iter() {
            child.update_header(header);
        }
    }

    fn set_context(&mut self, ctx: Context) {
        self.context = Some(ctx.clone());
        let children_ctx = Context::from(
            &ctx,
            self.get_current_width(),
            self.get_siblings(),
            self.get_raw_siblings(),
            0,
        );
        for (idx, item) in self.children.iter_mut().enumerate() {
            let mut child_ctx = children_ctx.clone();
            child_ctx.set_index(idx);
            item.set_context(child_ctx.clone());
        }
    }

    fn render(&self, header: &Header) -> Result<String, Error> {
        let mut res: Vec<String> = vec![];
        {
            let mut classnames: Vec<String> = vec![];
            classnames.push(self.get_column_class().0);
            classnames.push("mj-outlook-group-fix".into());
            res.push(open_tag!(
                "div",
                to_attributes!(
                    ("class", classnames.join(" ")),
                    ("style", self.get_style_div().to_string())
                )
            ));
        }
        res.push(START_CONDITIONAL_TAG.into());
        {
            let mut attrs = Attributes::new();
            attrs.maybe_set(
                "bgcolor",
                self.get_attribute("background-color").and_then(|value| {
                    if value == "none" {
                        None
                    } else {
                        Some(value)
                    }
                }),
            );
            attrs.set("border", "0");
            attrs.set("cellpadding", "0");
            attrs.set("cellspacing", "0");
            attrs.set("role", "presentation");
            res.push(open_tag!("table", attrs.to_string()));
        }
        res.push(open_tag!("tr"));
        res.push(END_CONDITIONAL_TAG.into());
        res.push(self.render_children(header)?);
        res.push(START_CONDITIONAL_TAG.into());
        res.push(close_tag!("tr"));
        res.push(close_tag!("table"));
        res.push(END_CONDITIONAL_TAG.into());
        res.push(close_tag!("div"));
        Ok(res.join(""))
    }
}

impl ComponentWithAttributes for MJGroup {
    fn default_attribute(&self, key: &str) -> Option<String> {
        debug!("default_attribute {}", key);
        match key {
            "direction" => Some("ltr".into()),
            _ => None,
        }
    }

    fn source_attributes(&self) -> Option<&HashMap<String, String>> {
        Some(&self.attributes)
    }
}

impl BodyComponent for MJGroup {
    fn get_style(&self, key: &str) -> Style {
        match key {
            "div" => self.get_style_div(),
            "td-outlook" => self.get_style_td_outlook(),
            _ => Style::new(),
        }
    }
}

impl ComponentWithChildren for MJGroup {
    fn get_children(&self) -> &Vec<BodyElement> {
        &self.children
    }

    fn get_current_width(&self) -> Option<Size> {
        let ctx = match self.context() {
            Some(value) => value,
            None => return None,
        };
        let parent_width = ctx.container_width().unwrap();
        let non_raw_siblings = ctx.non_raw_siblings();
        let borders = self.get_border_horizontal_width();
        let paddings = self.get_padding_horizontal_width();
        let inner_border_left = match self.get_prefixed_border_left("inner") {
            Some(size) => size.value(),
            None => 0.0,
        };
        let inner_border_right = match self.get_prefixed_border_right("inner") {
            Some(size) => size.value(),
            None => 0.0,
        };
        let inner_borders = inner_border_left + inner_border_right;
        let all_paddings = paddings.value() + borders.value() + inner_borders;

        let container_width = match self.get_size_attribute("width") {
            Some(value) => value,
            None => Size::Pixel(parent_width.value() / (non_raw_siblings as f32)),
        };
        if container_width.is_percent() {
            Some(Size::Pixel(
                (parent_width.value() * container_width.value() / 100.0) - all_paddings,
            ))
        } else {
            Some(Size::Pixel(container_width.value() - all_paddings))
        }
    }
}

impl ComponentWithSizeAttribute for MJGroup {}
impl BodyContainedComponent for MJGroup {}
impl BodyComponentWithBorder for MJGroup {}
impl BodyComponentWithBoxWidths for MJGroup {}
impl BodyComponentWithPadding for MJGroup {}

#[cfg(test)]
pub mod tests {
    use crate::tests::compare_render;

    #[test]
    fn base() {
        compare_render(
            include_str!("../../../test/mj-group.mjml"),
            include_str!("../../../test/mj-group.html"),
        );
    }
}
