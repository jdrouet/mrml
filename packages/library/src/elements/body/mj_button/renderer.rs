use super::MJButton;
use crate::elements::body::prelude::{
    as_body_component, BodyComponent, BodyComponentChildIterator,
};
use crate::elements::error::Error;
use crate::elements::prelude::*;
use crate::util::attributes::*;
use crate::util::context::Context;
use crate::util::header::Header;
use crate::util::size::Size;
use crate::util::tag::Tag;

impl MJButton {
    fn get_content(&self, header: &Header) -> Result<String, Error> {
        let mut res = String::from("");
        for item in self.children.iter() {
            res.push_str(item.render(header)?.as_str());
        }
        Ok(res)
    }

    fn set_style_table(&self, tag: Tag) -> Tag {
        tag.set_style("border-collapse", "separate")
            .maybe_set_style("width", self.get_attribute("width"))
            .set_style("line-height", "100%")
    }

    fn set_style_td(&self, tag: Tag) -> Tag {
        tag.maybe_set_style("background", self.get_attribute("background-color"))
            .maybe_set_style("border", self.get_attribute("border"))
            .maybe_set_style("border-top", self.get_attribute("border-top"))
            .maybe_set_style("border-right", self.get_attribute("border-right"))
            .maybe_set_style("border-bottom", self.get_attribute("border-bottom"))
            .maybe_set_style("border-left", self.get_attribute("border-left"))
            .maybe_set_style("border-radius", self.get_attribute("border-radius"))
            .set_style("cursor", "auto")
            .maybe_set_style("font-style", self.get_attribute("font-style"))
            .maybe_set_style("height", self.get_attribute("height"))
            .maybe_set_style("mso-padding-alt", self.get_attribute("inner-padding"))
            .maybe_set_style("text-align", self.get_attribute("text-align"))
    }

    fn set_style_content(&self, tag: Tag) -> Tag {
        tag.set_style("display", "inline-block")
            .maybe_set_style(
                "width",
                self.get_size_attribute("width")
                    .and_then(|value| self.calculate_a_width(Some(value))),
            )
            .maybe_set_style("background", self.get_attribute("background-color"))
            .maybe_set_style("color", self.get_attribute("color"))
            .maybe_set_style("font-family", self.get_attribute("font-family"))
            .maybe_set_style("font-size", self.get_attribute("font-size"))
            .maybe_set_style("font-style", self.get_attribute("font-style"))
            .maybe_set_style("font-weight", self.get_attribute("font-weight"))
            .maybe_set_style("line-height", self.get_attribute("line-height"))
            .maybe_set_style("line-spacing", self.get_attribute("line-spacing"))
            .set_style("margin", "0")
            .maybe_set_style("text-decoration", self.get_attribute("text-decoration"))
            .maybe_set_style("text-transform", self.get_attribute("text-transform"))
            .maybe_set_style("padding", self.get_attribute("inner-padding"))
            .set_style("mso-padding-alt", "0px")
            .maybe_set_style("border-radius", self.get_attribute("border-radius"))
    }

    fn calculate_a_width(&self, width: Option<Size>) -> Option<Size> {
        let width = match width {
            Some(value) => value,
            None => return None,
        };
        if !width.is_pixel() {
            return None;
        }
        let pad_left = match self.get_prefixed_padding_left("inner") {
            Some(value) => value.value(),
            None => 0.0,
        };
        let pad_right = match self.get_prefixed_padding_right("inner") {
            Some(value) => value.value(),
            None => 0.0,
        };

        Some(Size::Pixel(width.value() - pad_left - pad_right))
    }
}

impl Component for MJButton {
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
        let table = self.set_style_table(Tag::table_presentation());
        let tr = Tag::tr();
        let td = self
            .set_style_td(Tag::td())
            .set_attribute("align", "center")
            .maybe_set_attribute("bgcolor", self.get_attribute("background-color"))
            .set_attribute("role", "presentation")
            .maybe_set_attribute("valign", self.get_attribute("vertical-align"));
        let link = Tag::new(match self.get_attribute("href") {
            Some(_) => "a",
            None => "p",
        })
        .maybe_set_attribute("href", self.get_attribute("href"))
        .maybe_set_attribute("rel", self.get_attribute("rel"))
        .maybe_set_attribute("name", self.get_attribute("name"))
        .maybe_set_attribute(
            "target",
            self.get_attribute("href")
                .and_then(|_v| self.get_attribute("target")),
        );
        let link = self.set_style_content(link);

        Ok(table.render(tr.render(td.render(link.render(self.get_content(header)?)))))
    }
}

impl BodyComponent for MJButton {
    fn get_children<'p>(&'p self) -> BodyComponentChildIterator<'p> {
        Box::new(self.children.iter().map(as_body_component))
    }

    fn get_children_len(&self) -> usize {
        self.children.len()
    }

    fn attributes(&self) -> Option<&Attributes> {
        Some(&self.attributes)
    }

    fn get_current_width(&self) -> Option<Size> {
        None
    }

    fn set_style(&self, name: &str, tag: Tag) -> Tag {
        match name {
            "table" => self.set_style_table(tag),
            "td" => self.set_style_td(tag),
            "content" => self.set_style_content(tag),
            _ => tag,
        }
    }
}
