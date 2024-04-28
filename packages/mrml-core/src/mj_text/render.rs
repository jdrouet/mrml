use std::cell::{Ref, RefCell};
use std::rc::Rc;

use super::{MjText, NAME};
use crate::helper::condition::{END_CONDITIONAL_TAG, START_CONDITIONAL_TAG};
use crate::prelude::render::{Error, Header, Render, RenderBuffer, RenderOptions, Renderable, Tag};

struct MjTextRender<'e, 'h> {
    header: Rc<RefCell<Header<'h>>>,
    element: &'e MjText,
}

impl<'e, 'h> MjTextRender<'e, 'h> {
    fn set_style_text(&self, tag: Tag) -> Tag {
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

    fn render_content(&self, opts: &RenderOptions, buf: &mut RenderBuffer) -> Result<(), Error> {
        let root = self.set_style_text(Tag::div());
        root.render_open(buf);
        for child in self.element.children.iter() {
            let renderer = child.renderer(Rc::clone(&self.header));
            renderer.render(opts, buf)?;
        }
        root.render_close(buf);
        Ok(())
    }

    fn render_with_height(
        &self,
        height: &str,
        opts: &RenderOptions,
        buf: &mut RenderBuffer,
    ) -> Result<(), Error> {
        let table = Tag::table_presentation();
        let tr = Tag::tr();
        let td = Tag::td()
            .add_attribute("height", height.to_owned())
            .add_style("vertical-align", "top")
            .add_style("height", height.to_owned());

        buf.push_str(START_CONDITIONAL_TAG);
        table.render_open(buf);
        tr.render_open(buf);
        td.render_open(buf);
        buf.push_str(END_CONDITIONAL_TAG);
        self.render_content(opts, buf)?;
        buf.push_str(START_CONDITIONAL_TAG);
        td.render_close(buf);
        tr.render_close(buf);
        table.render_close(buf);
        buf.push_str(END_CONDITIONAL_TAG);
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

    fn header(&self) -> Ref<Header<'h>> {
        self.header.borrow()
    }

    fn render(&self, opts: &RenderOptions, buf: &mut RenderBuffer) -> Result<(), Error> {
        let font_family = self.attribute("font-family");
        self.header
            .borrow_mut()
            .maybe_add_font_families(font_family);
        if let Some(ref height) = self.attribute("height") {
            self.render_with_height(height, opts, buf)
        } else {
            self.render_content(opts, buf)
        }
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MjText {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'e, 'h> + 'r> {
        Box::new(MjTextRender::<'e, 'h> {
            element: self,
            header,
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
