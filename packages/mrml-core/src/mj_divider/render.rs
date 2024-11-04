use super::{MjDivider, NAME};
use crate::helper::size::{Pixel, Size};
use crate::prelude::render::*;

impl<'root> Renderer<'root, MjDivider, ()> {
    fn set_style_p_without_width<'t>(&self, tag: Tag<'t>) -> Tag<'t> {
        tag.add_style(
            "border-top",
            format!(
                "{} {} {}",
                self.attribute("border-style")
                    .unwrap_or(DEFAULT_BORDER_STYLE),
                self.attribute("border-width")
                    .unwrap_or(DEFAULT_BORDER_WIDTH),
                self.attribute("border-color")
                    .unwrap_or(DEFAULT_BORDER_COLOR)
            ),
        )
        .add_style("font-size", "1px")
        .add_style("margin", "0px auto")
    }
    fn set_style_p<'a, 't>(&'a self, tag: Tag<'t>) -> Tag<'t>
    where
        'root: 'a,
        'a: 't,
    {
        self.set_style_p_without_width(tag)
            .maybe_add_style("width", self.attribute("width"))
    }

    fn set_style_outlook<'t>(&self, tag: Tag<'t>) -> Tag<'t> {
        self.set_style_p_without_width(tag)
            .maybe_add_style("width", self.get_outlook_width().map(|v| v.to_string()))
    }

    fn get_outlook_width(&self) -> Option<Pixel> {
        let container_width = self.container_width.as_ref()?;
        let padding_horizontal = self.get_padding_horizontal();
        let width = self
            .attribute_as_size("width")
            .unwrap_or_else(|| Size::percent(100.0));
        Some(match width {
            Size::Percent(value) => {
                let effective = container_width.value() - padding_horizontal.value();
                let multiplier = value.value() / 100.0;
                Pixel::new(effective * multiplier)
            }
            Size::Pixel(value) => value,
            _ => Pixel::new(container_width.value() - padding_horizontal.value()),
        })
    }

    fn render_after(&self, buf: &mut RenderBuffer) -> Result<(), Error> {
        let table = self
            .set_style_outlook(Tag::table_presentation())
            .add_attribute("align", "center")
            .maybe_add_attribute("width", self.get_outlook_width().map(|v| v.to_string()));
        let tr = Tag::tr();
        let td = Tag::td()
            .add_style("height", "0")
            .add_style("line-height", "0");

        buf.start_conditional_tag();
        table.render_open(buf)?;
        tr.render_open(buf)?;
        td.render_text(buf, "&nbsp;")?;
        tr.render_close(buf);
        table.render_close(buf);
        buf.end_conditional_tag();

        Ok(())
    }
}

const DEFAULT_BORDER_COLOR: &str = "#000000";
const DEFAULT_BORDER_STYLE: &str = "solid";
const DEFAULT_BORDER_WIDTH: &str = "4px";

impl<'root> Render<'root> for Renderer<'root, MjDivider, ()> {
    fn default_attribute(&self, key: &str) -> Option<&'static str> {
        match key {
            "align" => Some("center"),
            "border-color" => Some(DEFAULT_BORDER_COLOR),
            "border-style" => Some(DEFAULT_BORDER_STYLE),
            "border-width" => Some(DEFAULT_BORDER_WIDTH),
            "padding" => Some("10px 25px"),
            "width" => Some("100%"),
            _ => None,
        }
    }

    fn raw_attribute(&self, key: &str) -> Option<&'root str> {
        match self.element.attributes.get(key) {
            Some(Some(inner)) => Some(inner),
            _ => None,
        }
    }

    fn tag(&self) -> Option<&str> {
        Some(NAME)
    }

    fn set_container_width(&mut self, width: Option<Pixel>) {
        self.container_width = width;
    }

    fn context(&self) -> &'root RenderContext<'root> {
        self.context
    }

    fn render(&self, cursor: &mut RenderCursor) -> Result<(), Error> {
        let p = self.set_style_p(Tag::new("p"));
        p.render_text(&mut cursor.buffer, "")?;

        self.render_after(&mut cursor.buffer)?;
        Ok(())
    }
}

impl<'render, 'root: 'render> Renderable<'render, 'root> for MjDivider {
    fn renderer(
        &'root self,
        context: &'root RenderContext<'root>,
    ) -> Box<dyn Render<'root> + 'render> {
        Box::new(Renderer::new(context, self, ()))
    }
}

#[cfg(test)]
mod tests {
    crate::should_render!(basic, "mj-divider");
    crate::should_render!(class, "mj-divider-class");
    crate::should_render!(
        container_background_color,
        "mj-divider-container-background-color"
    );
    crate::should_render!(padding, "mj-divider-padding");
    crate::should_render!(width, "mj-divider-width");
}
