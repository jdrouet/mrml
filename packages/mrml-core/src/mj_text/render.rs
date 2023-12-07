use std::cell::{Ref, RefCell};
use std::rc::Rc;

use super::{MjText, NAME};
use crate::helper::condition::conditional_tag;
use crate::helper::tag::Tag;
use crate::prelude::hash::Map;
use crate::prelude::render::{Error, Header, Render, RenderOptions, Renderable};

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

    fn render_content(&self, opts: &RenderOptions) -> Result<String, Error> {
        let res = self
            .element
            .children
            .iter()
            .try_fold(String::default(), |res, child| {
                let renderer = child.renderer(Rc::clone(&self.header));
                Ok(res + &renderer.render(opts)?)
            })?;
        Ok(self.set_style_text(Tag::div()).render(res))
    }

    fn render_with_height(&self, height: &str, opts: &RenderOptions) -> Result<String, Error> {
        let table = Tag::table_presentation();
        let tr = Tag::tr();
        let td = Tag::td()
            .add_attribute("height", height.to_owned())
            .add_style("vertical-align", "top")
            .add_style("height", height.to_owned());
        Ok(conditional_tag(table.open() + &tr.open() + &td.open())
            + &self.render_content(opts)?
            + &conditional_tag(td.close() + &tr.close() + &table.close()))
    }
}

impl<'e, 'h> Render<'h> for MjTextRender<'e, 'h> {
    fn default_attribute(&self, key: &str) -> Option<&str> {
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

    fn attributes(&self) -> Option<&Map<String, String>> {
        Some(&self.element.attributes)
    }

    fn tag(&self) -> Option<&str> {
        Some(NAME)
    }

    fn header(&self) -> Ref<Header<'h>> {
        self.header.borrow()
    }

    fn render(&self, opts: &RenderOptions) -> Result<String, Error> {
        let font_family = self.attribute("font-family");
        self.header
            .borrow_mut()
            .maybe_add_font_families(font_family);
        if let Some(ref height) = self.attribute("height") {
            self.render_with_height(height, opts)
        } else {
            self.render_content(opts)
        }
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MjText {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'h> + 'r> {
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
