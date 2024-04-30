use super::{MjDivider, NAME};
use crate::helper::size::{Pixel, Size};
use crate::prelude::render::*;

struct MjDividerRender<'e, 'h> {
    header: &'h Header<'h>,
    element: &'e MjDivider,
    container_width: Option<Pixel>,
}

impl<'e, 'h> MjDividerRender<'e, 'h> {
    fn set_style_p_without_width<'a>(&self, tag: Tag<'a>) -> Tag<'a> {
        tag.add_style(
            "border-top",
            format!(
                "{} {} {}",
                self.attribute("border-style").unwrap(),
                self.attribute("border-width").unwrap(),
                self.attribute("border-color").unwrap()
            ),
        )
        .add_style("font-size", "1px")
        .add_style("margin", "0px auto")
    }
    fn set_style_p<'a>(&self, tag: Tag<'a>) -> Tag<'a> {
        self.set_style_p_without_width(tag)
            .maybe_add_style("width", self.attribute("width"))
    }

    fn set_style_outlook<'a>(&self, tag: Tag<'a>) -> Tag<'a> {
        self.set_style_p_without_width(tag)
            .add_style("width", self.get_outlook_width().to_string())
    }

    fn get_outlook_width(&self) -> Pixel {
        let container_width = self.container_width.as_ref().unwrap();
        let padding_horizontal = self.get_padding_horizontal();
        let width = self
            .attribute_as_size("width")
            .unwrap_or_else(|| Size::percent(100.0));
        match width {
            Size::Percent(value) => {
                let effective = container_width.value() - padding_horizontal.value();
                let multiplier = value.value() / 100.0;
                Pixel::new(effective * multiplier)
            }
            Size::Pixel(value) => value,
            _ => Pixel::new(container_width.value() - padding_horizontal.value()),
        }
    }

    fn render_after(&self, buf: &mut RenderBuffer) {
        let table = self
            .set_style_outlook(Tag::table_presentation())
            .add_attribute("align", "center")
            .add_attribute("width", self.get_outlook_width().to_string());
        let tr = Tag::tr();
        let td = Tag::td()
            .add_style("height", "0")
            .add_style("line-height", "0");

        buf.start_conditional_tag();
        table.render_open(buf);
        tr.render_open(buf);
        td.render_text(buf, "&nbsp;");
        tr.render_close(buf);
        table.render_close(buf);
        buf.end_conditional_tag();
    }
}

impl<'e, 'h> Render<'e, 'h> for MjDividerRender<'e, 'h> {
    fn default_attribute(&self, key: &str) -> Option<&'static str> {
        match key {
            "align" => Some("center"),
            "border-color" => Some("#000000"),
            "border-style" => Some("solid"),
            "border-width" => Some("4px"),
            "padding" => Some("10px 25px"),
            "width" => Some("100%"),
            _ => None,
        }
    }

    fn raw_attribute(&self, key: &str) -> Option<&'e str> {
        self.element.attributes.get(key).map(|v| v.as_str())
    }

    fn tag(&self) -> Option<&str> {
        Some(NAME)
    }

    fn set_container_width(&mut self, width: Option<Pixel>) {
        self.container_width = width;
    }

    fn header(&self) -> &'h Header<'h> {
        self.header
    }

    fn render(
        &self,
        _opts: &RenderOptions,
        _header: &mut VariableHeader,
        buf: &mut RenderBuffer,
    ) -> Result<(), Error> {
        let p = self.set_style_p(Tag::new("p"));
        p.render_text(buf, "");

        self.render_after(buf);
        Ok(())
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MjDivider {
    fn renderer(&'e self, header: &'h Header<'h>) -> Box<dyn Render<'e, 'h> + 'r> {
        Box::new(MjDividerRender::<'e, 'h> {
            element: self,
            header,
            container_width: None,
        })
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
