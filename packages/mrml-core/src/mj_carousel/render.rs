use std::cell::{Ref, RefCell};
use std::rc::Rc;

use super::{MjCarousel, MjCarouselChild, NAME};
use crate::helper::size::{Pixel, Size};
use crate::helper::style::Style;
use crate::prelude::render::{Error, Header, Render, RenderBuffer, RenderOptions, Renderable, Tag};

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MjCarouselChild {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'e, 'h> + 'r> {
        match self {
            Self::MjCarouselImage(elt) => elt.renderer(header),
            Self::Comment(elt) => elt.renderer(header),
        }
    }
}

fn repeat(count: usize, value: &str) -> String {
    (0..count).map(|_idx| value).collect::<Vec<_>>().join("")
}

struct MjCarouselRender<'e, 'h> {
    header: Rc<RefCell<Header<'h>>>,
    element: &'e MjCarousel,
    container_width: Option<Pixel>,
    siblings: usize,
    raw_siblings: usize,
    id: String,
}

impl<'e, 'h> MjCarouselRender<'e, 'h> {
    fn get_thumbnails_width(&self) -> Pixel {
        let count = self.element.children.len();
        if count == 0 {
            Pixel::new(0.0)
        } else {
            self.attribute_as_pixel("tb-width")
                .or_else(|| {
                    self.container_width.as_ref().map(|width| {
                        let value = width.value() / (count as f32);
                        if value < 110.0 {
                            Pixel::new(value)
                        } else {
                            Pixel::new(110.0)
                        }
                    })
                })
                .unwrap_or_else(|| Pixel::new(0.0))
        }
    }

    fn set_style_carousel_div(&self, tag: Tag) -> Tag {
        tag.add_style("display", "table")
            .add_style("width", "100%")
            .add_style("table-layout", "fixed")
            .add_style("text-align", "center")
            .add_style("font-size", "0px")
    }

    fn set_style_carousel_table(&self, tag: Tag) -> Tag {
        tag.add_style("caption-side", "top")
            .add_style("display", "table-caption")
            .add_style("table-layout", "fixed")
            .add_style("width", "100%")
    }

    fn set_style_images_td(&self, tag: Tag) -> Tag {
        tag.add_style("padding", "0px")
    }

    fn set_style_controls_div(&self, tag: Tag) -> Tag {
        tag.add_style("display", "none")
            .add_style("mso-hide", "all")
    }

    fn set_style_controls_img(&self, tag: Tag) -> Tag {
        tag.add_style("display", "block")
            .maybe_add_style("width", self.attribute("icon-width"))
            .add_style("height", "auto")
    }

    fn set_style_controls_td(&self, tag: Tag) -> Tag {
        tag.add_style("font-size", "0px")
            .add_style("display", "none")
            .add_style("mso-hide", "all")
            .add_style("padding", "0px")
    }

    fn render_radios(&self, opts: &RenderOptions, buf: &mut RenderBuffer) -> Result<(), Error> {
        for (index, child) in self.element.children.iter().enumerate() {
            let mut renderer = child.renderer(Rc::clone(&self.header));
            renderer.add_extra_attribute("carousel-id", &self.id);
            renderer.maybe_add_extra_attribute("border-radius", self.attribute("border-radius"));
            renderer.maybe_add_extra_attribute("tb-border", self.attribute("tb-border"));
            renderer
                .maybe_add_extra_attribute("tb-border-radius", self.attribute("tb-border-radius"));
            renderer.set_index(index);
            renderer.render_fragment("radio", opts, buf)?;
        }
        Ok(())
    }

    fn render_thumbnails(&self, opts: &RenderOptions, buf: &mut RenderBuffer) -> Result<(), Error> {
        if self.attribute_equals("thumbnails", "visible") {
            let width = self.get_thumbnails_width();

            for (index, child) in self.element.children.iter().enumerate() {
                let mut renderer = child.renderer(Rc::clone(&self.header));
                renderer.add_extra_attribute("carousel-id", &self.id);
                renderer
                    .maybe_add_extra_attribute("border-radius", self.attribute("border-radius"));
                renderer.maybe_add_extra_attribute("tb-border", self.attribute("tb-border"));
                renderer.maybe_add_extra_attribute(
                    "tb-border-radius",
                    self.attribute("tb-border-radius"),
                );
                renderer.set_index(index);
                renderer.set_container_width(Some(width.clone()));
                renderer.render_fragment("thumbnail", opts, buf)?;
            }
        }

        Ok(())
    }

    fn render_controls(&self, direction: &str, icon: &str, buf: &mut RenderBuffer) {
        let icon_width = self
            .attribute_as_size("icon-width")
            .map(|value| value.value());
        let div = self
            .set_style_controls_div(Tag::div())
            .add_class(format!("mj-carousel-{direction}-icons"));
        let td = self
            .set_style_controls_td(Tag::td())
            .add_class(format!("mj-carousel-{}-icons-cell", self.id));

        td.render_open(buf);
        div.render_open(buf);
        for (index, _) in self.element.children.iter().enumerate() {
            let img = self
                .set_style_controls_img(Tag::new("img"))
                .add_attribute("src", icon.to_string())
                .add_attribute("alt", direction.to_string())
                .maybe_add_attribute("width", icon_width.map(|v| v.to_string()));
            let label = Tag::new("label")
                .add_attribute(
                    "for",
                    format!("mj-carousel-{}-radio-{}", self.id, index + 1),
                )
                .add_class(format!("mj-carousel-{direction}"))
                .add_class(format!("mj-carousel-{}-{}", direction, index + 1));
            label.render_open(buf);
            img.render_closed(buf);
            label.render_close(buf);
        }
        div.render_close(buf);
        td.render_close(buf);
    }

    fn render_images(&self, opts: &RenderOptions, buf: &mut RenderBuffer) -> Result<(), Error> {
        let div = Tag::div().add_class("mj-carousel-images");
        let td = self.set_style_images_td(Tag::td());

        td.render_open(buf);
        div.render_open(buf);

        for (index, child) in self.element.children.iter().enumerate() {
            let mut renderer = child.renderer(Rc::clone(&self.header));
            renderer.add_extra_attribute("carousel-id", &self.id);
            renderer.maybe_add_extra_attribute("border-radius", self.attribute("border-radius"));
            renderer.maybe_add_extra_attribute("tb-border", self.attribute("tb-border"));
            renderer
                .maybe_add_extra_attribute("tb-border-radius", self.attribute("tb-border-radius"));
            renderer.set_index(index);
            renderer.set_container_width(self.container_width.clone());
            renderer.render(opts, buf)?;
        }

        div.render_close(buf);
        td.render_close(buf);

        Ok(())
    }

    fn render_carousel(&self, opts: &RenderOptions, buf: &mut RenderBuffer) -> Result<(), Error> {
        let tr = Tag::tr();
        let tbody = Tag::tbody();
        let table = self
            .set_style_carousel_table(Tag::table_presentation())
            .add_attribute("width", "100%")
            .add_class("mj-carousel-main");

        table.render_open(buf);
        tbody.render_open(buf);
        tr.render_open(buf);

        self.render_controls(
            "previous",
            self.attribute("left-icon").unwrap().as_str(),
            buf,
        );
        self.render_images(opts, buf)?;
        self.render_controls("next", self.attribute("right-icon").unwrap().as_str(), buf);

        tr.render_close(buf);
        tbody.render_close(buf);
        table.render_close(buf);

        Ok(())
    }

    fn render_fallback(&self, opts: &RenderOptions, buf: &mut RenderBuffer) -> Result<(), Error> {
        if let Some(child) = self
            .element
            .children
            .iter()
            .find_map(|child| child.as_mj_carousel_image())
        {
            let mut renderer = child.renderer(Rc::clone(&self.header));
            renderer.add_extra_attribute("carousel-id", &self.id);
            renderer.maybe_add_extra_attribute("border-radius", self.attribute("border-radius"));
            renderer.maybe_add_extra_attribute("tb-border", self.attribute("tb-border"));
            renderer
                .maybe_add_extra_attribute("tb-border-radius", self.attribute("tb-border-radius"));
            renderer.set_container_width(self.container_width.clone());

            buf.start_mso_conditional_tag();
            renderer.render(opts, buf)?;
            buf.end_conditional_tag();
        }
        Ok(())
    }

    fn render_style(&self) -> Option<String> {
        if self.element.children.is_empty() {
            return None;
        }
        let length = self.element.children.len();
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
                    self.attribute("icon-width").unwrap()
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
        let base = (0..length).fold(base, |res, idx| {
            let ext = repeat(length - idx - 1, "+ * ");
            let index = (idx + 1) % length + 1;
            res.add_selector(format!(
                ".mj-carousel-{}-radio-{}:checked {}+ .mj-carousel-content .mj-carousel-next-{}",
                self.id,
                idx + 1,
                ext,
                index
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
                self.attribute("tb-selected-border-color").unwrap()
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
                    self.attribute("tb-hover-border-color").unwrap()
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
        @media screen, yahoo {{
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
        Some(style.join("\n"))
    }
}

impl<'e, 'h> Render<'e, 'h> for MjCarouselRender<'e, 'h> {
    fn default_attribute(&self, name: &str) -> Option<&'static str> {
        match name {
            "align" => Some("center"),
            "border-radius" => Some("6px"),
            "icon-width" => Some("44px"),
            "left-icon" => Some("https://i.imgur.com/xTh3hln.png"),
            "right-icon" => Some("https://i.imgur.com/os7o9kz.png"),
            "thumbnails" => Some("visible"),
            "tb-border" => Some("2px solid transparent"),
            "tb-border-radius" => Some("6px"),
            "tb-hover-border-color" => Some("#fead0d"),
            "tb-selected-border-color" => Some("#cccccc"),
            _ => None,
        }
    }

    fn raw_attribute(&self, key: &str) -> Option<&'e str> {
        self.element.attributes.get(key).map(|v| v.as_str())
    }

    fn tag(&self) -> Option<&str> {
        Some(NAME)
    }

    fn header(&self) -> Ref<Header<'h>> {
        self.header.borrow()
    }

    fn get_width(&self) -> Option<Size> {
        self.container_width
            .as_ref()
            .map(|w| Size::Pixel(w.clone()))
    }

    fn set_container_width(&mut self, width: Option<Pixel>) {
        self.container_width = width;
    }

    fn set_siblings(&mut self, value: usize) {
        self.siblings = value;
    }

    fn set_raw_siblings(&mut self, value: usize) {
        self.raw_siblings = value;
    }

    fn render(&self, opts: &RenderOptions, buf: &mut RenderBuffer) -> Result<(), Error> {
        let styles = self.render_style();
        self.header.borrow_mut().maybe_add_style(styles);
        let inner_div = self
            .set_style_carousel_div(Tag::div())
            .add_class("mj-carousel-content")
            .add_class(format!("mj-carousel-{}-content", self.id));
        let div = Tag::div().add_class("mj-carousel");

        buf.start_mso_negation_conditional_tag();
        div.render_open(buf);
        self.render_radios(opts, buf)?;
        inner_div.render_open(buf);
        self.render_thumbnails(opts, buf)?;
        self.render_carousel(opts, buf)?;
        inner_div.render_close(buf);
        div.render_close(buf);
        buf.end_negation_conditional_tag();
        self.render_fallback(opts, buf)?;

        Ok(())
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MjCarousel {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'e, 'h> + 'r> {
        let id = header.borrow().next_id();
        Box::new(MjCarouselRender::<'e, 'h> {
            element: self,
            header,
            id,
            container_width: None,
            siblings: 1,
            raw_siblings: 0,
        })
    }
}

#[cfg(test)]
mod tests {
    crate::should_render!(basic, "mj-carousel");
    crate::should_render!(
        align_border_radius_class,
        "mj-carousel-align-border-radius-class"
    );
    crate::should_render!(icon, "mj-carousel-icon");
    crate::should_render!(tb, "mj-carousel-tb");
    crate::should_render!(thumbnails, "mj-carousel-thumbnails");
}
