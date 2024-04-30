use super::{MjText, NAME};
use crate::prelude::render::*;

struct MjTextRender<'e, 'h> {
    context: &'h RenderContext<'h>,
    element: &'e MjText,
}

impl<'e, 'h> MjTextRender<'e, 'h> {
    fn set_style_text<'a>(&self, tag: Tag<'a>) -> Tag<'a> {
        tag.maybe_add_style("font-family", self.attribute("font-family"))
            .maybe_add_style("font-size", self.attribute("font-size"))
            .maybe_add_style("font-style", self.attribute("font-style"))
            .maybe_add_style("font-weight", self.attribute("font-weight"))
            .maybe_add_style("letter-spacing", self.attribute("letter-spacing"))
            .maybe_add_style("line-height", self.attribute("line-height"))
            .maybe_add_style("text-align", self.attribute("align"))
            .maybe_add_style("text-decoration", self.attribute("text-decoration"))
            .maybe_add_style("text-transform", self.attribute("text-transform"))
            .maybe_add_style("color", self.attribute("color"))
            .maybe_add_style("height", self.attribute("height"))
    }

    fn render_content(
        &self,
        header: &mut VariableHeader,
        buf: &mut RenderBuffer,
    ) -> Result<(), Error> {
        let root = self.set_style_text(Tag::div());
        root.render_open(buf);
        for child in self.element.children.iter() {
            child.renderer(self.context()).render(header, buf)?;
        }
        root.render_close(buf);
        Ok(())
    }

    fn render_with_height(
        &self,
        height: &str,
        header: &mut VariableHeader,
        buf: &mut RenderBuffer,
    ) -> Result<(), Error> {
        let table = Tag::table_presentation();
        let tr = Tag::tr();
        let td = Tag::td()
            .add_attribute("height", height.to_owned())
            .add_style("vertical-align", "top")
            .add_style("height", height.to_owned());

        buf.start_conditional_tag();
        table.render_open(buf);
        tr.render_open(buf);
        td.render_open(buf);
        buf.end_conditional_tag();
        self.render_content(header, buf)?;
        buf.start_conditional_tag();
        td.render_close(buf);
        tr.render_close(buf);
        table.render_close(buf);
        buf.end_conditional_tag();
        Ok(())
    }
}

impl<'e, 'h> Render<'e, 'h> for MjTextRender<'e, 'h> {
    fn default_attribute(&self, key: &str) -> Option<&'static str> {
        match key {
            "align" => Some("left"),
            "color" => Some("#000000"),
            "font-family" => Some("Ubuntu, Helvetica, Arial, sans-serif"),
            "font-size" => Some("13px"),
            "line-height" => Some("1"),
            "padding" => Some("10px 25px"),
            _ => None,
        }
    }

    fn raw_attribute(&self, key: &str) -> Option<&'e str> {
        self.element.attributes.get(key).map(|v| v.as_str())
    }

    fn tag(&self) -> Option<&str> {
        Some(NAME)
    }

    fn context(&self) -> &'h RenderContext<'h> {
        &self.context
    }

    fn render(&self, header: &mut VariableHeader, buf: &mut RenderBuffer) -> Result<(), Error> {
        let font_family = self.attribute("font-family");
        header.maybe_add_font_families(font_family);

        if let Some(ref height) = self.attribute("height") {
            self.render_with_height(height, header, buf)
        } else {
            self.render_content(header, buf)
        }
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MjText {
    fn renderer(&'e self, context: &'h RenderContext<'h>) -> Box<dyn Render<'e, 'h> + 'r> {
        Box::new(MjTextRender::<'e, 'h> {
            element: self,
            context,
        })
    }
}

#[cfg(test)]
mod tests {
    crate::should_render!(basic, "mj-text");
    crate::should_render!(align, "mj-text-align");
    crate::should_render!(class, "mj-text-class");
    crate::should_render!(color, "mj-text-color");
    crate::should_render!(
        container_background_color,
        "mj-text-container-background-color"
    );
    crate::should_render!(example, "mj-text-example");
    crate::should_render!(font_family, "mj-text-font-family");
    crate::should_render!(font_size, "mj-text-font-size");
    crate::should_render!(font_style, "mj-text-font-style");
    crate::should_render!(font_weight, "mj-text-font-weight");
    crate::should_render!(height, "mj-text-height");
    crate::should_render!(line_height, "mj-text-line-height");
    crate::should_render!(padding, "mj-text-padding");
}
