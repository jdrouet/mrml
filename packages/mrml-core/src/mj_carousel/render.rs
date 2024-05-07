use super::{MjCarousel, MjCarouselChild, NAME};
use crate::helper::size::{Pixel, Size};
use crate::helper::style::Style;
use crate::prelude::render::*;

impl<'render, 'root: 'render> Renderable<'render, 'root> for MjCarouselChild {
    fn renderer(
        &'root self,
        context: &'root RenderContext<'root>,
    ) -> Box<dyn Render<'root> + 'render> {
        match self {
            Self::MjCarouselImage(elt) => elt.renderer(context),
            Self::Comment(elt) => elt.renderer(context),
        }
    }
}

fn repeat(count: usize, value: &str) -> String {
    (0..count).map(|_idx| value).collect::<Vec<_>>().join("")
}

struct MjCarouselExtra {
    id: String,
}

impl<'root> Renderer<'root, MjCarousel, MjCarouselExtra> {
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

    fn set_style_carousel_div<'t>(&self, tag: Tag<'t>) -> Tag<'t> {
        tag.add_style("display", "table")
            .add_style("width", "100%")
            .add_style("table-layout", "fixed")
            .add_style("text-align", "center")
            .add_style("font-size", "0px")
    }

    fn set_style_carousel_table<'t>(&self, tag: Tag<'t>) -> Tag<'t> {
        tag.add_style("caption-side", "top")
            .add_style("display", "table-caption")
            .add_style("table-layout", "fixed")
            .add_style("width", "100%")
    }

    fn set_style_images_td<'t>(&self, tag: Tag<'t>) -> Tag<'t> {
        tag.add_style("padding", "0px")
    }

    fn set_style_controls_div<'t>(&self, tag: Tag<'t>) -> Tag<'t> {
        tag.add_style("display", "none")
            .add_style("mso-hide", "all")
    }

    fn set_style_controls_img<'a, 't>(&'a self, tag: Tag<'t>) -> Tag<'t>
    where
        'root: 'a,
        'a: 't,
    {
        tag.add_style("display", "block")
            .maybe_add_style("width", self.attribute("icon-width"))
            .add_style("height", "auto")
    }

    fn set_style_controls_td<'t>(&self, tag: Tag<'t>) -> Tag<'t> {
        tag.add_style("font-size", "0px")
            .add_style("display", "none")
            .add_style("mso-hide", "all")
            .add_style("padding", "0px")
    }

    fn render_radios(&self, cursor: &mut RenderCursor) -> Result<(), Error> {
        let border_radius = self.attribute("border-radius");
        let tb_border = self.attribute("tb-border");
        let tb_border_radius = self.attribute("tb-border-radius");

        for (index, child) in self.element.children.iter().enumerate() {
            let mut renderer = child.renderer(self.context());
            renderer.add_extra_attribute("carousel-id", &self.extra.id);
            renderer.maybe_add_extra_attribute("border-radius", border_radius);
            renderer.maybe_add_extra_attribute("tb-border", tb_border);
            renderer.maybe_add_extra_attribute("tb-border-radius", tb_border_radius);
            renderer.set_index(index);
            renderer.render_fragment("radio", cursor)?;
        }

        Ok(())
    }

    fn render_thumbnails(&self, cursor: &mut RenderCursor) -> Result<(), Error> {
        if self.attribute_equals("thumbnails", "visible") {
            let width = self.get_thumbnails_width();

            let border_radius = self.attribute("border-radius");
            let tb_border = self.attribute("tb-border");
            let tb_border_radius = self.attribute("tb-border-radius");

            for (index, child) in self.element.children.iter().enumerate() {
                let mut renderer = child.renderer(self.context());
                renderer.add_extra_attribute("carousel-id", &self.extra.id);
                renderer.maybe_add_extra_attribute("border-radius", border_radius);
                renderer.maybe_add_extra_attribute("tb-border", tb_border);
                renderer.maybe_add_extra_attribute("tb-border-radius", tb_border_radius);
                renderer.set_index(index);
                renderer.set_container_width(Some(width.clone()));
                renderer.render_fragment("thumbnail", cursor)?;
            }
        }

        Ok(())
    }

    fn render_controls(
        &self,
        direction: &str,
        icon: &str,
        buf: &mut RenderBuffer,
    ) -> Result<(), Error> {
        let icon_width = self
            .attribute_as_size("icon-width")
            .map(|value| value.value());
        let div = self
            .set_style_controls_div(Tag::div())
            .add_class(format!("mj-carousel-{direction}-icons"));
        let td = self
            .set_style_controls_td(Tag::td())
            .add_class(format!("mj-carousel-{}-icons-cell", self.extra.id));

        td.render_open(buf)?;
        div.render_open(buf)?;
        for (index, _) in self.element.children.iter().enumerate() {
            let img = self
                .set_style_controls_img(Tag::new("img"))
                .add_attribute("src", icon.to_string())
                .add_attribute("alt", direction.to_string())
                .maybe_add_attribute("width", icon_width.map(|v| v.to_string()));
            let label = Tag::new("label")
                .add_attribute(
                    "for",
                    format!("mj-carousel-{}-radio-{}", self.extra.id, index + 1),
                )
                .add_class(format!("mj-carousel-{direction}"))
                .add_class(format!("mj-carousel-{}-{}", direction, index + 1));
            label.render_open(buf)?;
            img.render_closed(buf)?;
            label.render_close(buf);
        }
        div.render_close(buf);
        td.render_close(buf);

        Ok(())
    }

    fn render_images(&self, cursor: &mut RenderCursor) -> Result<(), Error> {
        let div = Tag::div().add_class("mj-carousel-images");
        let td = self.set_style_images_td(Tag::td());

        td.render_open(&mut cursor.buffer)?;
        div.render_open(&mut cursor.buffer)?;

        for (index, child) in self.element.children.iter().enumerate() {
            let mut renderer = child.renderer(self.context());
            renderer.add_extra_attribute("carousel-id", &self.extra.id);
            renderer.maybe_add_extra_attribute("border-radius", self.attribute("border-radius"));
            renderer.maybe_add_extra_attribute("tb-border", self.attribute("tb-border"));
            renderer
                .maybe_add_extra_attribute("tb-border-radius", self.attribute("tb-border-radius"));
            renderer.set_index(index);
            renderer.set_container_width(self.container_width.clone());
            renderer.render(cursor)?;
        }

        div.render_close(&mut cursor.buffer);
        td.render_close(&mut cursor.buffer);

        Ok(())
    }

    fn render_carousel(&self, cursor: &mut RenderCursor) -> Result<(), Error> {
        let tr = Tag::tr();
        let tbody = Tag::tbody();
        let table = self
            .set_style_carousel_table(Tag::table_presentation())
            .add_attribute("width", "100%")
            .add_class("mj-carousel-main");

        table.render_open(&mut cursor.buffer)?;
        tbody.render_open(&mut cursor.buffer)?;
        tr.render_open(&mut cursor.buffer)?;

        self.render_controls(
            "previous",
            self.attribute("left-icon").unwrap(),
            &mut cursor.buffer,
        )?;
        self.render_images(cursor)?;
        self.render_controls(
            "next",
            self.attribute("right-icon").unwrap(),
            &mut cursor.buffer,
        )?;

        tr.render_close(&mut cursor.buffer);
        tbody.render_close(&mut cursor.buffer);
        table.render_close(&mut cursor.buffer);

        Ok(())
    }

    fn render_fallback(&self, cursor: &mut RenderCursor) -> Result<(), Error> {
        if let Some(child) = self
            .element
            .children
            .iter()
            .find_map(|child| child.as_mj_carousel_image())
        {
            let mut renderer = child.renderer(self.context());
            renderer.add_extra_attribute("carousel-id", &self.extra.id);
            renderer.maybe_add_extra_attribute("border-radius", self.attribute("border-radius"));
            renderer.maybe_add_extra_attribute("tb-border", self.attribute("tb-border"));
            renderer
                .maybe_add_extra_attribute("tb-border-radius", self.attribute("tb-border-radius"));
            renderer.set_container_width(self.container_width.clone());

            cursor.buffer.start_mso_conditional_tag();
            renderer.render(cursor)?;
            cursor.buffer.end_conditional_tag();
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
                .add_selector(".mj-carousel")
                .add_content("-webkit-user-select: none;")
                .add_content("-moz-user-select: none;")
                .add_content("user-select: none;")
                .to_string(),
            Style::default()
                .add_selector(format!(".mj-carousel-{}-icons-cell", self.extra.id))
                .add_content("display: table-cell !important;")
                .add_content(format!(
                    "width: {} !important;",
                    self.attribute("icon-width").unwrap()
                ))
                .to_string(),
            Style::default()
                .add_selector(".mj-carousel-radio")
                .add_selector(".mj-carousel-next")
                .add_selector(".mj-carousel-previous")
                .add_content("display: none !important;")
                .to_string(),
            Style::default()
                .add_selector(".mj-carousel-thumbnail")
                .add_selector(".mj-carousel-next")
                .add_selector(".mj-carousel-previous")
                .add_content("touch-action: manipulation;")
                .to_string(),
        ];
        style.push(
            (0..length)
                .fold(Style::default(), |res, idx| {
                    let ext = repeat(idx, "+ * ");
                    res.add_selector(format!(
                        ".mj-carousel-{}-radio:checked {}+ .mj-carousel-content .mj-carousel-image",
                        self.extra.id, ext
                    ))
                })
                .add_content("display: none !important;")
                .to_string(),
        );
        style.push(
            (0..length)
                .fold(Style::default(), |res, idx| {
                    let ext = repeat(length - idx - 1, "+ * ");
                    res.add_selector(format!(
                        ".mj-carousel-{}-radio-{}:checked {}+ .mj-carousel-content .mj-carousel-image-{}",
                        self.extra.id, idx + 1, ext, idx + 1
                    ))
                })
                .add_content("display: block !important;").to_string(),
        );
        let base = Style::default()
            .add_selector(".mj-carousel-previous-icons")
            .add_selector(".mj-carousel-next-icons");
        let base = (0..length).fold(base, |res, idx| {
            let ext = repeat(length - idx - 1, "+ * ");
            let index = (idx + 1) % length + 1;
            res.add_selector(format!(
                ".mj-carousel-{}-radio-{}:checked {}+ .mj-carousel-content .mj-carousel-next-{}",
                self.extra.id,
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
                self.extra.id, idx + 1, ext, index
            ))
        });
        style.push(base.add_content("display: block !important;").to_string());
        let base = (0..length).fold(Style::default(), |res, idx| {
            let ext = repeat(length - idx - 1, "+ * ");
            res.add_selector(format!(".mj-carousel-{}-radio-{}:checked {}+ .mj-carousel-content .mj-carousel-{}-thumbnail-{}", self.extra.id, idx + 1, ext, self.extra.id, idx + 1))
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
                .add_selector(".mj-carousel-image img + div")
                .add_selector(".mj-carousel-thumbnail img + div")
                .add_content("display: none !important;")
                .to_string(),
        );
        style.push(
            (0..length)
                .fold(Style::default(), |res, idx| {
                    let ext = repeat(length - idx - 1, "+ * ");
                    res.add_selector(format!(
                        ".mj-carousel-{}-thumbnail:hover {}+ .mj-carousel-main .mj-carousel-image",
                        self.extra.id, ext
                    ))
                })
                .add_content("display: none !important;")
                .to_string(),
        );
        style.push(
            Style::default()
                .add_selector(".mj-carousel-thumbnail:hover")
                .add_content(format!(
                    "border-color: {} !important;",
                    self.attribute("tb-hover-border-color").unwrap()
                ))
                .to_string(),
        );
        style.push((0..length).fold(Style::default(), |res, idx| {
            let ext = repeat(length - idx - 1, "+ * ");
            res.add_selector(format!(".mj-carousel-{}-thumbnail-{}:hover {}+ .mj-carousel-main .mj-carousel-image-{}", self.extra.id, idx + 1, ext, idx + 1))
        }).add_content("display: block !important;").to_string());
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
            self.extra.id,
            self.extra.id,
            repeat(length - 1, "+ *"),
            self.extra.id
        ));
        Some(style.join("\n"))
    }
}

impl<'root> Render<'root> for Renderer<'root, MjCarousel, MjCarouselExtra> {
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

    fn raw_attribute(&self, key: &str) -> Option<&'root str> {
        self.element.attributes.get(key).map(|v| v.as_str())
    }

    fn tag(&self) -> Option<&str> {
        Some(NAME)
    }

    fn context(&self) -> &'root RenderContext<'root> {
        self.context
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

    fn render(&self, cursor: &mut RenderCursor) -> Result<(), Error> {
        cursor.header.maybe_add_style(self.render_style());

        let inner_div = self
            .set_style_carousel_div(Tag::div())
            .add_class("mj-carousel-content")
            .add_class(format!("mj-carousel-{}-content", self.extra.id));
        let div = Tag::div().add_class("mj-carousel");

        cursor.buffer.start_mso_negation_conditional_tag();
        div.render_open(&mut cursor.buffer)?;
        self.render_radios(cursor)?;
        inner_div.render_open(&mut cursor.buffer)?;
        self.render_thumbnails(cursor)?;
        self.render_carousel(cursor)?;
        inner_div.render_close(&mut cursor.buffer);
        div.render_close(&mut cursor.buffer);
        cursor.buffer.end_negation_conditional_tag();
        self.render_fallback(cursor)?;

        Ok(())
    }
}

impl<'render, 'root: 'render> Renderable<'render, 'root> for MjCarousel {
    fn renderer(
        &'root self,
        context: &'root RenderContext<'root>,
    ) -> Box<dyn Render<'root> + 'render> {
        let id = context.generator.next_id();
        Box::new(Renderer::new(context, self, MjCarouselExtra { id }))
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
