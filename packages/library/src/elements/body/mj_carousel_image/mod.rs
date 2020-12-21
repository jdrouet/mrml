mod parser;

use crate::elements::body::prelude::*;
use crate::elements::body::BodyElement;
use crate::elements::error::Error;
use crate::elements::prelude::*;
use crate::util::attributes::*;
use crate::util::context::Context;
use crate::util::header::Header;
use crate::util::size::Size;
use crate::util::tag::Tag;

#[derive(Clone, Debug)]
pub struct MJCarouselImage {
    attributes: Attributes,
    carousel_id: String,
    context: Option<Context>,
    content: Option<String>,
}

impl MJCarouselImage {
    fn set_style_images_img(&self, tag: Tag) -> Tag {
        tag.maybe_set_style("border-radius", self.get_attribute("border-radius"))
            .set_style("display", "block")
            .set_style("max-width", "100%")
            .set_style("height", "auto")
    }

    fn set_style_radio_input(&self, tag: Tag) -> Tag {
        tag.set_style("display", "none")
            .set_style("mso-hide", "all")
    }

    fn set_style_thumbnails_a(&self, tag: Tag) -> Tag {
        tag.maybe_set_style("border", self.get_attribute("tb-border"))
            .maybe_set_style("border-radius", self.get_attribute("tb-border-radius"))
            .set_style("display", "inline-block")
            .set_style("overflow", "hidden")
            .maybe_set_style("width", self.get_attribute("tb-width"))
    }

    fn set_style_thumbnails_img(&self, tag: Tag) -> Tag {
        tag.set_style("display", "block")
            .set_style("height", "auto")
            .set_style("width", "100%")
    }

    pub fn render_radio(&self) -> String {
        let index = self.context().map(|ctx| ctx.index()).unwrap_or(0);
        self.set_style_radio_input(Tag::new("input"))
            .set_class("mj-carousel-radio")
            .set_class(format!("mj-carousel-{}-radio", self.carousel_id))
            .set_class(format!(
                "mj-carousel-{}-radio-{}",
                self.carousel_id,
                index + 1
            ))
            .maybe_set_attribute("checked", if index == 0 { Some("checked") } else { None })
            .set_attribute("type", "radio")
            .set_attribute("name", format!("mj-carousel-radio-{}", self.carousel_id))
            .set_attribute(
                "id",
                format!("mj-carousel-{}-radio-{}", self.carousel_id, index + 1),
            )
            .closed()
    }

    pub fn render_thumbnail(&self, width: &Size) -> String {
        let index = self.context().map(|ctx| ctx.index()).unwrap_or(0) + 1;
        let img = self
            .set_style_thumbnails_img(Tag::new("img"))
            .maybe_set_attribute(
                "src",
                self.get_attribute("thumbnails-src")
                    .or_else(|| self.get_attribute("src")),
            )
            .maybe_set_attribute("alt", self.get_attribute("alt"))
            .set_attribute("width", width.value())
            .closed();
        let label = Tag::new("label")
            .set_attribute(
                "for",
                format!("mj-carousel-{}-radio-{}", self.carousel_id, index),
            )
            .render(img);
        self.set_style_thumbnails_a(Tag::new("a"))
            .set_attribute("href", format!("#{}", index))
            .maybe_set_attribute("target", self.get_attribute("target"))
            .set_class("mj-carousel-thumbnail")
            .set_class(format!("mj-carousel-{}-thumbnail", self.carousel_id))
            .set_class(format!(
                "mj-carousel-{}-thumbnail-{}",
                self.carousel_id, index
            ))
            .maybe_set_class(suffix_css_classes(
                self.get_attribute("css-class"),
                "thumbnail",
            ))
            .set_style("width", width.to_string())
            .render(label)
    }
}

impl Component for MJCarouselImage {
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
        let index = self.context().map(|ctx| ctx.index()).unwrap_or(0);
        let width = self.context().and_then(|ctx| ctx.container_width());
        let img = self
            .set_style_images_img(Tag::new("img"))
            .set_attribute("border", 0)
            .maybe_set_attribute("alt", self.get_attribute("alt"))
            .maybe_set_attribute("src", self.get_attribute("src"))
            .maybe_set_attribute("title", self.get_attribute("title"))
            .maybe_set_attribute("width", width.as_ref().map(|width| width.value()))
            .maybe_set_style("width", width)
            .closed();
        let link = match self.get_attribute("href") {
            None => img,
            Some(href) => Tag::new("a")
                .set_attribute("href", href)
                .maybe_set_attribute("rel", self.get_attribute("rel"))
                .set_attribute("target", "_blank")
                .render(img),
        };
        let div = if index == 0 {
            Tag::div()
        } else {
            Tag::div()
                .set_style("display", "none")
                .set_style("mso-hide", "all")
        };
        let div = div
            .set_class("mj-carousel-image")
            .set_class(format!("mj-carousel-image-{}", index + 1))
            .maybe_set_class(self.get_attribute("css-class"))
            .render(link);
        Ok(div)
    }
}

impl BodyComponent for MJCarouselImage {
    fn attributes(&self) -> Option<&Attributes> {
        Some(&self.attributes)
    }

    fn get_current_width(&self) -> Option<Size> {
        None
    }

    fn get_children(&self) -> &Vec<BodyElement> {
        &EMPTY_CHILDREN
    }
}
