use super::{MjWrapper, NAME};
use crate::helper::size::Pixel;
use crate::mj_section::{SectionLikeRender, WithMjSectionBackground};
use crate::prelude::render::*;

#[derive(Default)]
struct MjWrapperExtra {
    container_width: Option<Pixel>,
}

impl<'element, 'header> Renderer<'element, 'header, MjWrapper, MjWrapperExtra> {
    fn current_width(&self) -> Option<Pixel> {
        self.extra.container_width.as_ref().map(|width| {
            let hborder = self.get_border_horizontal();
            let hpadding = self.get_padding_horizontal();
            Pixel::new(width.value() - hborder.value() - hpadding.value())
        })
    }
}

impl<'element, 'header> WithMjSectionBackground<'element, 'header>
    for Renderer<'element, 'header, MjWrapper, MjWrapperExtra>
{
}

impl<'element, 'header> SectionLikeRender<'element, 'header>
    for Renderer<'element, 'header, MjWrapper, MjWrapperExtra>
{
    fn children(&self) -> &Vec<crate::mj_body::MjBodyChild> {
        &self.element.children
    }

    fn container_width(&self) -> &Option<Pixel> {
        &self.extra.container_width
    }

    fn render_wrapped_children(&self, cursor: &mut RenderCursor) -> Result<(), Error> {
        let tr = Tag::tr();
        let siblings = self.get_siblings();
        let raw_siblings = self.get_raw_siblings();
        let current_width = self.current_width();
        let container_width = self.extra.container_width.as_ref().map(|v| v.to_string());
        for child in self.children().iter() {
            let mut renderer = child.renderer(self.context());
            renderer.set_siblings(siblings);
            renderer.set_raw_siblings(raw_siblings);
            renderer.set_container_width(current_width.clone());
            if child.is_raw() {
                renderer.render(cursor)?;
            } else {
                let td = renderer
                    .set_style("td-outlook", Tag::td())
                    .maybe_add_attribute("align", renderer.attribute("align"))
                    .maybe_add_attribute("width", container_width.as_ref().cloned())
                    .maybe_add_suffixed_class(renderer.attribute("css-class"), "outlook");
                tr.render_open(&mut cursor.buffer);
                td.render_open(&mut cursor.buffer);
                cursor.buffer.end_conditional_tag();
                renderer.render(cursor)?;
                cursor.buffer.start_conditional_tag();
                td.render_close(&mut cursor.buffer);
                tr.render_close(&mut cursor.buffer);
            }
        }
        Ok(())
    }
}

impl<'element, 'header> Render<'element, 'header>
    for Renderer<'element, 'header, MjWrapper, MjWrapperExtra>
{
    fn default_attribute(&self, name: &str) -> Option<&'static str> {
        match name {
            "background-position" => Some("top center"),
            "background-repeat" => Some("repeat"),
            "background-size" => Some("auto"),
            "direction" => Some("ltr"),
            "padding" => Some("20px 0"),
            "text-align" => Some("center"),
            "text-padding" => Some("4px 4px 4px 0"),
            _ => None,
        }
    }

    fn context(&self) -> &'header RenderContext<'header> {
        self.context
    }

    fn raw_attribute(&self, key: &str) -> Option<&'element str> {
        self.element.attributes.get(key).map(|v| v.as_str())
    }

    fn tag(&self) -> Option<&str> {
        Some(NAME)
    }

    fn set_container_width(&mut self, width: Option<Pixel>) {
        self.extra.container_width = width;
    }

    fn render(&self, cursor: &mut RenderCursor) -> Result<(), Error> {
        if self.is_full_width() {
            self.render_full_width(cursor)
        } else {
            self.render_simple(cursor)
        }
    }
}

struct MjWrapperRender<'e, 'h> {
    context: &'h RenderContext<'h>,
    element: &'e MjWrapper,
    container_width: Option<Pixel>,
}

impl<'e, 'h> MjWrapperRender<'e, 'h> {
    fn current_width(&self) -> Option<Pixel> {
        self.container_width.as_ref().map(|width| {
            let hborder = self.get_border_horizontal();
            let hpadding = self.get_padding_horizontal();
            Pixel::new(width.value() - hborder.value() - hpadding.value())
        })
    }
}

impl<'e, 'h> WithMjSectionBackground<'e, 'h> for MjWrapperRender<'e, 'h> {}

impl<'e, 'h> SectionLikeRender<'e, 'h> for MjWrapperRender<'e, 'h> {
    fn children(&self) -> &Vec<crate::mj_body::MjBodyChild> {
        &self.element.children
    }

    fn container_width(&self) -> &Option<Pixel> {
        &self.container_width
    }

    fn render_wrapped_children(&self, cursor: &mut RenderCursor) -> Result<(), Error> {
        let tr = Tag::tr();
        let siblings = self.get_siblings();
        let raw_siblings = self.get_raw_siblings();
        let current_width = self.current_width();
        let container_width = self.container_width.as_ref().map(|v| v.to_string());
        for child in self.children().iter() {
            let mut renderer = child.renderer(self.context());
            renderer.set_siblings(siblings);
            renderer.set_raw_siblings(raw_siblings);
            renderer.set_container_width(current_width.clone());
            if child.is_raw() {
                renderer.render(cursor)?;
            } else {
                let td = renderer
                    .set_style("td-outlook", Tag::td())
                    .maybe_add_attribute("align", renderer.attribute("align"))
                    .maybe_add_attribute("width", container_width.as_ref().cloned())
                    .maybe_add_suffixed_class(renderer.attribute("css-class"), "outlook");
                tr.render_open(&mut cursor.buffer);
                td.render_open(&mut cursor.buffer);
                cursor.buffer.end_conditional_tag();
                renderer.render(cursor)?;
                cursor.buffer.start_conditional_tag();
                td.render_close(&mut cursor.buffer);
                tr.render_close(&mut cursor.buffer);
            }
        }
        Ok(())
    }
}

impl<'r, 'e: 'r, 'h: 'r> Render<'e, 'h> for MjWrapperRender<'e, 'h> {
    fn default_attribute(&self, name: &str) -> Option<&'static str> {
        match name {
            "background-position" => Some("top center"),
            "background-repeat" => Some("repeat"),
            "background-size" => Some("auto"),
            "direction" => Some("ltr"),
            "padding" => Some("20px 0"),
            "text-align" => Some("center"),
            "text-padding" => Some("4px 4px 4px 0"),
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
        self.context
    }

    fn set_container_width(&mut self, width: Option<Pixel>) {
        self.container_width = width;
    }

    fn render(&self, cursor: &mut RenderCursor) -> Result<(), Error> {
        if self.is_full_width() {
            self.render_full_width(cursor)
        } else {
            self.render_simple(cursor)
        }
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MjWrapper {
    fn renderer(&'e self, context: &'h RenderContext<'h>) -> Box<dyn Render<'e, 'h> + 'r> {
        Box::new(Renderer::new(context, self, MjWrapperExtra::default()))
    }
}

#[cfg(test)]
mod tests {
    crate::should_render!(basic, "mj-wrapper");
    crate::should_render!(background, "mj-wrapper-background");
    crate::should_render!(border, "mj-wrapper-border");
    crate::should_render!(other, "mj-wrapper-other");
    crate::should_render!(padding, "mj-wrapper-padding");
}
