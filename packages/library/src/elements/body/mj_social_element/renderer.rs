use super::MJSocialElement;
use crate::elements::body::prelude::*;
use crate::elements::body::BodyElement;
use crate::elements::error::Error;
use crate::elements::prelude::*;
use crate::util::attributes::*;
use crate::util::context::Context;
use crate::util::header::Header;
use crate::util::size::Size;
use crate::util::tag::Tag;

impl MJSocialElement {
    fn get_background_color(&self) -> Option<String> {
        if let Some(bg) = self.get_attribute("background-color") {
            return Some(bg.to_string());
        }
        self.social_network
            .as_ref()
            .map(|net| net.background_color.clone())
    }

    fn get_icon_size(&self) -> Option<Size> {
        self.get_size_attribute("icon-size")
    }

    fn get_icon_height(&self) -> Option<Size> {
        self.get_size_attribute("icon-height")
    }

    fn get_icon_src(&self) -> Option<String> {
        if let Some(src) = self.get_attribute("src") {
            return Some(src.to_string());
        }
        self.social_network.as_ref().map(|net| net.src.clone())
    }

    fn set_style_img(&self, tag: Tag) -> Tag {
        tag.set_style("display", "block")
            .maybe_set_style("border-radius", self.get_attribute("border-radius"))
    }

    fn set_style_icon(&self, tag: Tag) -> Tag {
        tag.maybe_set_style("padding", self.get_attribute("icon-padding"))
            .set_style("font-size", "0")
            .maybe_set_style(
                "height",
                self.get_icon_height().or_else(|| self.get_icon_size()),
            )
            .set_style("vertical-align", "middle")
            .maybe_set_style("width", self.get_icon_size())
    }

    fn set_style_table(&self, tag: Tag) -> Tag {
        tag.maybe_set_style("background", self.get_background_color())
            .maybe_set_style("border-radius", self.get_attribute("border-radius"))
            .maybe_set_style("width", self.get_icon_size())
    }

    fn set_style_td(&self, tag: Tag) -> Tag {
        tag.maybe_set_style("padding", self.get_attribute("padding"))
            .maybe_set_style("vertical-align", self.get_attribute("vertical-align"))
    }

    fn set_style_td_text(&self, tag: Tag) -> Tag {
        tag.maybe_set_style("padding", self.get_attribute("text-padding"))
            .set_style("vertical-align", "middle")
    }

    fn set_style_text(&self, tag: Tag) -> Tag {
        tag.maybe_set_style("color", self.get_attribute("color"))
            .maybe_set_style("font-size", self.get_attribute("font-size"))
            .maybe_set_style("font-weight", self.get_attribute("font-weight"))
            .maybe_set_style("font-style", self.get_attribute("font-style"))
            .maybe_set_style("font-family", self.get_attribute("font-family"))
            .maybe_set_style("line-height", self.get_attribute("line-height"))
            .maybe_set_style("text-decoration", self.get_attribute("text-decoration"))
    }

    fn get_href(&self) -> Option<String> {
        self.get_attribute("href")
            .map(|href| {
                self.social_network
                    .as_ref()
                    .and_then(|net| net.share_url.clone())
                    .map(move |url| url.replace("[[URL]]", href))
                    .or_else(move || Some(href.to_string()))
            })
            .unwrap_or_default()
    }

    fn render_icon(&self, href: &Option<String>) -> String {
        let table = self.set_style_table(Tag::table_presentation());
        let tr = Tag::tr();
        let td = self.set_style_icon(Tag::td());
        let a = Tag::new("a")
            .maybe_set_attribute("href", href.as_ref())
            .maybe_set_attribute("rel", self.get_attribute("rel"))
            .maybe_set_attribute("target", self.get_attribute("target"));
        let img = self
            .set_style_img(Tag::new("img"))
            .maybe_set_attribute("alt", self.get_attribute("alt"))
            .maybe_set_attribute("title", self.get_attribute("title"))
            .maybe_set_attribute(
                "height",
                self.get_icon_height()
                    .or_else(|| self.get_icon_size())
                    .map(|size| size.value()),
            )
            .maybe_set_attribute("src", self.get_icon_src())
            .maybe_set_attribute("width", self.get_icon_size().map(|size| size.value()));

        table.render(tr.render(td.render(if href.is_some() {
            a.render(img.closed())
        } else {
            img.closed()
        })))
    }

    fn render_text(&self, href: &Option<String>) -> String {
        let td = self.set_style_td_text(Tag::td());
        let wrapper = if href.is_some() {
            Tag::new("a")
                .maybe_set_attribute("href", href.as_ref())
                .maybe_set_attribute("rel", self.get_attribute("rel"))
                .maybe_set_attribute("target", self.get_attribute("target"))
        } else {
            Tag::new("span")
        };
        let wrapper = self.set_style_text(wrapper);
        td.render(wrapper.render(self.content.as_deref().unwrap_or("")))
    }
}

impl Component for MJSocialElement {
    fn update_header(&self, header: &mut Header) {
        header.maybe_add_font_families(self.get_attribute("font-family"));
    }

    fn context(&self) -> Option<&Context> {
        self.context.as_ref()
    }

    fn set_context(&mut self, ctx: Context) {
        self.context = Some(ctx);
    }

    fn render(&self, _header: &Header) -> Result<String, Error> {
        let href = self.get_href();
        let tr = Tag::tr().maybe_set_class(self.get_attribute("css-class"));
        let td = self.set_style_td(Tag::td());

        let mut res = vec![];
        res.push(tr.open());
        res.push(td.render(self.render_icon(&href)));
        if self.content.is_some() {
            res.push(self.render_text(&href));
        }
        res.push(tr.close());
        Ok(res.join(""))
    }
}

impl BodyComponent for MJSocialElement {
    fn attributes(&self) -> Option<&Attributes> {
        Some(&self.attributes)
    }

    fn get_children(&self) -> &Vec<BodyElement> {
        &EMPTY_CHILDREN
    }

    fn get_current_width(&self) -> Option<Size> {
        None
    }

    fn set_style(&self, name: &str, tag: Tag) -> Tag {
        match name {
            "table" => self.set_style_table(tag),
            "td" => self.set_style_td(tag),
            "icon" => self.set_style_icon(tag),
            "img" => self.set_style_img(tag),
            "td-text" => self.set_style_td_text(tag),
            "text" => self.set_style_text(tag),
            _ => tag,
        }
    }
}
