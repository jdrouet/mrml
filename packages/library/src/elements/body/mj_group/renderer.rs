use super::MJGroup;
use crate::elements::body::mj_body::children::MJBodyChild;
use crate::elements::body::prelude::*;
use crate::elements::error::Error;
use crate::elements::prelude::*;
use crate::util::attributes::*;
use crate::util::condition::*;
use crate::util::context::Context;
use crate::util::header::Header;
use crate::util::size::Size;
use crate::util::tag::Tag;
use std::str::FromStr;

impl MJGroup {
    fn get_width(&self) -> Option<Size> {
        self.get_current_width().and_then(|width| {
            if width.is_percent() {
                self.get_container_width()
                    .map(|container| Size::Pixel(container.value() * width.value() / 100.0))
            } else {
                Some(width)
            }
        })
    }

    fn set_style_div(&self, tag: Tag) -> Tag {
        tag.set_style("font-size", "0")
            .set_style("line-height", "0")
            .set_style("text-align", "left")
            .set_style("display", "inline-block")
            .set_style("width", "100%")
            .maybe_set_style("background-color", self.get_attribute("background-color"))
            .maybe_set_style("direction", self.get_attribute("direction"))
            .maybe_set_style("vertical-align", self.get_attribute("vertical-align"))
    }

    fn set_style_td_outlook(&self, tag: Tag) -> Tag {
        tag.maybe_set_style("vertical-align", self.get_attribute("vertical-align"))
            .maybe_set_style("width", self.get_width())
    }

    fn get_parsed_width(&self) -> Size {
        let non_raw_siblings = self
            .context()
            .map(|ctx| ctx.non_raw_siblings())
            .unwrap_or(1);
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

    fn render_child(&self, header: &Header, child: &MJBodyChild) -> Result<String, Error> {
        let td = Tag::td()
            .maybe_set_style("align", child.get_attribute("align"))
            .maybe_set_style("vertical-align", child.get_attribute("vertical-align"))
            .maybe_set_style(
                "width",
                child.get_width().or_else(|| {
                    child
                        .get_attribute("width")
                        .and_then(|value| Size::from_str(value.as_str()).ok())
                }),
            );
        Ok(format!(
            "{}{}{}",
            conditional_tag(td.open()),
            child.render(header)?,
            conditional_tag(td.close())
        ))
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
        self.context = Some(ctx);
        let child_base = Context::new(
            self.get_current_width(),
            self.get_siblings(),
            self.get_raw_siblings(),
            0,
        );
        for (idx, child) in self.children.iter_mut().enumerate() {
            child.set_context(child_base.clone().set_index(idx));
        }
    }

    fn render(&self, header: &Header) -> Result<String, Error> {
        let div = self
            .set_style_div(Tag::div())
            .set_class(self.get_column_class().0)
            .set_class("mj-outlook-group-fix")
            .maybe_set_class(self.get_attribute("css-class"));
        let table = Tag::table_presentation();
        let tr = Tag::tr();
        let mut res: Vec<String> = vec![];
        res.push(div.open());
        res.push(START_CONDITIONAL_TAG.into());
        res.push(table.open());
        res.push(tr.open());
        res.push(END_CONDITIONAL_TAG.into());
        res.push(self.render_children(header)?);
        res.push(START_CONDITIONAL_TAG.into());
        res.push(tr.close());
        res.push(table.close());
        res.push(END_CONDITIONAL_TAG.into());
        res.push(div.close());
        Ok(res.join(""))
    }
}

impl BodyComponent for MJGroup {
    fn attributes(&self) -> Option<&Attributes> {
        Some(&self.attributes)
    }

    fn get_children<'p>(&'p self) -> Box<dyn Iterator<Item = &'p MJBodyChild> + 'p> {
        Box::new(self.children.iter())
    }

    fn get_children_len(&self) -> usize {
        self.children.len()
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

    fn set_style(&self, key: &str, tag: Tag) -> Tag {
        match key {
            "div" => self.set_style_div(tag),
            "td-outlook" => self.set_style_td_outlook(tag),
            _ => tag,
        }
    }
}
