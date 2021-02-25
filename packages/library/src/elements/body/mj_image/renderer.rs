use super::MJImage;
use crate::elements::body::prelude::*;
use crate::elements::error::Error;
use crate::elements::prelude::*;
use crate::util::attributes::*;
use crate::util::context::Context;
use crate::util::header::Header;
use crate::util::size::Size;
use crate::util::tag::Tag;

impl MJImage {
    fn is_fluid_on_mobile(&self) -> bool {
        self.get_attribute("fluid-on-mobile")
            .and_then(|value| value.parse::<bool>().ok())
            .unwrap_or(false)
    }

    fn is_full_width(&self) -> bool {
        self.get_attribute("full-width").is_some()
    }

    fn get_content_width(&self) -> Option<Size> {
        self.get_size_attribute("width")
            .map(|width| match self.get_box_widths() {
                Some(box_size) => {
                    if width.value() < box_size.value() {
                        width
                    } else {
                        box_size
                    }
                }
                None => width,
            })
            // when no width given
            .or_else(|| self.get_box_widths())
    }

    fn set_style_img(&self, tag: Tag) -> Tag {
        let tag = tag
            .maybe_set_style("border", self.get_attribute("border"))
            .maybe_set_style("border-left", self.get_attribute("left"))
            .maybe_set_style("border-right", self.get_attribute("right"))
            .maybe_set_style("border-top", self.get_attribute("top"))
            .maybe_set_style("border-bottom", self.get_attribute("bottom"))
            .maybe_set_style("border-radius", self.get_attribute("border-radius"))
            .set_style("display", "block")
            .set_style("outline", "none")
            .set_style("text-decoration", "none")
            .maybe_set_style("height", self.get_attribute("height"))
            .maybe_set_style("max-height", self.get_attribute("max-height"))
            .set_style("width", "100%");
        let tag = if self.is_full_width() {
            tag.set_style("min-width", "100%")
                .set_style("max-width", "100%")
        } else {
            tag
        };
        tag.maybe_set_style("font-size", self.get_attribute("font-size"))
    }

    fn set_style_td(&self, tag: Tag) -> Tag {
        if self.is_full_width() {
            tag
        } else {
            tag.maybe_set_style("width", self.get_content_width())
        }
    }

    fn set_style_table(&self, tag: Tag) -> Tag {
        let tag = if self.is_full_width() {
            tag.set_style("min-width", "100%")
                .set_style("max-width", "100%")
                .maybe_set_style("width", self.get_content_width())
        } else {
            tag
        };
        tag.set_style("border-collapse", "collapse")
            .set_style("border-spacing", "0px")
    }

    fn render_image(&self) -> String {
        let img = Tag::new("img")
            .maybe_set_attribute("alt", self.get_attribute("alt"))
            .set_attribute(
                "height",
                self.get_size_attribute("height")
                    .map(|size| size.value().to_string())
                    .unwrap_or_else(|| "auto".into()),
            )
            .maybe_set_attribute("src", self.get_attribute("src"))
            .maybe_set_attribute("srcset", self.get_attribute("srcset"))
            .maybe_set_attribute("title", self.get_attribute("title"))
            .maybe_set_attribute("width", self.get_content_width().map(|size| size.value()))
            .maybe_set_attribute("usemap", self.get_attribute("usemap"));
        self.set_style_img(img).closed()
    }

    fn render_link(&self) -> String {
        Tag::new("a")
            .maybe_set_attribute("href", self.get_attribute("href"))
            .maybe_set_attribute("name", self.get_attribute("name"))
            .maybe_set_attribute("rel", self.get_attribute("rel"))
            .maybe_set_attribute("target", self.get_attribute("target"))
            .render(self.render_image())
    }
}

impl Component for MJImage {
    fn update_header(&self, header: &mut Header) {
        let mut style = format!(
            "@media only screen and (max-width:{}) {{\n",
            header.breakpoint.to_string(),
        );
        style.push_str("table.mj-full-width-mobile { width: 100% !important; }\n");
        style.push_str("td.mj-full-width-mobile { width: auto !important; }\n");
        style.push_str("}\n");
        header.add_style(style);
    }

    fn context(&self) -> Option<&Context> {
        self.context.as_ref()
    }

    fn set_context(&mut self, ctx: Context) {
        self.context = Some(ctx);
    }

    fn render(&self, _header: &Header) -> Result<String, Error> {
        let table = Tag::table_presentation().maybe_set_class(if self.is_fluid_on_mobile() {
            Some("mj-full-width-mobile")
        } else {
            None
        });
        let table = self.set_style_table(table);
        let tbody = Tag::tbody();
        let tr = Tag::tr();
        let td = self
            .set_style_td(Tag::td())
            .maybe_set_class(if self.is_fluid_on_mobile() {
                Some("mj-full-width-mobile")
            } else {
                None
            });
        let content = if self.get_attribute("href").is_some() {
            self.render_link()
        } else {
            self.render_image()
        };
        Ok(table.render(tbody.render(tr.render(td.render(content)))))
    }
}

impl BodyComponent for MJImage {
    fn attributes(&self) -> Option<&Attributes> {
        Some(&self.attributes)
    }

    fn get_current_width(&self) -> Option<Size> {
        None
    }

    fn set_style(&self, name: &str, tag: Tag) -> Tag {
        match name {
            "img" => self.set_style_img(tag),
            "td" => self.set_style_td(tag),
            "table" => self.set_style_table(tag),
            _ => tag,
        }
    }
}
