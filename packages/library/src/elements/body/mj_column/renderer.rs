use super::MJColumn;
use crate::elements::body::mj_body::children::MJBodyChild;
use crate::elements::body::prelude::*;
use crate::elements::error::Error;
use crate::elements::prelude::*;
use crate::util::attributes::*;
use crate::util::context::Context;
use crate::util::header::Header;
use crate::util::size::Size;
use crate::util::tag::Tag;

impl MJColumn {
    fn get_column_class(&self) -> (String, Size) {
        let parsed_width = self.get_parsed_width();
        let classname = match parsed_width {
            Size::Percent(value) => format!("mj-column-per-{}", value),
            _ => format!("mj-column-px-{}", parsed_width.value()),
        };
        (classname.replace(".", "-"), parsed_width)
    }

    fn get_mobile_width(&self) -> Option<Size> {
        if self.get_attribute("mobile-width").is_none() {
            return Some(Size::Percent(100.0));
        }
        let width = self.get_size_attribute("width");
        if width.is_none() {
            self.context()
                .map(|ctx| ctx.non_raw_siblings())
                .map(|count| Size::Percent(100.0 / (count as f32)))
        } else {
            width.and_then(|width| match width {
                Size::Percent(_) => Some(width),
                _ => match self.get_container_width() {
                    Some(container) => Some(Size::Percent(width.value() / container.value())),
                    None => None,
                },
            })
        }
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

    fn set_style_div(&self, tag: Tag) -> Tag {
        tag.set_style("font-size", "0px")
            .set_style("text-align", "left")
            .maybe_set_style("direction", self.get_attribute("direction"))
            .set_style("display", "inline-block")
            .maybe_set_style("vertical-align", self.get_attribute("vertical-align"))
            .maybe_set_style("width", self.get_mobile_width())
    }

    fn set_style_table_gutter(&self, tag: Tag) -> Tag {
        tag.maybe_set_style("background-color", self.get_attribute("background-color"))
            .maybe_set_style("border", self.get_attribute("border"))
            .maybe_set_style("border-bottom", self.get_attribute("border-bottom"))
            .maybe_set_style("border-left", self.get_attribute("border-left"))
            .maybe_set_style("border-radius", self.get_attribute("border-radius"))
            .maybe_set_style("border-right", self.get_attribute("border-right"))
            .maybe_set_style("border-top", self.get_attribute("border-top"))
    }

    fn set_style_table_simple(&self, tag: Tag) -> Tag {
        self.set_style_table_gutter(tag)
            .maybe_set_style("vertical-align", self.get_attribute("vertical-align"))
    }

    fn set_style_table(&self, tag: Tag) -> Tag {
        if self.has_gutter() {
            self.set_style_table_gutter(tag)
        } else {
            self.set_style_table_simple(tag)
        }
    }

    fn set_style_td_outlook(&self, tag: Tag) -> Tag {
        tag.maybe_set_style("vertical-align", self.get_attribute("vertical-align"))
            .set_style("width", self.get_width_as_pixel())
    }

    fn set_style_gutter(&self, tag: Tag) -> Tag {
        self.set_style_table_simple(tag)
            .maybe_set_style("padding", self.get_attribute("padding"))
            .maybe_set_style("padding-top", self.get_attribute("padding-top"))
            .maybe_set_style("padding-right", self.get_attribute("padding-right"))
            .maybe_set_style("padding-bottom", self.get_attribute("padding-bottom"))
            .maybe_set_style("padding-left", self.get_attribute("padding-left"))
    }

    fn render_gutter(&self, header: &Header) -> Result<String, Error> {
        let table = Tag::table_presentation().set_attribute("width", "100%");
        let tbody = Tag::tbody();
        let tr = Tag::tr();
        let td = self.set_style_gutter(Tag::td());
        Ok(table.render(tbody.render(tr.render(td.render(self.render_column(header)?)))))
    }

    fn render_mj_child(&self, header: &Header, child: &MJBodyChild) -> Result<String, Error> {
        let tr = Tag::tr();
        let td = Tag::td()
            .maybe_set_style(
                "background",
                child.get_attribute("container-background-color"),
            )
            .set_style("font-size", "0px")
            .maybe_set_style("padding", child.get_attribute("padding"))
            .maybe_set_style("padding-top", child.get_attribute("padding-top"))
            .maybe_set_style("padding-right", child.get_attribute("padding-right"))
            .maybe_set_style("padding-bottom", child.get_attribute("padding-bottom"))
            .maybe_set_style("padding-left", child.get_attribute("padding-left"))
            .set_style("word-break", "break-word")
            .maybe_set_attribute("align", child.get_attribute("align"))
            .maybe_set_attribute("vertical-align", child.get_attribute("vertical-align"))
            .maybe_set_attribute("class", child.get_attribute("css-class"));
        Ok(tr.render(td.render(child.render(header)?)))
    }

    fn render_column(&self, header: &Header) -> Result<String, Error> {
        let table = self
            .set_style_table(Tag::table_presentation())
            .set_attribute("width", "100%");
        let mut res = vec![];
        res.push(table.open());
        for child in self.children.iter() {
            if child.is_raw() {
                res.push(child.render(header)?);
            } else {
                res.push(self.render_mj_child(header, &child)?);
            }
        }
        res.push(table.close());
        Ok(res.join(""))
    }
}

impl Component for MJColumn {
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
        let (classname, _size) = self.get_column_class();
        Ok(self
            .set_style_div(Tag::div())
            .set_class("mj-outlook-group-fix")
            .set_class(classname)
            .maybe_set_class(self.get_attribute("css-class"))
            .render(if self.has_gutter() {
                self.render_gutter(header)?
            } else {
                self.render_column(header)?
            }))
    }
}

impl BodyComponent for MJColumn {
    fn attributes(&self) -> Option<&Attributes> {
        Some(&self.attributes)
    }

    fn get_children(&self) -> &Vec<MJBodyChild> {
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

    fn set_style(&self, key: &str, tag: Tag) -> Tag {
        match key {
            "div" => self.set_style_div(tag),
            "table" => self.set_style_table(tag),
            "td-outlook" => self.set_style_td_outlook(tag),
            "gutter" => self.set_style_gutter(tag),
            _ => tag,
        }
    }

    fn get_width(&self) -> Option<Size> {
        self.get_container_width().map(|container_width| {
            let parsed_width = self.get_parsed_width();
            match parsed_width {
                Size::Percent(value) => Size::Pixel(container_width.value() * value / 100.0),
                _ => parsed_width,
            }
        })
    }
}
