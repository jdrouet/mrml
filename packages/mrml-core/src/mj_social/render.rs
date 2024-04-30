use super::{MjSocial, MjSocialChild, NAME};
use crate::helper::size::{Pixel, Size};
use crate::prelude::render::*;

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MjSocialChild {
    fn renderer(&'e self, context: &'h RenderContext<'h>) -> Box<dyn Render<'e, 'h> + 'r> {
        match self {
            Self::MjSocialElement(elt) => elt.renderer(context),
            Self::Comment(elt) => elt.renderer(context),
        }
    }
}

const EXTRA_CONTAINER_KEY: [&str; 13] = [
    "inner-padding",
    "border-radius",
    "color",
    "font-family",
    "font-size",
    "font-weight",
    "font-style",
    "icon-size",
    "icon-height",
    "icon-padding",
    "text-padding",
    "line-height",
    "text-decoration",
];
const EXTRA_CHILD_KEY: [&str; 13] = [
    "padding",
    "border-radius",
    "color",
    "font-family",
    "font-size",
    "font-weight",
    "font-style",
    "icon-size",
    "icon-height",
    "icon-padding",
    "text-padding",
    "line-height",
    "text-decoration",
];

struct MjSocialRender<'e, 'h> {
    context: &'h RenderContext<'h>,
    element: &'e MjSocial,
    container_width: Option<Pixel>,
    siblings: usize,
    raw_siblings: usize,
}

impl<'e, 'h> MjSocialRender<'e, 'h> {
    fn set_style_table_vertical<'a>(&self, tag: Tag<'a>) -> Tag<'a> {
        tag.add_style("margin", "0px")
    }

    fn is_horizontal(&self) -> bool {
        self.attribute("mode")
            .map(|mode| mode == "horizontal")
            .unwrap_or(true)
    }

    fn build_child_attributes(&self) -> Vec<(&str, String)> {
        EXTRA_CONTAINER_KEY
            .iter()
            .zip(EXTRA_CHILD_KEY.iter())
            .filter_map(|(con_key, child_key)| {
                self.attribute(con_key).map(|value| (*child_key, value))
            })
            .collect::<Vec<_>>()
    }

    fn render_horizontal(
        &self,
        header: &mut VariableHeader,
        buf: &mut RenderBuffer,
    ) -> Result<(), Error> {
        let table = Tag::table_presentation().maybe_add_attribute("align", self.attribute("align"));
        let tr = Tag::tr();
        let td = Tag::td();
        let inner_table = Tag::table_presentation()
            .maybe_add_attribute("align", self.attribute("align"))
            .add_style("float", "none")
            .add_style("display", "inline-table");
        let inner_tbody = Tag::tbody();
        let child_attributes = self.build_child_attributes();

        buf.start_conditional_tag();
        table.render_open(buf);
        tr.render_open(buf);
        buf.end_conditional_tag();

        for (index, child) in self.element.children.iter().enumerate() {
            buf.start_conditional_tag();
            td.render_open(buf);
            buf.end_conditional_tag();
            inner_table.render_open(buf);
            inner_tbody.render_open(buf);
            let mut renderer = child.renderer(self.context());
            renderer.set_index(index);
            child_attributes.iter().for_each(|(key, value)| {
                renderer.add_extra_attribute(key, value);
            });
            renderer.render(header, buf)?;
            inner_tbody.render_close(buf);
            inner_table.render_close(buf);
            buf.start_conditional_tag();
            td.render_close(buf);
            buf.end_conditional_tag();
        }

        buf.start_conditional_tag();
        tr.render_close(buf);
        table.render_close(buf);
        buf.end_conditional_tag();
        Ok(())
    }

    fn render_vertical(
        &self,
        header: &mut VariableHeader,
        buf: &mut RenderBuffer,
    ) -> Result<(), Error> {
        let table = self.set_style_table_vertical(Tag::table_presentation());
        let tbody = Tag::tbody();
        let child_attributes = self.build_child_attributes();

        table.render_open(buf);
        tbody.render_open(buf);
        for (index, child) in self.element.children.iter().enumerate() {
            let mut renderer = child.renderer(self.context());
            renderer.set_index(index);
            child_attributes.iter().for_each(|(key, value)| {
                renderer.add_extra_attribute(key, value);
            });
            renderer.render(header, buf)?;
        }
        tbody.render_close(buf);
        table.render_close(buf);

        Ok(())
    }
}

impl<'e, 'h> Render<'e, 'h> for MjSocialRender<'e, 'h> {
    fn default_attribute(&self, name: &str) -> Option<&'static str> {
        match name {
            "align" => Some("center"),
            "border-radius" => Some("3px"),
            "color" => Some("#333333"),
            "font-family" => Some("Ubuntu, Helvetica, Arial, sans-serif"),
            "font-size" => Some("13px"),
            "icon-size" => Some("20px"),
            "line-height" => Some("22px"),
            "mode" => Some("horizontal"),
            "padding" => Some("10px 25px"),
            "text-decoration" => Some("none"),
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

    fn get_width(&self) -> Option<Size> {
        self.container_width
            .as_ref()
            .map(|w| Size::Pixel(w.clone()))
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

    fn render(&self, header: &mut VariableHeader, buf: &mut RenderBuffer) -> Result<(), Error> {
        let font_families = self.attribute("font-family").unwrap_or_default(); // never happens
        header.add_font_families(font_families);

        if self.is_horizontal() {
            self.render_horizontal(header, buf)
        } else {
            self.render_vertical(header, buf)
        }
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MjSocial {
    fn renderer(&'e self, context: &'h RenderContext<'h>) -> Box<dyn Render<'e, 'h> + 'r> {
        Box::new(MjSocialRender::<'e, 'h> {
            element: self,
            context,
            container_width: None,
            siblings: 1,
            raw_siblings: 0,
        })
    }
}

#[cfg(test)]
mod tests {
    crate::should_render!(basic, "mj-social");
    crate::should_render!(align, "mj-social-align");
    crate::should_render!(border_radius, "mj-social-border-radius");
    crate::should_render!(class, "mj-social-class");
    crate::should_render!(color, "mj-social-color");
    crate::should_render!(
        container_background_color,
        "mj-social-container-background-color"
    );
    crate::should_render!(font_family, "mj-social-font-family");
    crate::should_render!(font, "mj-social-font");
    crate::should_render!(icon, "mj-social-icon");
    crate::should_render!(link, "mj-social-link");
    crate::should_render!(mode, "mj-social-mode");
    crate::should_render!(padding, "mj-social-padding");
    crate::should_render!(text, "mj-social-text");
}
