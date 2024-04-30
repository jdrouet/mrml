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

struct MjAccordionRender<'e, 'h> {
    header: &'h Header<'h>,
    element: &'e MjAccordion,
    container_width: Option<Pixel>,
    siblings: usize,
    raw_siblings: usize,
}

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

impl<'e, 'h> MjAccordionRender<'e, 'h> {
    fn update_header(&self, header: &mut VariableHeader) {
        let font_families = self.attribute("font-family");
        header.maybe_add_font_families(font_families);
        header.add_style(STYLE);
    }
}

impl<'e, 'h> Render<'e, 'h> for MjAccordionRender<'e, 'h> {
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

    fn raw_attribute(&self, key: &str) -> Option<&'e str> {
        self.element.attributes.get(key).map(|v| v.as_str())
    }

    fn tag(&self) -> Option<&str> {
        Some(NAME)
    }

    fn header(&self) -> &'h Header<'h> {
        self.header
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

    fn render(
        &self,
        opts: &RenderOptions,
        header: &mut VariableHeader,
        buf: &mut RenderBuffer,
    ) -> Result<(), Error> {
        self.update_header(header);

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

        table.render_open(buf);
        tbody.render_open(buf);
        for child in self.element.children.iter() {
            let mut renderer = child.renderer(self.header);
            CHILDREN_ATTRIBUTES.iter().for_each(|key| {
                renderer.maybe_add_extra_attribute(key, self.attribute(key));
            });
            renderer.render(opts, header, buf)?;
        }
        tbody.render_close(buf);
        table.render_close(buf);
        Ok(())
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MjAccordion {
    fn renderer(&'e self, header: &'h Header<'h>) -> Box<dyn Render<'e, 'h> + 'r> {
        Box::new(MjAccordionRender::<'e, 'h> {
            element: self,
            header,
            container_width: None,
            siblings: 1,
            raw_siblings: 0,
        })
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MjAccordionChild {
    fn renderer(&'e self, header: &'h Header<'h>) -> Box<dyn Render<'e, 'h> + 'r> {
        match self {
            Self::MjAccordionElement(elt) => elt.renderer(header),
            Self::Comment(elt) => elt.renderer(header),
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
