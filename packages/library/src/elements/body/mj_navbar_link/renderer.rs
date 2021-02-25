use super::MJNavbarLink;
use crate::elements::body::prelude::BodyComponent;
use crate::elements::error::Error;
use crate::elements::prelude::*;
use crate::util::attributes::*;
use crate::util::condition::*;
use crate::util::context::Context;
use crate::util::header::Header;
use crate::util::size::Size;
use crate::util::tag::Tag;

impl MJNavbarLink {
    fn set_style_a(&self, tag: Tag) -> Tag {
        tag.set_style("display", "inline-block")
            .maybe_set_style("color", self.get_attribute("color"))
            .maybe_set_style("font-family", self.get_attribute("font-family"))
            .maybe_set_style("font-size", self.get_attribute("font-size"))
            .maybe_set_style("font-style", self.get_attribute("font-style"))
            .maybe_set_style("font-weight", self.get_attribute("font-weight"))
            .maybe_set_style("letter-spacing", self.get_attribute("letter-spacing"))
            .maybe_set_style("line-height", self.get_attribute("line-height"))
            .maybe_set_style("text-decoration", self.get_attribute("text-decoration"))
            .maybe_set_style("text-transform", self.get_attribute("text-transform"))
            .maybe_set_style("padding", self.get_attribute("padding"))
            .maybe_set_style("padding-top", self.get_attribute("padding-top"))
            .maybe_set_style("padding-right", self.get_attribute("padding-right"))
            .maybe_set_style("padding-bottom", self.get_attribute("padding-bottom"))
            .maybe_set_style("padding-left", self.get_attribute("padding-left"))
    }

    fn set_style_td(&self, tag: Tag) -> Tag {
        tag.maybe_set_style("padding", self.get_attribute("padding"))
            .maybe_set_style("padding-top", self.get_attribute("padding-top"))
            .maybe_set_style("padding-right", self.get_attribute("padding-right"))
            .maybe_set_style("padding-bottom", self.get_attribute("padding-bottom"))
            .maybe_set_style("padding-left", self.get_attribute("padding-left"))
    }

    fn get_link(&self) -> Option<String> {
        self.get_attribute("href").as_ref().and_then(|href| {
            self.get_attribute("navbar-base-url")
                .map(move |base| format!("{}{}", base, href))
                .or_else(|| Some(href.to_string()))
        })
    }

    fn render_content(&self, _header: &Header) -> String {
        let link = self
            .set_style_a(Tag::new("a"))
            .set_class("mj-link")
            .maybe_set_class(self.get_attribute("css-class"))
            .maybe_set_attribute("href", self.get_link())
            .maybe_set_attribute("rel", self.get_attribute("rel"))
            .maybe_set_attribute("target", self.get_attribute("target"))
            .maybe_set_attribute("name", self.get_attribute("name"));
        link.render(self.content.as_deref().unwrap_or(""))
    }
}

impl Component for MJNavbarLink {
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
        let td = self
            .set_style_td(Tag::td())
            .maybe_set_class(suffix_css_classes(
                self.get_attribute("css-class"),
                "outlook",
            ));
        let mut res: Vec<String> = vec![];
        res.push(conditional_tag(td.open()));
        res.push(self.render_content(header));
        res.push(conditional_tag(td.close()));
        Ok(res.join(""))
    }
}

impl BodyComponent for MJNavbarLink {
    fn attributes(&self) -> Option<&Attributes> {
        Some(&self.attributes)
    }

    fn get_current_width(&self) -> Option<Size> {
        None
    }

    fn set_style(&self, name: &str, tag: Tag) -> Tag {
        match name {
            "a" => self.set_style_a(tag),
            "td" => self.set_style_td(tag),
            _ => tag,
        }
    }
}
