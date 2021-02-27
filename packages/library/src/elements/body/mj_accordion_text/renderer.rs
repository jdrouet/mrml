use super::MJAccordionText;
use crate::elements::body::prelude::BodyComponent;
use crate::elements::error::Error;
use crate::elements::prelude::Component;
use crate::util::attributes::*;
use crate::util::context::Context;
use crate::util::header::Header;
use crate::util::size::Size;
use crate::util::tag::Tag;

impl MJAccordionText {
    fn render_children(&self, header: &Header) -> Result<String, Error> {
        self.get_children()
            .try_fold(String::default(), |res, child| {
                Ok(res + &child.render(header)?)
            })
    }

    fn render_content(&self, header: &Header) -> Result<String, Error> {
        Ok(Tag::td()
            .maybe_set_class(self.get_attribute("css-class"))
            .maybe_set_style("background", self.get_attribute("background-color"))
            .maybe_set_style("color", self.get_attribute("color"))
            .maybe_set_style("font-family", self.get_attribute("font-family"))
            .maybe_set_style("font-size", self.get_attribute("font-size"))
            .maybe_set_style("line-height", self.get_attribute("line-height"))
            .maybe_set_style("padding", self.get_attribute("padding"))
            .maybe_set_style("padding-top", self.get_attribute("padding-top"))
            .maybe_set_style("padding-right", self.get_attribute("padding-right"))
            .maybe_set_style("padding-bottom", self.get_attribute("padding-bottom"))
            .maybe_set_style("padding-left", self.get_attribute("padding-left"))
            .render(self.render_children(header)?))
    }
}

impl Component for MJAccordionText {
    fn update_header(&self, header: &mut Header) {
        header.maybe_add_font_families(self.get_attribute("font-family"));
    }

    fn context(&self) -> Option<&Context> {
        self.context.as_ref()
    }

    fn set_context(&mut self, ctx: Context) {
        self.context = Some(ctx);
    }

    fn render(&self, header: &Header) -> Result<String, Error> {
        let tr = Tag::tr().render(self.render_content(header)?);
        let tbody = Tag::tbody().render(tr);
        let table = Tag::table()
            .set_attribute("cell-spacing", 0)
            .set_attribute("cell-padding", 0)
            .set_style("width", "100%")
            .maybe_set_style("border-bottom", self.get_attribute("border"))
            .render(tbody);
        let div = Tag::div().set_class("mj-accordion-content").render(table);
        Ok(div)
    }
}

impl BodyComponent for MJAccordionText {
    fn attributes(&self) -> Option<&Attributes> {
        Some(&self.attributes)
    }

    fn get_current_width(&self) -> Option<Size> {
        None
    }
}
