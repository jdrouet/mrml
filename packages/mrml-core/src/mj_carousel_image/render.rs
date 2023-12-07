use std::cell::{Ref, RefCell};
use std::rc::Rc;

use super::{MjCarouselImage, NAME};
use crate::helper::size::Pixel;
use crate::helper::tag::Tag;
use crate::prelude::hash::Map;
use crate::prelude::render::{Error, Header, Render, RenderOptions, Renderable};

struct MjCarouselImageRender<'e, 'h> {
    header: Rc<RefCell<Header<'h>>>,
    element: &'e MjCarouselImage,
    extra: Map<String, String>,
    container_width: Option<Pixel>,
    index: usize,
}

impl<'e, 'h> MjCarouselImageRender<'e, 'h> {
    fn set_style_images_img(&self, tag: Tag) -> Tag {
        tag.maybe_add_style("border-radius", self.attribute("border-radius"))
            .add_style("display", "block")
            .maybe_add_style(
                "width",
                self.container_width.as_ref().map(|value| value.to_string()),
            )
            .add_style("max-width", "100%")
            .add_style("height", "auto")
    }

    fn set_style_radio_input(&self, tag: Tag) -> Tag {
        tag.add_style("display", "none")
            .add_style("mso-hide", "all")
    }

    fn set_style_thumbnails_a(&self, tag: Tag) -> Tag {
        tag.maybe_add_style("border", self.attribute("tb-border"))
            .maybe_add_style("border-radius", self.attribute("tb-border-radius"))
            .add_style("display", "inline-block")
            .add_style("overflow", "hidden")
            .maybe_add_style("width", self.attribute("tb-width"))
    }

    fn set_style_thumbnails_img(&self, tag: Tag) -> Tag {
        tag.add_style("display", "block")
            .add_style("width", "100%")
            .add_style("height", "auto")
    }

    fn render_radio(&self) -> String {
        self.set_style_radio_input(Tag::new("input"))
            .add_class("mj-carousel-radio")
            .maybe_add_class(
                self.extra
                    .get("carousel-id")
                    .map(|id| format!("mj-carousel-{id}-radio")),
            )
            .maybe_add_class(
                self.extra
                    .get("carousel-id")
                    .map(|id| format!("mj-carousel-{}-radio-{}", id, self.index + 1)),
            )
            .maybe_add_attribute(
                "checked",
                if self.index == 0 {
                    Some("checked")
                } else {
                    None
                },
            )
            .add_attribute("type", "radio")
            .maybe_add_attribute(
                "name",
                self.extra
                    .get("carousel-id")
                    .map(|id| format!("mj-carousel-radio-{id}")),
            )
            .maybe_add_attribute(
                "id",
                self.extra
                    .get("carousel-id")
                    .map(|id| format!("mj-carousel-{}-radio-{}", id, self.index + 1)),
            )
            .closed()
    }

    pub fn render_thumbnail(&self) -> Result<String, Error> {
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
                self.container_width
                    .as_ref()
                    .map(|item| item.value().to_string()),
            )
            .closed();
        let label = Tag::new("label")
            .maybe_add_attribute(
                "for",
                self.extra
                    .get("carousel-id")
                    .map(|id| format!("mj-carousel-{}-radio-{}", id, self.index + 1)),
            )
            .render(img);
        Ok(self
            .set_style_thumbnails_a(Tag::new("a"))
            .add_attribute("href", format!("#{}", self.index + 1))
            .maybe_add_attribute("target", self.attribute("target"))
            .add_class("mj-carousel-thumbnail")
            .maybe_add_class(
                self.extra
                    .get("carousel-id")
                    .map(|id| format!("mj-carousel-{id}-thumbnail")),
            )
            .maybe_add_class(
                self.extra
                    .get("carousel-id")
                    .map(|id| format!("mj-carousel-{}-thumbnail-{}", id, self.index + 1)),
            )
            .maybe_add_suffixed_class(self.attribute("css-class"), "thumbnail")
            .maybe_add_style(
                "width",
                self.container_width.as_ref().map(|item| item.to_string()),
            )
            .render(label))
    }
}

impl<'e, 'h> Render<'h> for MjCarouselImageRender<'e, 'h> {
    fn default_attribute(&self, key: &str) -> Option<&str> {
        match key {
            "target" => Some("_blank"),
            _ => None,
        }
    }

    fn add_extra_attribute(&mut self, key: &str, value: &str) {
        self.extra.insert(key.to_string(), value.to_string());
    }

    fn extra_attributes(&self) -> Option<&Map<String, String>> {
        Some(&self.extra)
    }

    fn attributes(&self) -> Option<&Map<String, String>> {
        Some(&self.element.attributes)
    }

    fn tag(&self) -> Option<&str> {
        Some(NAME)
    }

    fn set_container_width(&mut self, width: Option<Pixel>) {
        self.container_width = width;
    }

    fn set_index(&mut self, index: usize) {
        self.index = index;
    }

    fn header(&self) -> Ref<Header<'h>> {
        self.header.borrow()
    }

    fn render_fragment(&self, name: &str, opts: &RenderOptions) -> Result<String, Error> {
        match name {
            "main" => self.render(opts),
            "radio" => Ok(self.render_radio()),
            "thumbnail" => self.render_thumbnail(),
            _ => Err(Error::UnknownFragment(name.to_string())),
        }
    }

    fn render(&self, _opts: &RenderOptions) -> Result<String, Error> {
        let img = self
            .set_style_images_img(Tag::new("img"))
            .add_attribute("border", "0")
            .maybe_add_attribute("alt", self.attribute("alt"))
            .maybe_add_attribute("src", self.attribute("src"))
            .maybe_add_attribute("title", self.attribute("title"))
            .maybe_add_attribute(
                "width",
                self.container_width
                    .as_ref()
                    .map(|width| width.value().to_string()),
            )
            .closed();
        let link = match self.attribute("href") {
            None => img,
            Some(href) => Tag::new("a")
                .add_attribute("href", href)
                .maybe_add_attribute("rel", self.attribute("rel"))
                .add_attribute("target", "_blank")
                .render(img),
        };
        let div = if self.index == 0 {
            Tag::div()
        } else {
            Tag::div()
                .add_style("display", "none")
                .add_style("mso-hide", "all")
        };
        let div = div
            .add_class("mj-carousel-image")
            .add_class(format!("mj-carousel-image-{}", self.index + 1))
            .maybe_add_class(self.attribute("css-class"))
            .render(link);
        Ok(div)
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MjCarouselImage {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'h> + 'r> {
        Box::new(MjCarouselImageRender::<'e, 'h> {
            element: self,
            header,
            extra: Map::new(),
            container_width: None,
            index: 0,
        })
    }
}
