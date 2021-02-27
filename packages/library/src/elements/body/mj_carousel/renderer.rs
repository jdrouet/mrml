use super::MJCarousel;
use crate::elements::body::mj_carousel_image::MJCarouselImage;
use crate::elements::body::prelude::{
    to_children_iterator, BodyChild, BodyComponent, BodyComponentChildIterator,
};
use crate::elements::error::Error;
use crate::elements::prelude::*;
use crate::util::attributes::*;
use crate::util::condition::*;
use crate::util::context::Context;
use crate::util::header::Header;
use crate::util::size::Size;
use crate::util::style::Style;
use crate::util::tag::Tag;

fn repeat(count: usize, value: &str) -> String {
    (0..count).map(|_idx| value).collect::<Vec<_>>().join("")
}

impl MJCarousel {
    fn get_images(&self) -> Vec<&MJCarouselImage> {
        self.children
            .iter()
            .filter_map(|item| item.as_element())
            .collect()
    }

    fn get_thumbnails_width(&self) -> Size {
        let count = self.children.len();
        if count == 0 {
            Size::Raw(0.0)
        } else {
            self.get_size_attribute("tb-width")
                .or_else(|| {
                    self.context()
                        .and_then(|ctx| ctx.container_width())
                        .map(|width| {
                            let value = width.value() / (count as f32);
                            if value < 110.0 {
                                Size::Pixel(value)
                            } else {
                                Size::Pixel(110.0)
                            }
                        })
                })
                .unwrap_or(Size::Raw(0.0))
        }
    }

    fn set_style_carousel_div(&self, tag: Tag) -> Tag {
        tag.set_style("display", "table")
            .set_style("font-size", "0px")
            .set_style("table-layout", "fixed")
            .set_style("text-align", "center")
            .set_style("width", "100%")
    }

    fn set_style_carousel_table(&self, tag: Tag) -> Tag {
        tag.set_style("caption-side", "top")
            .set_style("display", "table-caption")
            .set_style("table-layout", "fixed")
            .set_style("width", "100%")
    }

    fn set_style_images_td(&self, tag: Tag) -> Tag {
        tag.set_style("padding", "0px")
    }

    fn set_style_controls_div(&self, tag: Tag) -> Tag {
        tag.set_style("display", "none")
            .set_style("mso-hide", "all")
    }

    fn set_style_controls_img(&self, tag: Tag) -> Tag {
        tag.set_style("display", "block")
            .maybe_set_style("width", self.get_attribute("icon-width"))
            .set_style("height", "auto")
    }

    fn set_style_controls_td(&self, tag: Tag) -> Tag {
        tag.set_style("display", "none")
            .set_style("font-size", "0px")
            .set_style("mso-hide", "all")
            .set_style("padding", "0px")
    }

    fn render_radios(&self) -> String {
        self.get_images()
            .iter()
            .map(|item| item.render_radio())
            .collect::<Vec<_>>()
            .join("")
    }

    fn render_thumbnails(&self) -> String {
        let thumbnails = self.get_attribute("thumbnails").map(|value| value.as_str());
        if thumbnails != Some("visible") {
            String::new()
        } else {
            let width = self.get_thumbnails_width();
            self.get_images()
                .iter()
                .map(|item| item.render_thumbnail(&width))
                .collect::<Vec<_>>()
                .join("")
        }
    }

    fn render_controls(&self, direction: &str, icon: &str) -> String {
        let icon_width = self
            .get_size_attribute("icon-width")
            .map(|value| value.value());
        let items = self
            .children
            .iter()
            .enumerate()
            .map(|(idx, _item)| {
                let img = self
                    .set_style_controls_img(Tag::new("img"))
                    .set_attribute("src", icon)
                    .set_attribute("alt", direction)
                    .maybe_set_attribute("width", icon_width)
                    .closed();
                Tag::new("label")
                    .set_attribute("for", format!("mj-carousel-{}-radio-{}", self.id, idx + 1))
                    .set_class(format!("mj-carousel-{}", direction))
                    .set_class(format!("mj-carousel-{}-{}", direction, idx + 1))
                    .render(img)
            })
            .collect::<Vec<_>>()
            .join("");
        let div = self
            .set_style_controls_div(Tag::div())
            .set_class(format!("mj-carousel-{}-icons", direction))
            .render(items);
        self.set_style_controls_td(Tag::td())
            .set_class(format!("mj-carousel-{}-icons-cell", self.id))
            .render(div)
    }

    fn render_images(&self, header: &Header) -> Result<String, Error> {
        let mut content = vec![];
        for child in self.get_images().iter() {
            content.push(child.render(header)?);
        }
        let div = Tag::div()
            .set_class("mj-carousel-images")
            .render(content.join(""));
        let td = self.set_style_images_td(Tag::td()).render(div);
        Ok(td)
    }

    fn render_carousel(&self, header: &Header) -> Result<String, Error> {
        let previous = self.render_controls(
            "previous",
            self.get_attribute("left-icon").unwrap().as_str(),
        );
        let images = self.render_images(header)?;
        let next = self.render_controls("next", self.get_attribute("right-icon").unwrap().as_str());
        let tr = Tag::tr().render(previous + &images + &next);
        let tbody = Tag::tbody().render(tr);
        let table = self
            .set_style_carousel_table(Tag::table_presentation())
            .set_attribute("width", "100%")
            .set_class("mj-carousel-main")
            .render(tbody);
        Ok(table)
    }

    fn render_fallback(&self, header: &Header) -> Result<String, Error> {
        match self.children.first() {
            Some(child) => Ok(mso_conditional_tag(child.inner().render(header)?)),
            None => Ok("".into()),
        }
    }
}

impl Component for MJCarousel {
    fn update_header(&self, header: &mut Header) {
        let length = self.children.len();
        if length == 0 {
            return;
        }
        let mut style = vec![
            Style::default()
                .add_str_selector(".mj-carousel")
                .add_str_content("-webkit-user-select: none;")
                .add_str_content("-moz-user-select: none;")
                .add_str_content("user-select: none;")
                .to_string(),
            Style::default()
                .add_selector(format!(".mj-carousel-{}-icons-cell", self.id))
                .add_str_content("display: table-cell !important;")
                .add_content(format!(
                    "width: {} !important;",
                    self.get_attribute("icon-width").unwrap()
                ))
                .to_string(),
            Style::default()
                .add_str_selector(".mj-carousel-radio")
                .add_str_selector(".mj-carousel-next")
                .add_str_selector(".mj-carousel-previous")
                .add_str_content("display: none !important;")
                .to_string(),
            Style::default()
                .add_str_selector(".mj-carousel-thumbnail")
                .add_str_selector(".mj-carousel-next")
                .add_str_selector(".mj-carousel-previous")
                .add_str_content("touch-action: manipulation;")
                .to_string(),
        ];
        style.push(
            (0..length)
                .fold(Style::default(), |res, idx| {
                    let ext = repeat(idx, "+ * ");
                    res.add_selector(format!(
                        ".mj-carousel-{}-radio:checked {}+ .mj-carousel-content .mj-carousel-image",
                        self.id, ext
                    ))
                })
                .add_str_content("display: none !important;")
                .to_string(),
        );
        style.push(
            (0..length)
                .fold(Style::default(), |res, idx| {
                    let ext = repeat(length - idx - 1, "+ * ");
                    res.add_selector(format!(
                        ".mj-carousel-{}-radio-{}:checked {}+ .mj-carousel-content .mj-carousel-image-{}",
                        self.id, idx + 1, ext, idx + 1
                    ))
                })
                .add_str_content("display: block !important;").to_string(),
        );
        let base = Style::default()
            .add_str_selector(".mj-carousel-previous-icons")
            .add_str_selector(".mj-carousel-next-icons");
        let base =
            (0..length).fold(base, |res, idx| {
                let ext = repeat(length - idx - 1, "+ * ");
                let index = (idx + 1) % length + 1;
                res.add_selector(format!(
                ".mj-carousel-{}-radio-{}:checked {}+ .mj-carousel-content .mj-carousel-next-{}",
                self.id, idx + 1, ext, index
            ))
            });
        let base = (0..length).fold(base, |res, idx| {
            let ext = repeat(length - idx - 1, "+ * ");
            let index = (idx + length - 1) % length + 1;
            res.add_selector(format!(
                ".mj-carousel-{}-radio-{}:checked {}+ .mj-carousel-content .mj-carousel-previous-{}",
                self.id, idx + 1, ext, index
            ))
        });
        style.push(
            base.add_str_content("display: block !important;")
                .to_string(),
        );
        let base = (0..length).fold(Style::default(), |res, idx| {
            let ext = repeat(length - idx - 1, "+ * ");
            res.add_selector(format!(".mj-carousel-{}-radio-{}:checked {}+ .mj-carousel-content .mj-carousel-{}-thumbnail-{}", self.id, idx + 1, ext, self.id, idx + 1))
        });
        style.push(
            base.add_content(format!(
                "border-color: {} !important;",
                self.get_attribute("tb-selected-border-color").unwrap()
            ))
            .to_string(),
        );
        style.push(
            Style::default()
                .add_str_selector(".mj-carousel-image img + div")
                .add_str_selector(".mj-carousel-thumbnail img + div")
                .add_str_content("display: none !important;")
                .to_string(),
        );
        style.push(
            (0..length)
                .fold(Style::default(), |res, idx| {
                    let ext = repeat(length - idx - 1, "+ * ");
                    res.add_selector(format!(
                        ".mj-carousel-{}-thumbnail:hover {}+ .mj-carousel-main .mj-carousel-image",
                        self.id, ext
                    ))
                })
                .add_str_content("display: none !important;")
                .to_string(),
        );
        style.push(
            Style::default()
                .add_str_selector(".mj-carousel-thumbnail:hover")
                .add_content(format!(
                    "border-color: {} !important;",
                    self.get_attribute("tb-hover-border-color").unwrap()
                ))
                .to_string(),
        );
        style.push((0..length).fold(Style::default(), |res, idx| {
            let ext = repeat(length - idx - 1, "+ * ");
            res.add_selector(format!(".mj-carousel-{}-thumbnail-{}:hover {}+ .mj-carousel-main .mj-carousel-image-{}", self.id, idx + 1, ext, idx + 1))
        }).add_str_content("display: block !important;").to_string());
        style.push(".mj-carousel noinput { display:block !important; }".into());
        style.push(
            ".mj-carousel noinput .mj-carousel-image-1 { display: block !important;  }".into(),
        );
        style.push(".mj-carousel noinput .mj-carousel-arrows, .mj-carousel noinput .mj-carousel-thumbnails { display: none !important; }".into());

        style.push("[owa] .mj-carousel-thumbnail { display: none !important; }".into());

        style.push(format!(
            r#"
        @media screen yahoo {{
            .mj-carousel-{}-icons-cell,
            .mj-carousel-previous-icons,
            .mj-carousel-next-icons {{
                display: none !important;
            }}

            .mj-carousel-{}-radio-1:checked {}+ .mj-carousel-content .mj-carousel-{}-thumbnail-1 {{
                border-color: transparent;
            }}
        }}
        "#,
            self.id,
            self.id,
            repeat(length - 1, "+ *"),
            self.id
        ));

        header.add_style(style.join("\n"));
        self.get_children().for_each(|child| {
            child.update_header(header);
        });
    }

    fn context(&self) -> Option<&Context> {
        self.context.as_ref()
    }

    fn set_context(&mut self, ctx: Context) {
        self.context = Some(ctx);
        let child_base = Context::new(
            self.get_container_width(),
            self.get_siblings(),
            self.get_raw_siblings(),
            0,
        );
        self.children
            .iter_mut()
            .enumerate()
            .for_each(|(idx, child)| {
                child
                    .inner_mut()
                    .set_context(child_base.clone().set_index(idx));
            });
    }

    fn render(&self, header: &Header) -> Result<String, Error> {
        let radios = self.render_radios();
        let thumbnails = self.render_thumbnails();
        let carousel = self.render_carousel(header)?;
        let inner_div = self
            .set_style_carousel_div(Tag::div())
            .set_class("mj-carousel-content")
            .set_class(format!("mj-carousel-{}-content", self.id))
            .render(thumbnails + &carousel);
        let fallback = self.render_fallback(header)?;
        Ok(mso_negation_conditional_tag(
            Tag::div()
                .set_class("mj-carousel")
                .render(radios + &inner_div),
        ) + &fallback)
    }
}

impl BodyComponent for MJCarousel {
    fn get_children(&self) -> BodyComponentChildIterator {
        to_children_iterator(&self.children)
    }

    fn get_children_len(&self) -> usize {
        self.children.len()
    }

    fn get_current_width(&self) -> Option<Size> {
        self.context().and_then(|ctx| ctx.container_width())
    }

    fn attributes(&self) -> Option<&Attributes> {
        Some(&self.attributes)
    }

    fn set_style(&self, _name: &str, tag: Tag) -> Tag {
        tag
    }
}
