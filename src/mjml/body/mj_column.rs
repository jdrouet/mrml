use super::BodyElement;
use crate::mjml::body::prelude::*;
use crate::mjml::error::Error;
use crate::mjml::prelude::*;
use crate::util::prelude::PropertyMap;
use crate::util::{Attributes, Context, Header, Size, Style};
use crate::Options;
use crate::{close_tag, open_tag, to_attributes};
use log::debug;
use roxmltree::Node;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct MJColumn {
    attributes: HashMap<String, String>,
    context: Option<Context>,
    children: Vec<BodyElement>,
}

impl MJColumn {
    pub fn parse<'a, 'b>(
        node: Node<'a, 'b>,
        opts: &Options,
        extra: Option<&Attributes>,
    ) -> Result<MJColumn, Error> {
        let mut children = vec![];
        for child in node.children() {
            children.push(BodyElement::parse(child, opts, None)?);
        }
        let mut attributes = match extra {
            Some(attrs) => attrs.inner().clone(),
            None => HashMap::new(),
        };
        add_node_attributes(&mut attributes, &node);
        Ok(MJColumn {
            attributes,
            context: None,
            children,
        })
    }

    fn get_column_class(&self) -> Option<(String, Size)> {
        let parsed_width = self.get_parsed_width();
        let classname = match parsed_width {
            Size::Percent(value) => format!("mj-column-per-{}", value),
            _ => format!("mj-column-px-{}", parsed_width.value()),
        };
        Some((classname.replace(".", "-"), parsed_width))
    }

    fn get_mobile_width(&self) -> Option<Size> {
        if self.get_attribute("mobile-width").is_none() {
            return Some(Size::Percent(100.0));
        }
        let width = self.get_size_attribute("width");
        if width.is_none() {
            return self
                .context()
                .and_then(|ctx| Some(ctx.non_raw_siblings()))
                .and_then(|count| Some(Size::Percent(100.0 / (count as f32))));
        }
        return width.and_then(|width| match width {
            Size::Percent(_) => Some(width),
            _ => match self.get_container_width() {
                Some(container) => Some(Size::Percent(width.value() / container.value())),
                None => None,
            },
        });
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

    fn get_width_as_pixel(&self) -> String {
        let container_width = self.get_container_width();
        if container_width.is_none() {
            return "100%".into();
        }
        let container_width = container_width.unwrap();
        let parsed_width = self.get_parsed_width();
        let result = match parsed_width {
            Size::Percent(value) => Size::Pixel(container_width.value() * value / 100.0),
            _ => parsed_width,
        };
        result.to_string()
    }

    fn has_gutter(&self) -> bool {
        self.get_attribute("padding").is_some()
            || self.get_attribute("padding-bottom").is_some()
            || self.get_attribute("padding-left").is_some()
            || self.get_attribute("padding-right").is_some()
            || self.get_attribute("padding-top").is_some()
    }

    fn get_style_div(&self) -> Style {
        let mut res = Style::new();
        res.set("font-size", "0px");
        res.set("text-align", "left");
        res.maybe_set("direction", self.get_attribute("direction"));
        res.set("display", "inline-block");
        res.maybe_set("vertical-align", self.get_attribute("vertical-align"));
        res.maybe_set("width", self.get_mobile_width());
        res
    }

    fn get_style_table_gutter(&self) -> Style {
        let mut res = Style::new();
        res.maybe_set("background-color", self.get_attribute("background-color"));
        res.maybe_set("border", self.get_attribute("border"));
        res.maybe_set("border-bottom", self.get_attribute("border-bottom"));
        res.maybe_set("border-left", self.get_attribute("border-left"));
        res.maybe_set("border-radius", self.get_attribute("border-radius"));
        res.maybe_set("border-right", self.get_attribute("border-right"));
        res.maybe_set("border-top", self.get_attribute("border-top"));
        res
    }

    fn get_style_table_simple(&self) -> Style {
        let mut res = self.get_style_table_gutter();
        res.maybe_set("vertical-align", self.get_attribute("vertical-align"));
        res
    }

    fn get_style_table(&self) -> Style {
        if self.has_gutter() {
            self.get_style_table_gutter()
        } else {
            self.get_style_table_simple()
        }
    }

    fn get_style_td_outlook(&self) -> Style {
        let mut res = Style::new();
        res.maybe_set("vertical-align", self.get_attribute("vertical-align"));
        res.set("width", self.get_width_as_pixel());
        res
    }

    fn get_style_gutter(&self) -> Style {
        let mut res = self.get_style_table_simple();
        res.maybe_set("padding", self.get_attribute("padding"));
        res.maybe_set("padding-top", self.get_attribute("padding-top"));
        res.maybe_set("padding-right", self.get_attribute("padding-right"));
        res.maybe_set("padding-bottom", self.get_attribute("padding-bottom"));
        res.maybe_set("padding-left", self.get_attribute("padding-left"));
        res
    }

    fn render_gutter(&self, header: &Header) -> Result<String, Error> {
        let mut res = vec![];
        res.push(open_tag!(
            "table",
            to_attributes!(
                ("border", "0"),
                ("cellpadding", "0"),
                ("cellspacing", "0"),
                ("role", "presentation"),
                ("width", "100%")
            )
        ));
        res.push(open_tag!("tbody"));
        res.push(open_tag!("tr"));
        res.push(open_tag!(
            "td",
            to_attributes!(("style", self.get_style_gutter().to_string()))
        ));
        res.push(self.render_column(header)?);
        res.push(close_tag!("td"));
        res.push(close_tag!("tr"));
        res.push(close_tag!("tbody"));
        res.push(close_tag!("table"));
        Ok(res.join(""))
    }

    fn render_mj_child(&self, header: &Header, child: &BodyElement) -> Result<String, Error> {
        let mut res = vec![];
        res.push(open_tag!("tr"));
        let mut style = Style::new();
        style.maybe_set(
            "background",
            child.get_attribute("container-background-color"),
        );
        style.set("font-size", "0px");
        style.maybe_set("padding", child.get_attribute("padding"));
        style.maybe_set("padding-top", child.get_attribute("padding-top"));
        style.maybe_set("padding-right", child.get_attribute("padding-right"));
        style.maybe_set("padding-bottom", child.get_attribute("padding-bottom"));
        style.maybe_set("padding-left", child.get_attribute("padding-left"));
        style.set("word-break", "break-word");
        let mut attrs = Attributes::new();
        attrs.maybe_set("align", child.get_attribute("align"));
        attrs.maybe_set("vertical-align", child.get_attribute("vertical-align"));
        attrs.maybe_set("class", child.get_attribute("css-class"));
        attrs.set("style", style.to_string());
        res.push(open_tag!("td", attrs.to_string()));
        res.push(child.render(header)?);
        res.push(close_tag!("td"));
        res.push(close_tag!("tr"));
        Ok(res.join(""))
    }

    fn render_column(&self, header: &Header) -> Result<String, Error> {
        let mut res = vec![];
        res.push(open_tag!(
            "table",
            to_attributes!(
                ("border", "0"),
                ("cellpadding", "0"),
                ("cellspacing", "0"),
                ("role", "presentation"),
                ("style", self.get_style_table().to_string()),
                ("width", "100%")
            )
        ));
        for child in self.children.iter() {
            match child {
                BodyElement::Raw(_) => res.push(child.render(header)?),
                _ => res.push(self.render_mj_child(header, &child)?),
            };
        }
        res.push(close_tag!("table"));
        Ok(res.join(""))
    }
}

impl Component for MJColumn {
    fn context(&self) -> Option<&Context> {
        self.context.as_ref()
    }

    fn update_header(&self, header: &mut Header) {
        if let Some((classname, size)) = self.get_column_class() {
            header.add_media_query(classname, size);
        }
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
        let mut classes = vec![];
        if let Some((classname, _size)) = self.get_column_class() {
            classes.push(classname);
        }
        classes.push("mj-outlook-group-fix".into());
        if let Some(class) = self.get_attribute("css-class") {
            classes.push(class.clone());
        }
        let mut attrs = Attributes::new();
        attrs.set("class", classes.join(" "));
        attrs.set("style", self.get_style_div().to_string());
        let mut res = vec![];
        res.push(open_tag!("div", attrs.to_string()));
        if self.has_gutter() {
            res.push(self.render_gutter(header)?);
        } else {
            res.push(self.render_column(header)?);
        }
        res.push(close_tag!("div"));
        Ok(res.join(""))
    }
}

impl ComponentWithAttributes for MJColumn {
    fn default_attribute(&self, key: &str) -> Option<String> {
        debug!("default_attribute {}", key);
        match key {
            "direction" => Some("ltr".into()),
            "vertical-align" => Some("top".into()),
            _ => None,
        }
    }

    fn source_attributes(&self) -> Option<&HashMap<String, String>> {
        Some(&self.attributes)
    }
}

impl BodyComponent for MJColumn {
    fn get_style(&self, key: &str) -> Style {
        match key {
            "div" => self.get_style_div(),
            "table" => self.get_style_table(),
            "td-outlook" => self.get_style_td_outlook(),
            "gutter" => self.get_style_gutter(),
            _ => Style::new(),
        }
    }

    fn get_width(&self) -> Option<Size> {
        self.get_container_width().and_then(|container_width| {
            let parsed_width = self.get_parsed_width();
            let result = match parsed_width {
                Size::Percent(value) => Size::Pixel(container_width.value() * value / 100.0),
                _ => parsed_width,
            };
            Some(result)
        })
    }
}

impl ComponentWithChildren for MJColumn {
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

impl ComponentWithSizeAttribute for MJColumn {}
impl BodyContainedComponent for MJColumn {}
impl BodyComponentWithBorder for MJColumn {}
impl BodyComponentWithBoxWidths for MJColumn {}
impl BodyComponentWithPadding for MJColumn {}

#[cfg(test)]
pub mod tests {
    use crate::tests::compare_render;

    #[test]
    fn base() {
        compare_render(
            include_str!("../../../test/mj-column.mjml"),
            include_str!("../../../test/mj-column.html"),
        );
    }

    #[test]
    fn with_background_color() {
        compare_render(
            include_str!("../../../test/mj-column-background-color.mjml"),
            include_str!("../../../test/mj-column-background-color.html"),
        );
    }

    #[test]
    fn with_border() {
        compare_render(
            include_str!("../../../test/mj-column-border.mjml"),
            include_str!("../../../test/mj-column-border.html"),
        );
    }

    #[test]
    fn with_border_radius() {
        compare_render(
            include_str!("../../../test/mj-column-border-radius.mjml"),
            include_str!("../../../test/mj-column-border-radius.html"),
        );
    }

    #[test]
    fn with_class() {
        compare_render(
            include_str!("../../../test/mj-column-class.mjml"),
            include_str!("../../../test/mj-column-class.html"),
        );
    }

    #[test]
    fn with_padding() {
        compare_render(
            include_str!("../../../test/mj-column-padding.mjml"),
            include_str!("../../../test/mj-column-padding.html"),
        );
    }

    #[test]
    fn with_vertical_align() {
        compare_render(
            include_str!("../../../test/mj-column-vertical-align.mjml"),
            include_str!("../../../test/mj-column-vertical-align.html"),
        );
    }

    #[test]
    fn with_width() {
        compare_render(
            include_str!("../../../test/mj-column-width.mjml"),
            include_str!("../../../test/mj-column-width.html"),
        );
    }
}
