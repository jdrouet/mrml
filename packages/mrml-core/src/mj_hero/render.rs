use std::borrow::Cow;

use super::{MjHero, NAME};
use crate::helper::size::Pixel;
use crate::prelude::render::*;

impl<'root> Renderer<'root, MjHero, ()> {
    fn set_style_div<'t>(&self, tag: Tag<'t>) -> Tag<'t> {
        tag.add_style("margin", "0 auto").maybe_add_style(
            "max-width",
            self.container_width.as_ref().map(|w| w.to_string()),
        )
    }

    fn set_style_table<'t>(&self, tag: Tag<'t>) -> Tag<'t> {
        tag.add_style("width", "100%")
    }

    fn set_style_tr<'t>(&self, tag: Tag<'t>) -> Tag<'t> {
        tag.add_style("vertical-align", "top")
    }

    fn set_style_td_fluid<'t>(&self, tag: Tag<'t>) -> Tag<'t> {
        // TODO check size type compatibility
        let bg_ratio = self
            .attribute_as_size("background-height")
            .and_then(|height| {
                self.attribute_as_size("background-width")
                    .map(|width| height.value() * 100.0 / width.value())
            });
        tag.add_style("mso-padding-bottom-alt", "0")
            .maybe_add_style("padding-bottom", bg_ratio.map(|v| v.to_string()))
            .add_style("width", "0.01%")
    }

    fn set_style_outlook_table<'t>(&self, tag: Tag<'t>) -> Tag<'t> {
        tag.maybe_add_style(
            "width",
            self.container_width.as_ref().map(|w| w.to_string()),
        )
    }

    fn set_style_outlook_inner_table<'t>(&self, tag: Tag<'t>) -> Tag<'t> {
        self.set_style_outlook_table(tag)
    }

    fn set_style_outlook_inner_td<'a, 't>(&'a self, tag: Tag<'t>) -> Tag<'t>
    where
        'root: 'a,
        'a: 't,
    {
        tag.maybe_add_style("background-color", self.attribute("inner-background-color"))
            .maybe_add_style("padding", self.attribute("inner-padding"))
            .maybe_add_style("padding-top", self.attribute("inner-padding-top"))
            .maybe_add_style("padding-right", self.attribute("inner-padding-right"))
            .maybe_add_style("padding-bottom", self.attribute("inner-padding-bottom"))
            .maybe_add_style("padding-left", self.attribute("inner-padding-left"))
    }

    fn set_style_inner_div<'a, 't>(&'a self, tag: Tag<'t>) -> Tag<'t>
    where
        'root: 'a,
        'a: 't,
    {
        tag.maybe_add_style("background-color", self.attribute("inner-background-color"))
            .maybe_add_style("float", self.attribute("align"))
            .add_style("margin", "0px auto")
            .maybe_add_style("width", self.attribute("width"))
    }

    fn set_style_inner_table<'t>(&self, tag: Tag<'t>) -> Tag<'t> {
        tag.add_style("width", "100%").add_style("margin", "0px")
    }

    fn set_style_outlook_image<'a, 't>(&'a self, tag: Tag<'t>) -> Tag<'t>
    where
        'root: 'a,
        'a: 't,
    {
        tag.add_style("border", "0")
            .maybe_add_style("height", self.attribute("background-height"))
            .add_style("mso-position-horizontal", "center")
            .add_style("position", "absolute")
            .add_style("top", "0")
            .maybe_add_style(
                "width",
                self.attribute("background-width")
                    .map(Cow::Borrowed)
                    .or_else(|| {
                        self.container_width
                            .as_ref()
                            .map(|w| Cow::Owned(w.to_string()))
                    }),
            )
            .add_style("z-index", "-3")
    }

    fn set_style_outlook_td<'t>(&self, tag: Tag<'t>) -> Tag<'t> {
        tag.add_style("line-height", "0")
            .add_style("font-size", "0")
            .add_style("mso-line-height-rule", "exactly")
    }

    fn get_background<'a>(&'a self) -> Option<Cow<'a, str>>
    where
        'root: 'a,
    {
        if let (Some(url), Some(color), Some(position)) = (
            self.attribute("background-url"),
            self.attribute("background-color"),
            self.attribute("background-position"),
        ) {
            Some(Cow::Owned(format!(
                "{} url('{}') no-repeat {} / cover",
                // has default value
                color,
                url,
                // has default value
                position
            )))
        } else {
            self.attribute("background-color").map(Cow::Borrowed)
        }
    }

    fn set_style_hero<'a, 't>(&'a self, tag: Tag<'t>) -> Tag<'t>
    where
        'root: 'a,
        'a: 't,
    {
        tag.maybe_add_style("background", self.get_background())
            .maybe_add_style("background-position", self.attribute("background-position"))
            .add_style("background-repeat", "no-repeat")
            .maybe_add_style("padding", self.attribute("padding"))
            .maybe_add_style("padding-top", self.attribute("padding-top"))
            .maybe_add_style("padding-right", self.attribute("padding-right"))
            .maybe_add_style("padding-bottom", self.attribute("padding-bottom"))
            .maybe_add_style("padding-left", self.attribute("padding-left"))
            .maybe_add_style("vertical-align", self.attribute("vertical-align"))
    }

    fn render_children(&self, cursor: &mut RenderCursor) -> Result<(), Error> {
        let siblings = self.element.children.len();
        let raw_siblings = self.element.children.iter().filter(|c| c.is_raw()).count();
        for (index, child) in self.element.children.iter().enumerate() {
            let mut renderer = child.renderer(self.context());
            renderer.set_index(index);
            renderer.set_siblings(siblings);
            renderer.set_raw_siblings(raw_siblings);
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
                    .maybe_add_attribute(
                        "background",
                        renderer.attribute("container-background-color"),
                    )
                    .maybe_add_attribute("class", renderer.attribute("css-class"));

                tr.render_open(&mut cursor.buffer)?;
                td.render_open(&mut cursor.buffer)?;
                renderer.render(cursor)?;
                td.render_close(&mut cursor.buffer);
                tr.render_close(&mut cursor.buffer);
            };
        }

        Ok(())
    }

    fn render_content(&self, cursor: &mut RenderCursor) -> Result<(), Error> {
        let table = self
            .set_style_outlook_inner_table(Tag::table_borderless())
            .maybe_add_attribute("align", self.attribute("align"))
            .maybe_add_attribute(
                "width",
                self.container_width.as_ref().map(|w| w.value().to_string()),
            );
        let tbody = Tag::tbody();
        let tr = Tag::tr();
        let td = Tag::td();
        let outlook_inner_td = self.set_style_outlook_inner_td(Tag::td());
        let outlook_inner_div = self
            .set_style_inner_div(Tag::div())
            .maybe_add_attribute("width", self.attribute("align"))
            .add_class("mj-hero-content");
        let inner_table = self.set_style_inner_table(Tag::table_presentation());

        cursor.buffer.start_conditional_tag();
        table.render_open(&mut cursor.buffer)?;
        tr.render_open(&mut cursor.buffer)?;
        outlook_inner_td.render_open(&mut cursor.buffer)?;
        cursor.buffer.end_conditional_tag();

        outlook_inner_div.render_open(&mut cursor.buffer)?;
        inner_table.render_open(&mut cursor.buffer)?;
        tbody.render_open(&mut cursor.buffer)?;
        tr.render_open(&mut cursor.buffer)?;
        td.render_open(&mut cursor.buffer)?;
        inner_table.render_open(&mut cursor.buffer)?;
        tbody.render_open(&mut cursor.buffer)?;
        self.render_children(cursor)?;
        tbody.render_close(&mut cursor.buffer);
        inner_table.render_close(&mut cursor.buffer);
        td.render_close(&mut cursor.buffer);
        tr.render_close(&mut cursor.buffer);
        tbody.render_close(&mut cursor.buffer);
        inner_table.render_close(&mut cursor.buffer);
        outlook_inner_div.render_close(&mut cursor.buffer);

        cursor.buffer.start_conditional_tag();
        outlook_inner_td.render_close(&mut cursor.buffer);
        tr.render_close(&mut cursor.buffer);
        table.render_close(&mut cursor.buffer);
        cursor.buffer.end_conditional_tag();

        Ok(())
    }

    fn render_mode_fluid(&self, cursor: &mut RenderCursor) -> Result<(), Error> {
        let td_fluid = self.set_style_td_fluid(Tag::td());
        let td = self
            .set_style_hero(Tag::td())
            .maybe_add_attribute("background", self.attribute("background-url"));

        td_fluid.render_closed(&mut cursor.buffer)?;
        td.render_open(&mut cursor.buffer)?;
        self.render_content(cursor)?;
        td.render_close(&mut cursor.buffer);
        td_fluid.render_closed(&mut cursor.buffer)?;

        Ok(())
    }

    fn render_mode_fixed(&self, cursor: &mut RenderCursor) -> Result<(), Error> {
        // has a default value
        let height = self.attribute_as_pixel("height").unwrap().value();
        let padding = self.get_padding_vertical().value();
        let height = height - padding;
        let td = self
            .set_style_hero(Tag::td())
            .add_style("height", format!("{height}px"))
            .maybe_add_attribute("background", self.attribute("background-url"))
            .add_attribute("height", height.to_string());

        td.render_open(&mut cursor.buffer)?;
        self.render_content(cursor)?;
        td.render_close(&mut cursor.buffer);

        Ok(())
    }

    fn render_mode(&self, cursor: &mut RenderCursor) -> Result<(), Error> {
        match self.attribute("mode") {
            Some(inner) if inner.eq("fluid") => self.render_mode_fluid(cursor),
            _ => self.render_mode_fixed(cursor),
        }
    }
}

impl<'root> Render<'root> for Renderer<'root, MjHero, ()> {
    fn default_attribute(&self, name: &str) -> Option<&'static str> {
        match name {
            "background-color" => Some("#ffffff"),
            "background-position" => Some("center center"),
            "height" => Some("0px"),
            "mode" => Some("fixed-height"),
            "padding" => Some("0px"),
            "vertical-align" => Some("top"),
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

    fn set_container_width(&mut self, width: Option<Pixel>) {
        self.container_width = width;
    }

    fn set_siblings(&mut self, value: usize) {
        self.siblings = value;
    }

    fn set_raw_siblings(&mut self, value: usize) {
        self.raw_siblings = value;
    }

    fn render(&self, cursor: &mut RenderCursor) -> Result<(), Error> {
        let outlook_table = self
            .set_style_outlook_table(Tag::table_presentation())
            .add_attribute("align", "center")
            .maybe_add_attribute(
                "width",
                self.container_width.as_ref().map(|v| v.value().to_string()),
            );
        let outlook_tr = Tag::tr();
        let outlook_td = self.set_style_outlook_td(Tag::td());
        let v_image = self
            .set_style_outlook_image(Tag::new("v:image"))
            .maybe_add_attribute("src", self.attribute("background-url"))
            .add_attribute("xmlns:v", "urn:schemas-microsoft-com:vml");
        let div = self
            .set_style_div(Tag::div())
            .maybe_add_attribute("align", self.attribute("align"))
            .maybe_add_class(self.attribute("css-class"));
        let table = self.set_style_table(Tag::table_presentation());
        let tbody = Tag::tbody();
        let tr = self.set_style_tr(Tag::tr());

        cursor.buffer.start_conditional_tag();
        outlook_table.render_open(&mut cursor.buffer)?;
        outlook_tr.render_open(&mut cursor.buffer)?;
        outlook_td.render_open(&mut cursor.buffer)?;
        v_image.render_closed(&mut cursor.buffer)?;
        cursor.buffer.end_conditional_tag();

        div.render_open(&mut cursor.buffer)?;
        table.render_open(&mut cursor.buffer)?;
        tbody.render_open(&mut cursor.buffer)?;
        tr.render_open(&mut cursor.buffer)?;

        self.render_mode(cursor)?;

        tr.render_close(&mut cursor.buffer);
        tbody.render_close(&mut cursor.buffer);
        table.render_close(&mut cursor.buffer);
        div.render_close(&mut cursor.buffer);

        cursor.buffer.start_conditional_tag();
        outlook_td.render_close(&mut cursor.buffer);
        outlook_tr.render_close(&mut cursor.buffer);
        outlook_table.render_close(&mut cursor.buffer);
        cursor.buffer.end_conditional_tag();

        Ok(())
    }
}

impl<'render, 'root: 'render> Renderable<'render, 'root> for MjHero {
    fn renderer(
        &'root self,
        context: &'root RenderContext<'root>,
    ) -> Box<dyn Render<'root> + 'render> {
        Box::new(Renderer::new(context, self, ()))
    }
}

#[cfg(test)]
mod tests {
    crate::should_render!(basic, "mj-hero");
    crate::should_render!(background_color, "mj-hero-background-color");
    crate::should_render!(background_height, "mj-hero-background-height");
    crate::should_render!(background_position, "mj-hero-background-position");
    crate::should_render!(background_url, "mj-hero-background-url");
    crate::should_render!(background_width, "mj-hero-background-width");
    crate::should_render!(class, "mj-hero-class");
    crate::should_render!(height, "mj-hero-height");
    crate::should_render!(mode, "mj-hero-mode");
    crate::should_render!(vertical_align, "mj-hero-vertical-align");
    crate::should_render!(width, "mj-hero-width");
}
