use super::{MjAccordion, MjAccordionChild, NAME};
use crate::helper::size::{Pixel, Size};
use crate::prelude::render::*;

const CHILDREN_ATTRIBUTES: [&str; 9] = [
    "border",
    "icon-align",
    "icon-width",
    "icon-height",
    "icon-position",
    "icon-wrapped-url",
    "icon-wrapped-alt",
    "icon-unwrapped-url",
    "icon-unwrapped-alt",
];

const STYLE: &str = r#"noinput.mj-accordion-checkbox { display: block! important; }
@media yahoo, only screen and (min-width:0) {
  .mj-accordion-element { display:block; }
  input.mj-accordion-checkbox, .mj-accordion-less { display: none !important; }
  input.mj-accordion-checkbox+* .mj-accordion-title { cursor: pointer; touch-action: manipulation; -webkit-user-select: none; -moz-user-select: none; user-select: none; }
  input.mj-accordion-checkbox+* .mj-accordion-content { overflow: hidden; display: none; }
  input.mj-accordion-checkbox+* .mj-accordion-more { display: block !important; }
  input.mj-accordion-checkbox:checked+* .mj-accordion-content { display: block; }
  input.mj-accordion-checkbox:checked+* .mj-accordion-more { display: none !important; }
  input.mj-accordion-checkbox:checked+* .mj-accordion-less { display: block !important; }
}
.moz-text-html input.mj-accordion-checkbox+* .mj-accordion-title { cursor: auto; touch-action: auto; -webkit-user-select: auto; -moz-user-select: auto; user-select: auto; }
.moz-text-html input.mj-accordion-checkbox+* .mj-accordion-content { overflow: hidden; display: block; }
.moz-text-html input.mj-accordion-checkbox+* .mj-accordion-ico { display: none; }
@goodbye { @gmail }
"#;

impl<'root> Renderer<'root, MjAccordion, ()> {
    fn update_header(&self, header: &mut VariableHeader) {
        let font_families = self.attribute("font-family");
        header.maybe_add_font_families(font_families);
        header.add_style(STYLE);
    }
}

impl<'root> Render<'root> for Renderer<'root, MjAccordion, ()> {
    fn default_attribute(&self, name: &str) -> Option<&'static str> {
        match name {
            "border" => Some("2px solid black"),
            "font-family" => Some("Ubuntu, Helvetica, Arial, sans-serif"),
            "icon-align" => Some("middle"),
            "icon-position" => Some("right"),
            "icon-height" => Some("32px"),
            "icon-width" => Some("32px"),
            "icon-wrapped-url" => Some("https://i.imgur.com/bIXv1bk.png"),
            "icon-wrapped-alt" => Some("+"),
            "icon-unwrapped-url" => Some("https://i.imgur.com/w4uTygT.png"),
            "icon-unwrapped-alt" => Some("-"),
            "padding" => Some("10px 25px"),
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

    fn render(&self, cursor: &mut RenderCursor) -> Result<(), Error> {
        self.update_header(&mut cursor.header);

        let tbody = Tag::tbody();
        let table = Tag::table()
            .add_style("width", "100%")
            .add_style("border-collapse", "collapse")
            .maybe_add_style("border", self.attribute("border"))
            .add_style("border-bottom", "none")
            .maybe_add_style("font-family", self.attribute("font-family"))
            .add_attribute("cellspacing", "0")
            .add_attribute("cellpadding", "0")
            .add_class("mj-accordion");

        table.render_open(&mut cursor.buffer)?;
        tbody.render_open(&mut cursor.buffer)?;

        let children_attrs = CHILDREN_ATTRIBUTES
            .iter()
            .copied()
            .filter_map(|key| self.attribute(key).map(|found| (key, found)))
            .collect::<Vec<_>>();

        for child in self.element.children.iter() {
            let mut renderer = child.renderer(self.context());
            children_attrs.iter().copied().for_each(|(key, value)| {
                renderer.add_extra_attribute(key, value);
            });
            renderer.render(cursor)?;
        }
        tbody.render_close(&mut cursor.buffer);
        table.render_close(&mut cursor.buffer);
        Ok(())
    }
}

impl<'render, 'root: 'render> Renderable<'render, 'root> for MjAccordion {
    fn renderer(
        &'root self,
        context: &'root RenderContext<'root>,
    ) -> Box<dyn Render<'root> + 'render> {
        Box::new(Renderer::new(context, self, ()))
    }
}

impl<'render, 'root: 'render> Renderable<'render, 'root> for MjAccordionChild {
    fn renderer(
        &'root self,
        context: &'root RenderContext<'root>,
    ) -> Box<dyn Render<'root> + 'render> {
        match self {
            Self::MjAccordionElement(elt) => elt.renderer(context),
            Self::Comment(elt) => elt.renderer(context),
        }
    }
}

#[cfg(test)]
mod tests {
    crate::should_render!(basic, "mj-accordion");
    crate::should_render!(font_padding, "mj-accordion-font-padding");
    crate::should_render!(icon, "mj-accordion-icon");
    crate::should_render!(other, "mj-accordion-other");
}
