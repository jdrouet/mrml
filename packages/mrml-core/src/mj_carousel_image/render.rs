use super::{MjCarouselImage, NAME};
use crate::helper::size::Pixel;
use crate::prelude::hash::Map;
use crate::prelude::render::*;

struct MjCarouselImageExtra {
    attributes: Map<String, String>,
    container_width: Option<Pixel>,
    index: usize,
}

impl<'element, 'header> Renderer<'element, 'header, MjCarouselImage, MjCarouselImageExtra> {
    fn set_style_images_img<'a>(&self, tag: Tag<'a>) -> Tag<'a> {
        tag.maybe_add_style("border-radius", self.attribute("border-radius"))
            .add_style("display", "block")
            .maybe_add_style(
                "width",
                self.extra
                    .container_width
                    .as_ref()
                    .map(|value| value.to_string()),
            )
            .add_style("max-width", "100%")
            .add_style("height", "auto")
    }

    fn set_style_radio_input<'a>(&self, tag: Tag<'a>) -> Tag<'a> {
        tag.add_style("display", "none")
            .add_style("mso-hide", "all")
    }

    fn set_style_thumbnails_a<'a>(&self, tag: Tag<'a>) -> Tag<'a> {
        tag.maybe_add_style("border", self.attribute("tb-border"))
            .maybe_add_style("border-radius", self.attribute("tb-border-radius"))
            .add_style("display", "inline-block")
            .add_style("overflow", "hidden")
            .maybe_add_style("width", self.attribute("tb-width"))
    }

    fn set_style_thumbnails_img<'a>(&self, tag: Tag<'a>) -> Tag<'a> {
        tag.add_style("display", "block")
            .add_style("width", "100%")
            .add_style("height", "auto")
    }

    fn render_radio(&self, buf: &mut RenderBuffer) {
        self.set_style_radio_input(Tag::new("input"))
            .add_class("mj-carousel-radio")
            .maybe_add_class(
                self.extra
                    .attributes
                    .get("carousel-id")
                    .map(|id| format!("mj-carousel-{id}-radio")),
            )
            .maybe_add_class(
                self.extra
                    .attributes
                    .get("carousel-id")
                    .map(|id| format!("mj-carousel-{}-radio-{}", id, self.extra.index + 1)),
            )
            .maybe_add_attribute(
                "checked",
                if self.extra.index == 0 {
                    Some("checked")
                } else {
                    None
                },
            )
            .add_attribute("type", "radio")
            .maybe_add_attribute(
                "name",
                self.extra
                    .attributes
                    .get("carousel-id")
                    .map(|id| format!("mj-carousel-radio-{id}")),
            )
            .maybe_add_attribute(
                "id",
                self.extra
                    .attributes
                    .get("carousel-id")
                    .map(|id| format!("mj-carousel-{}-radio-{}", id, self.extra.index + 1)),
            )
            .render_closed(buf);
    }

    pub fn render_thumbnail(&self, buf: &mut RenderBuffer) -> Result<(), Error> {
        let img = self
            .set_style_thumbnails_img(Tag::new("img"))
            .maybe_add_attribute(
                "src",
                self.attribute("thumbnails-src")
                    .or_else(|| self.attribute("src")),
            )
            .maybe_add_attribute("alt", self.attribute("alt"))
            .maybe_add_attribute(
                "width",
                self.extra
                    .container_width
                    .as_ref()
                    .map(|item| item.value().to_string()),
            );
        let label = Tag::new("label").maybe_add_attribute(
            "for",
            self.extra
                .attributes
                .get("carousel-id")
                .map(|id| format!("mj-carousel-{}-radio-{}", id, self.extra.index + 1)),
        );
        let link = self
            .set_style_thumbnails_a(Tag::new("a"))
            .add_attribute("href", format!("#{}", self.extra.index + 1))
            .maybe_add_attribute("target", self.attribute("target"))
            .add_class("mj-carousel-thumbnail")
            .maybe_add_class(
                self.extra
                    .attributes
                    .get("carousel-id")
                    .map(|id| format!("mj-carousel-{id}-thumbnail")),
            )
            .maybe_add_class(
                self.extra
                    .attributes
                    .get("carousel-id")
                    .map(|id| format!("mj-carousel-{}-thumbnail-{}", id, self.extra.index + 1)),
            )
            .maybe_add_suffixed_class(self.attribute("css-class"), "thumbnail")
            .maybe_add_style(
                "width",
                self.extra
                    .container_width
                    .as_ref()
                    .map(|item| item.to_string()),
            );

        link.render_open(buf);
        label.render_open(buf);
        img.render_closed(buf);
        label.render_close(buf);
        link.render_close(buf);

        Ok(())
    }
}

impl<'element, 'header> Render<'element, 'header>
    for Renderer<'element, 'header, MjCarouselImage, MjCarouselImageExtra>
{
    fn default_attribute(&self, key: &str) -> Option<&'static str> {
        match key {
            "target" => Some("_blank"),
            _ => None,
        }
    }

    fn add_extra_attribute(&mut self, key: &str, value: &str) {
        self.extra
            .attributes
            .insert(key.to_string(), value.to_string());
    }

    fn raw_extra_attribute(&self, key: &str) -> Option<&str> {
        self.extra.attributes.get(key).map(|v| v.as_str())
    }

    fn raw_attribute(&self, key: &str) -> Option<&'element str> {
        self.element.attributes.get(key).map(|v| v.as_str())
    }

    fn tag(&self) -> Option<&str> {
        Some(NAME)
    }

    fn set_container_width(&mut self, width: Option<Pixel>) {
        self.extra.container_width = width;
    }

    fn set_index(&mut self, index: usize) {
        self.extra.index = index;
    }

    fn context(&self) -> &'header RenderContext<'header> {
        self.context
    }

    fn render_fragment(&self, name: &str, cursor: &mut RenderCursor) -> Result<(), Error> {
        match name {
            "main" => self.render(cursor),
            "radio" => {
                self.render_radio(&mut cursor.buffer);
                Ok(())
            }
            "thumbnail" => self.render_thumbnail(&mut cursor.buffer),
            _ => Err(Error::UnknownFragment(name.to_string())),
        }
    }

    fn render(&self, cursor: &mut RenderCursor) -> Result<(), Error> {
        let img = self
            .set_style_images_img(Tag::new("img"))
            .add_attribute("border", "0")
            .maybe_add_attribute("alt", self.attribute("alt"))
            .maybe_add_attribute("src", self.attribute("src"))
            .maybe_add_attribute("title", self.attribute("title"))
            .maybe_add_attribute(
                "width",
                self.extra
                    .container_width
                    .as_ref()
                    .map(|width| width.value().to_string()),
            );
        let div = if self.extra.index == 0 {
            Tag::div()
        } else {
            Tag::div()
                .add_style("display", "none")
                .add_style("mso-hide", "all")
        };
        let div = div
            .add_class("mj-carousel-image")
            .add_class(format!("mj-carousel-image-{}", self.extra.index + 1))
            .maybe_add_class(self.attribute("css-class"));

        div.render_open(&mut cursor.buffer);
        if let Some(href) = self.attribute("href") {
            let link = Tag::new("a")
                .add_attribute("href", href)
                .maybe_add_attribute("rel", self.attribute("rel"))
                .add_attribute("target", "_blank");
            link.render_open(&mut cursor.buffer);
            img.render_closed(&mut cursor.buffer);
            link.render_close(&mut cursor.buffer);
        } else {
            img.render_closed(&mut cursor.buffer);
        }
        div.render_close(&mut cursor.buffer);

        Ok(())
    }
}

impl<'r, 'element: 'r, 'header: 'r> Renderable<'r, 'element, 'header> for MjCarouselImage {
    fn renderer(
        &'element self,
        context: &'header RenderContext<'header>,
    ) -> Box<dyn Render<'element, 'header> + 'r> {
        Box::new(Renderer::new(
            context,
            self,
            MjCarouselImageExtra {
                attributes: Map::new(),
                container_width: None,
                index: 0,
            },
        ))
    }
}
