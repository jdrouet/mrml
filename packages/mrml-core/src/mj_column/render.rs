use super::{MjColumn, NAME};
use crate::helper::size::{Pixel, Size};
use crate::prelude::hash::Map;
use crate::prelude::render::*;

struct MjColumnExtra<'a> {
    attributes: Map<&'a str, &'a str>,
}

impl<'root> Renderer<'root, MjColumn, MjColumnExtra<'root>> {
    fn current_width(&self) -> Option<Pixel> {
        let parent_width = self.container_width.as_ref()?;
        let non_raw_siblings = self.non_raw_siblings();
        let borders = self.get_border_horizontal();
        let paddings = self.get_padding_horizontal();
        let inner_border_left = self
            .get_inner_border_left()
            .map(|size| size.value())
            .unwrap_or(0.0);
        let inner_border_right = self
            .get_inner_border_right()
            .map(|size| size.value())
            .unwrap_or(0.0);
        let inner_borders = inner_border_left + inner_border_right;
        let all_paddings = paddings.value() + borders.value() + inner_borders;

        let container_width = self
            .attribute_as_size("width")
            .unwrap_or_else(|| Size::pixel(parent_width.value() / (non_raw_siblings as f32)));
        if let Size::Percent(pc) = container_width {
            Some(Pixel::new(
                (parent_width.value() * pc.value() / 100.0) - all_paddings,
            ))
        } else {
            Some(Pixel::new(container_width.value() - all_paddings))
        }
    }

    fn non_raw_siblings(&self) -> usize {
        self.siblings - self.raw_siblings
    }

    fn get_parsed_width(&self) -> Size {
        self.attribute_as_size("width")
            .unwrap_or_else(|| Size::percent(100.0 / (self.non_raw_siblings() as f32)))
    }

    fn get_column_class(&self) -> (String, Size) {
        let parsed_width = self.get_parsed_width();
        let classname = if parsed_width.is_percent() {
            format!("mj-column-per-{}", parsed_width.value())
        } else {
            format!("mj-column-px-{}", parsed_width.value())
        };
        (classname.replace('.', "-"), parsed_width)
    }

    fn get_mobile_width(&self) -> Option<Size> {
        if !self.attribute_exists("mobile-width") {
            return Some(Size::percent(100.0));
        }
        if let Some(width) = self.attribute_as_size("width") {
            if width.is_percent() {
                Some(width)
            } else if width.is_pixel() {
                self.container_width
                    .as_ref()
                    .map(|w| Size::percent(width.value() / w.value()))
            } else {
                None
            }
        } else {
            Some(Size::percent(100.0 / (self.non_raw_siblings() as f32)))
        }
    }

    fn has_gutter(&self) -> bool {
        self.attribute_exists("padding")
            || self.attribute_exists("padding-bottom")
            || self.attribute_exists("padding-left")
            || self.attribute_exists("padding-right")
            || self.attribute_exists("padding-top")
    }

    fn get_width_as_pixel(&self) -> String {
        if let Some(ref container_width) = self.container_width {
            let parsed_width = self.get_parsed_width();
            match parsed_width {
                Size::Percent(value) => {
                    Pixel::new(container_width.value() * value.value() / 100.0).to_string()
                }
                _ => parsed_width.to_string(),
            }
        } else {
            String::from("100%")
        }
    }

    fn set_style_td_outlook<'a, 't>(&'a self, tag: Tag<'t>) -> Tag<'t>
    where
        'root: 'a,
        'a: 't,
    {
        tag.maybe_add_style("vertical-align", self.attribute("vertical-align"))
            .add_style("width", self.get_width_as_pixel())
    }

    fn set_style_root_div<'a, 't>(&'a self, tag: Tag<'t>) -> Tag<'t>
    where
        'root: 'a,
        'a: 't,
    {
        tag.add_style("font-size", "0px")
            .add_style("text-align", "left")
            .maybe_add_style("direction", self.attribute("direction"))
            .add_style("display", "inline-block")
            .maybe_add_style("vertical-align", self.attribute("vertical-align"))
            .maybe_add_style("width", self.get_mobile_width().map(|v| v.to_string()))
    }

    fn set_style_table_gutter<'a, 't>(&'a self, tag: Tag<'t>) -> Tag<'t>
    where
        'root: 'a,
        'a: 't,
    {
        tag.maybe_add_style(
            "background-color",
            self.attribute("inner-background-color")
                .or_else(|| self.attribute("background-color")),
        )
        .maybe_add_style(
            "border",
            self.attribute("inner-border")
                .or_else(|| self.attribute("border")),
        )
        .maybe_add_style(
            "border-bottom",
            self.attribute("inner-border-bottom")
                .or_else(|| self.attribute("border-bottom")),
        )
        .maybe_add_style(
            "border-left",
            self.attribute("inner-border-left")
                .or_else(|| self.attribute("border-left")),
        )
        .maybe_add_style(
            "border-radius",
            self.attribute("inner-border-radius")
                .or_else(|| self.attribute("border-radius")),
        )
        .maybe_add_style(
            "border-right",
            self.attribute("inner-border-right")
                .or_else(|| self.attribute("border-right")),
        )
        .maybe_add_style(
            "border-top",
            self.attribute("inner-border-top")
                .or_else(|| self.attribute("border-top")),
        )
    }

    fn set_style_table_simple<'a, 't>(&'a self, tag: Tag<'t>) -> Tag<'t>
    where
        'root: 'a,
        'a: 't,
    {
        tag.maybe_add_style("background-color", self.attribute("background-color"))
            .maybe_add_style("border", self.attribute("border"))
            .maybe_add_style("border-bottom", self.attribute("border-bottom"))
            .maybe_add_style("border-left", self.attribute("border-left"))
            .maybe_add_style("border-radius", self.attribute("border-radius"))
            .maybe_add_style("border-right", self.attribute("border-right"))
            .maybe_add_style("border-top", self.attribute("border-top"))
            .maybe_add_style("vertical-align", self.attribute("vertical-align"))
    }

    fn set_style_gutter_td<'a, 't>(&'a self, tag: Tag<'t>) -> Tag<'t>
    where
        'root: 'a,
        'a: 't,
    {
        self.set_style_table_simple(tag)
            .maybe_add_style("padding", self.attribute("padding"))
            .maybe_add_style("padding-top", self.attribute("padding-top"))
            .maybe_add_style("padding-right", self.attribute("padding-right"))
            .maybe_add_style("padding-bottom", self.attribute("padding-bottom"))
            .maybe_add_style("padding-left", self.attribute("padding-left"))
    }

    fn render_gutter(&self, cursor: &mut RenderCursor) -> Result<(), Error> {
        let table = Tag::table_presentation().add_attribute("width", "100%");
        let tbody = Tag::tbody();
        let tr = Tag::tr();
        let td = self.set_style_gutter_td(Tag::td());

        table.render_open(&mut cursor.buffer)?;
        tbody.render_open(&mut cursor.buffer)?;
        tr.render_open(&mut cursor.buffer)?;
        td.render_open(&mut cursor.buffer)?;
        self.render_column(cursor)?;
        td.render_close(&mut cursor.buffer);
        tr.render_close(&mut cursor.buffer);
        tbody.render_close(&mut cursor.buffer);
        table.render_close(&mut cursor.buffer);

        Ok(())
    }

    fn set_style_table<'a, 't>(&'a self, tag: Tag<'t>) -> Tag<'t>
    where
        'root: 'a,
        'a: 't,
    {
        if self.has_gutter() {
            self.set_style_table_gutter(tag)
        } else {
            self.set_style_table_simple(tag)
        }
    }

    fn render_column(&self, cursor: &mut RenderCursor) -> Result<(), Error> {
        let table = self
            .set_style_table(Tag::table_presentation())
            .add_attribute("width", "100%");
        let tbody = Tag::tbody();
        let siblings = self.element.children.len();
        let raw_siblings = self.element.children.iter().filter(|i| i.is_raw()).count();
        let current_width = self.current_width();

        table.render_open(&mut cursor.buffer)?;
        tbody.render_open(&mut cursor.buffer)?;

        for (index, child) in self.element.children.iter().enumerate() {
            let mut renderer = child.renderer(self.context());
            renderer.set_index(index);
            renderer.set_raw_siblings(raw_siblings);
            renderer.set_siblings(siblings);
            renderer.set_container_width(current_width.clone());
            if child.is_raw() {
                renderer.render(cursor)?;
            } else {
                let tr = Tag::tr();
                let td = Tag::td()
                    .maybe_add_style(
                        "background",
                        renderer.attribute("container-background-color"),
                    )
                    .add_style("font-size", "0px")
                    .maybe_add_style("padding", renderer.attribute("padding"))
                    .maybe_add_style("padding-top", renderer.attribute("padding-top"))
                    .maybe_add_style("padding-right", renderer.attribute("padding-right"))
                    .maybe_add_style("padding-bottom", renderer.attribute("padding-bottom"))
                    .maybe_add_style("padding-left", renderer.attribute("padding-left"))
                    .add_style("word-break", "break-word")
                    .maybe_add_attribute("align", renderer.attribute("align"))
                    .maybe_add_attribute("vertical-align", renderer.attribute("vertical-align"))
                    .maybe_add_class(renderer.attribute("css-class"));

                tr.render_open(&mut cursor.buffer)?;
                td.render_open(&mut cursor.buffer)?;
                renderer.render(cursor)?;
                td.render_close(&mut cursor.buffer);
                tr.render_close(&mut cursor.buffer);
            }
        }

        tbody.render_close(&mut cursor.buffer);
        table.render_close(&mut cursor.buffer);

        Ok(())
    }
}

impl<'root> Render<'root> for Renderer<'root, MjColumn, MjColumnExtra<'root>> {
    fn default_attribute(&self, name: &str) -> Option<&'static str> {
        match name {
            "direction" => Some("ltr"),
            "vertical-align" => Some("top"),
            _ => None,
        }
    }

    fn get_width(&self) -> Option<Size> {
        self.current_width().map(Size::Pixel)
    }

    fn raw_attribute(&self, key: &str) -> Option<&'root str> {
        self.element.attributes.get(key).map(|v| v.as_str())
    }

    fn raw_extra_attribute(&self, key: &str) -> Option<&'root str> {
        self.extra.attributes.get(key).copied()
    }

    fn add_extra_attribute(&mut self, key: &'root str, value: &'root str) {
        self.extra.attributes.insert(key, value);
    }

    fn tag(&self) -> Option<&str> {
        Some(NAME)
    }

    fn context(&self) -> &'root RenderContext<'root> {
        self.context
    }

    fn set_container_width(&mut self, width: Option<Pixel>) {
        self.container_width = width;
    }

    fn set_siblings(&mut self, value: usize) {
        self.siblings = value;
    }

    fn set_raw_siblings(&mut self, value: usize) {
        self.raw_siblings = value;
    }

    fn set_style<'a, 't>(&'a self, name: &str, tag: Tag<'t>) -> Tag<'t>
    where
        'root: 'a,
        'a: 't,
    {
        match name {
            "td-outlook" => self.set_style_td_outlook(tag),
            _ => tag,
        }
    }

    fn render(&self, cursor: &mut RenderCursor) -> Result<(), Error> {
        let (classname, size) = self.get_column_class();
        cursor.header.add_media_query(classname.clone(), size);

        let div = self
            .set_style_root_div(Tag::div())
            .add_class("mj-outlook-group-fix")
            .add_class(classname)
            .maybe_add_class(self.attribute("css-class"));

        div.render_open(&mut cursor.buffer)?;
        if self.has_gutter() {
            self.render_gutter(cursor)?;
        } else {
            self.render_column(cursor)?;
        }
        div.render_close(&mut cursor.buffer);
        Ok(())
    }
}

impl<'render, 'root: 'render> Renderable<'render, 'root> for MjColumn {
    fn renderer(
        &'root self,
        context: &'root RenderContext<'root>,
    ) -> Box<dyn Render<'root> + 'render> {
        Box::new(Renderer::new(
            context,
            self,
            MjColumnExtra {
                attributes: Map::new(),
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    crate::should_render!(basic, "mj-column");
    crate::should_render!(background_color, "mj-column-background-color");
    crate::should_render!(border_radius, "mj-column-border-radius");
    crate::should_render!(border, "mj-column-border");
    crate::should_render!(class, "mj-column-class");
    crate::should_render!(inner_background_color, "mj-column-inner-background-color");
    crate::should_render!(padding, "mj-column-padding");
    crate::should_render!(vertical_align, "mj-column-vertical-align");
    crate::should_render!(width, "mj-column-width");
}
