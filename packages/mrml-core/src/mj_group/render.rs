use std::borrow::Cow;

use super::{MjGroup, NAME};
use crate::helper::size::{Pixel, Size};
use crate::prelude::render::*;

impl<'root> Renderer<'root, MjGroup, ()> {
    fn current_width(&self) -> Pixel {
        let parent_width = self.container_width.as_ref().unwrap();
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
            Pixel::new((parent_width.value() * pc.value() / 100.0) - all_paddings)
        } else {
            Pixel::new(container_width.value() - all_paddings)
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

    fn set_style_root_div<'a, 't>(&'a self, tag: Tag<'t>) -> Tag<'t>
    where
        'root: 'a,
        'a: 't,
    {
        tag.add_style("font-size", "0")
            .add_style("line-height", "0")
            .add_style("text-align", "left")
            .add_style("display", "inline-block")
            .add_style("width", "100%")
            .maybe_add_style("direction", self.attribute("direction"))
            .maybe_add_style("background-color", self.attribute("background-color"))
            .maybe_add_style("vertical-align", self.attribute("vertical-align"))
    }

    fn set_style_td_outlook<'a, 't>(&'a self, tag: Tag<'t>) -> Tag<'t>
    where
        'root: 'a,
        'a: 't,
    {
        tag.maybe_add_style("vertical-align", self.attribute("vertical-align"))
            .add_style("width", self.current_width().to_string())
    }

    fn render_children(&self, cursor: &mut RenderCursor) -> Result<(), Error> {
        let current_width = self.current_width();
        let siblings = self.element.children.len();
        let raw_siblings = self
            .element
            .children
            .iter()
            .filter(|item| item.is_raw())
            .count();

        for (index, child) in self.element.children.iter().enumerate() {
            let mut renderer = child.renderer(self.context());
            renderer.set_index(index);
            renderer.set_siblings(siblings);
            renderer.set_raw_siblings(raw_siblings);
            renderer.set_container_width(Some(current_width.clone()));
            renderer.add_extra_attribute("mobile-width", "mobile-width");
            if child.is_raw() {
                renderer.render(cursor)?;
            } else {
                let td = Tag::td()
                    .maybe_add_style("align", renderer.attribute("align"))
                    .maybe_add_style("vertical-align", renderer.attribute("vertical-align"))
                    .maybe_add_style(
                        "width",
                        renderer
                            .get_width()
                            .map(|w| Cow::Owned(w.to_string()))
                            .or_else(|| renderer.attribute("width").map(Cow::Borrowed)),
                    );

                cursor.buffer.start_conditional_tag();
                td.render_open(&mut cursor.buffer)?;
                cursor.buffer.end_conditional_tag();
                renderer.render(cursor)?;
                cursor.buffer.start_conditional_tag();
                td.render_close(&mut cursor.buffer);
                cursor.buffer.end_conditional_tag();
            }
        }
        Ok(())
    }
}

impl<'root> Render<'root> for Renderer<'root, MjGroup, ()> {
    fn default_attribute(&self, name: &str) -> Option<&'static str> {
        match name {
            "direction" => Some("ltr"),
            _ => None,
        }
    }

    fn raw_attribute(&self, key: &str) -> Option<&'root str> {
        self.element.attributes.get(key).map(|v| v.as_str())
    }

    fn tag(&self) -> Option<&str> {
        Some(NAME)
    }

    fn context(&self) -> &'root RenderContext<'root> {
        self.context
    }

    fn get_width(&self) -> Option<Size> {
        Some(Size::Pixel(self.current_width()))
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
            .add_class(classname)
            .add_class("mj-outlook-group-fix")
            .maybe_add_class(self.attribute("css-class"));
        let table = Tag::table_presentation().maybe_add_attribute(
            "bgcolor",
            self.attribute("background-color").and_then(|color| {
                if color == "none" {
                    None
                } else {
                    Some(color)
                }
            }),
        );
        let tr = Tag::tr();

        div.render_open(&mut cursor.buffer)?;
        cursor.buffer.start_conditional_tag();
        table.render_open(&mut cursor.buffer)?;
        tr.render_open(&mut cursor.buffer)?;
        cursor.buffer.end_conditional_tag();
        self.render_children(cursor)?;
        cursor.buffer.start_conditional_tag();
        tr.render_close(&mut cursor.buffer);
        table.render_close(&mut cursor.buffer);
        cursor.buffer.end_conditional_tag();
        div.render_close(&mut cursor.buffer);

        Ok(())
    }
}

impl<'render, 'root: 'render> Renderable<'render, 'root> for MjGroup {
    fn renderer(
        &'root self,
        context: &'root RenderContext<'root>,
    ) -> Box<dyn Render<'root> + 'render> {
        Box::new(Renderer::new(context, self, ()))
    }
}

#[cfg(test)]
mod tests {
    crate::should_render!(basic, "mj-group");
    crate::should_render!(background_color, "mj-group-background-color");
    crate::should_render!(class, "mj-group-class");
    crate::should_render!(direction, "mj-group-direction");
    crate::should_render!(vertical_align, "mj-group-vertical-align");
    crate::should_render!(width, "mj-group-width");
}
