use super::MJDivider;
use crate::elements::body::prelude::BodyComponent;
use crate::elements::error::Error;
use crate::elements::prelude::*;
use crate::util::attributes::*;
use crate::util::condition::conditional_tag;
use crate::util::context::Context;
use crate::util::header::Header;
use crate::util::size::Size;
use crate::util::tag::Tag;

impl MJDivider {
    fn set_style_p(&self, tag: Tag) -> Tag {
        tag.set_style(
            "border-top",
            format!(
                "{} {} {}",
                self.get_attribute("border-style").unwrap(),
                self.get_attribute("border-width").unwrap(),
                self.get_attribute("border-color").unwrap()
            ),
        )
        .set_style("font-size", "1px")
        .set_style("margin", "0px auto")
        .maybe_set_style("width", self.get_attribute("width"))
    }

    fn set_style_outlook(&self, tag: Tag) -> Tag {
        self.set_style_p(tag)
            .set_style("width", self.get_outlook_width())
    }

    fn get_outlook_width(&self) -> Size {
        let container_width = match self.get_container_width() {
            Some(value) => value,
            None => Size::Percent(100.0),
        };
        let padding_horizontal = self.get_padding_horizontal_width();
        let width = match self.get_size_attribute("width") {
            Some(value) => value,
            None => Size::Percent(100.0),
        };
        match width {
            Size::Percent(value) => {
                Size::Pixel((container_width.value() * value) / 100.0 - padding_horizontal.value())
            }
            Size::Pixel(value) => Size::Pixel(value),
            Size::Raw(_) => Size::Pixel(container_width.value() - padding_horizontal.value()),
        }
    }

    fn render_after(&self) -> String {
        let table = self
            .set_style_outlook(Tag::table_presentation())
            .set_attribute("align", "center")
            .set_attribute("width", self.get_outlook_width());
        let tr = Tag::tr();
        let td = Tag::td().set_style("height", 0).set_style("line-height", 0);
        conditional_tag(table.render(tr.render(td.render("&nbsp;"))))
    }
}

impl Component for MJDivider {
    fn context(&self) -> Option<&Context> {
        self.context.as_ref()
    }

    fn set_context(&mut self, ctx: Context) {
        self.context = Some(ctx);
    }

    fn render(&self, _header: &Header) -> Result<String, Error> {
        Ok(self.set_style_p(Tag::new("p")).render("") + &self.render_after())
    }
}

impl BodyComponent for MJDivider {
    fn attributes(&self) -> Option<&Attributes> {
        Some(&self.attributes)
    }

    fn get_current_width(&self) -> Option<Size> {
        None
    }
    fn set_style(&self, name: &str, tag: Tag) -> Tag {
        match name {
            "p" => self.set_style_p(tag),
            "outlook" => self.set_style_outlook(tag),
            _ => tag,
        }
    }
}
