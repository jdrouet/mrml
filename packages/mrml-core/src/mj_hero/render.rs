use std::cell::{Ref, RefCell};
use std::rc::Rc;

use super::{MjHero, NAME};
use crate::helper::condition::conditional_tag;
use crate::helper::size::Pixel;
use crate::helper::tag::Tag;
use crate::prelude::hash::Map;
use crate::prelude::render::{Error, Header, Render, RenderOptions, Renderable};

struct MjHeroRender<'e, 'h> {
    header: Rc<RefCell<Header<'h>>>,
    element: &'e MjHero,
    container_width: Option<Pixel>,
    siblings: usize,
    raw_siblings: usize,
}

impl<'e, 'h> MjHeroRender<'e, 'h> {
    fn set_style_div(&self, tag: Tag) -> Tag {
        tag.add_style("margin", "0 auto").maybe_add_style(
            "max-width",
            self.container_width.as_ref().map(|w| w.to_string()),
        )
    }

    fn set_style_table(&self, tag: Tag) -> Tag {
        tag.add_style("width", "100%")
    }

    fn set_style_tr(&self, tag: Tag) -> Tag {
        tag.add_style("vertical-align", "top")
    }

    fn set_style_td_fluid(&self, tag: Tag) -> Tag {
        // TODO check size type compatibility
        let bg_ratio = self
            .attribute_as_size("background-height")
            .and_then(|height| {
                self.attribute_as_size("background-width")
                    .map(|width| height.value() * 100.0 / width.value())
            });
        tag.add_style("mso-padding-bottom-alt", "0")
            .maybe_add_style("padding-bottom", bg_ratio.map(|v| v.to_string()))
            .add_style("width", "0.01%")
    }

    fn set_style_outlook_table(&self, tag: Tag) -> Tag {
        tag.maybe_add_style(
            "width",
            self.container_width.as_ref().map(|w| w.to_string()),
        )
    }

    fn set_style_outlook_inner_table(&self, tag: Tag) -> Tag {
        self.set_style_outlook_table(tag)
    }

    fn set_style_outlook_inner_td(&self, tag: Tag) -> Tag {
        tag.maybe_add_style("background-color", self.attribute("inner-background-color"))
            .maybe_add_style("padding", self.attribute("inner-padding"))
            .maybe_add_style("padding-top", self.attribute("inner-padding-top"))
            .maybe_add_style("padding-right", self.attribute("inner-padding-right"))
            .maybe_add_style("padding-bottom", self.attribute("inner-padding-bottom"))
            .maybe_add_style("padding-left", self.attribute("inner-padding-left"))
    }

    fn set_style_inner_div(&self, tag: Tag) -> Tag {
        tag.maybe_add_style("background-color", self.attribute("inner-background-color"))
            .maybe_add_style("float", self.attribute("align"))
            .add_style("margin", "0px auto")
            .maybe_add_style("width", self.attribute("width"))
    }

    fn set_style_inner_table(&self, tag: Tag) -> Tag {
        tag.add_style("width", "100%").add_style("margin", "0px")
    }

    fn set_style_outlook_image(&self, tag: Tag) -> Tag {
        tag.add_style("border", "0")
            .maybe_add_style("height", self.attribute("background-height"))
            .add_style("mso-position-horizontal", "center")
            .add_style("position", "absolute")
            .add_style("top", "0")
            .maybe_add_style(
                "width",
                self.attribute("background-width")
                    .or_else(|| self.container_width.as_ref().map(|w| w.to_string())),
            )
            .add_style("z-index", "-3")
    }

    fn set_style_outlook_td(&self, tag: Tag) -> Tag {
        tag.add_style("line-height", "0")
            .add_style("font-size", "0")
            .add_style("mso-line-height-rule", "exactly")
    }

    fn get_background(&self) -> Option<String> {
        self.attribute("background-url")
            .map(|url| {
                format!(
                    "{} url('{}') no-repeat {} / cover",
                    // has default value
                    self.attribute("background-color").unwrap(),
                    url,
                    // has default value
                    self.attribute("background-position").unwrap()
                )
            })
            .or_else(|| self.attribute("background-color"))
    }

    fn set_style_hero(&self, tag: Tag) -> Tag {
        tag.maybe_add_style("background", self.get_background())
            .maybe_add_style("background-position", self.attribute("background-position"))
            .add_style("background-repeat", "no-repeat")
            .maybe_add_style("padding", self.attribute("padding"))
            .maybe_add_style("padding-top", self.attribute("padding-top"))
            .maybe_add_style("padding-right", self.attribute("padding-right"))
            .maybe_add_style("padding-bottom", self.attribute("padding-bottom"))
            .maybe_add_style("padding-left", self.attribute("padding-left"))
            .maybe_add_style("vertical-align", self.attribute("vertical-align"))
    }

    fn render_children(&self, opts: &RenderOptions) -> Result<String, Error> {
        let siblings = self.element.children.len();
        let raw_siblings = self.element.children.iter().filter(|c| c.is_raw()).count();
        self.element.children.iter().enumerate().try_fold(
            String::default(),
            |res, (index, child)| {
                let mut renderer = child.renderer(Rc::clone(&self.header));
                renderer.set_index(index);
                renderer.set_siblings(siblings);
                renderer.set_raw_siblings(raw_siblings);
                let result = if child.is_raw() {
                    renderer.render(opts)?
                } else {
                    let tr = Tag::tr();
                    let td = Tag::td()
                        .maybe_add_style(
                            "background",
                            renderer.attribute("container-background-color"),
                        )
                        .add_style("font-size", "0px")
                        .maybe_add_style("padding", renderer.attribute("padding"))
                        .maybe_add_style("padding-top", renderer.attribute("padding-top"))
                        .maybe_add_style("padding-right", renderer.attribute("padding-right"))
                        .maybe_add_style("padding-bottom", renderer.attribute("padding-bottom"))
                        .maybe_add_style("padding-left", renderer.attribute("padding-left"))
                        .add_style("word-break", "break-word")
                        .maybe_add_attribute("align", renderer.attribute("align"))
                        .maybe_add_attribute(
                            "background",
                            renderer.attribute("container-background-color"),
                        )
                        .maybe_add_attribute("class", renderer.attribute("css-class"));
                    tr.render(td.render(renderer.render(opts)?))
                };
                Ok(res + &result)
            },
        )
    }

    fn render_content(&self, opts: &RenderOptions) -> Result<String, Error> {
        let table = self
            .set_style_outlook_inner_table(Tag::table_borderless())
            .maybe_add_attribute("align", self.attribute("align"))
            .maybe_add_attribute(
                "width",
                self.container_width.as_ref().map(|w| w.value().to_string()),
            );
        let tr = Tag::tr();
        let outlook_inner_td = self.set_style_outlook_inner_td(Tag::td());
        let outlook_inner_div = self
            .set_style_inner_div(Tag::div())
            .maybe_add_attribute("width", self.attribute("align"))
            .add_class("mj-hero-content");
        let inner_table = self.set_style_inner_table(Tag::table_presentation());
        let content = outlook_inner_div.render(inner_table.render(Tag::tbody().render(tr.render(
            Tag::td().render(inner_table.render(Tag::tbody().render(self.render_children(opts)?))),
        ))));
        let before = conditional_tag(table.open() + &tr.open() + &outlook_inner_td.open());
        let after = conditional_tag(outlook_inner_td.close() + &tr.close() + &table.close());
        Ok(before + &content + &after)
    }

    fn render_mode_fluid(&self, opts: &RenderOptions) -> Result<String, Error> {
        let td_fluid = self.set_style_td_fluid(Tag::td());
        let td = self
            .set_style_hero(Tag::td())
            .maybe_add_attribute("background", self.attribute("background-url"));
        Ok(td_fluid.closed() + &td.render(self.render_content(opts)?) + &td_fluid.closed())
    }

    fn render_mode_fixed(&self, opts: &RenderOptions) -> Result<String, Error> {
        // has a default value
        let height = self.attribute_as_pixel("height").unwrap().value();
        let padding = self.get_padding_vertical().value();
        let height = height - padding;
        let td = self
            .set_style_hero(Tag::td())
            .add_style("height", format!("{height}px"))
            .maybe_add_attribute("background", self.attribute("background-url"))
            .add_attribute("height", height.to_string());
        Ok(td.render(self.render_content(opts)?))
    }

    fn render_mode(&self, opts: &RenderOptions) -> Result<String, Error> {
        if let Some(ref mode) = self.attribute("mode") {
            if mode == "fluid" {
                return self.render_mode_fluid(opts);
            }
        }
        self.render_mode_fixed(opts)
    }
}

impl<'e, 'h> Render<'h> for MjHeroRender<'e, 'h> {
    fn default_attribute(&self, name: &str) -> Option<&str> {
        match name {
            "background-color" => Some("#ffffff"),
            "background-position" => Some("center center"),
            "height" => Some("0px"),
            "mode" => Some("fixed-height"),
            "padding" => Some("0px"),
            "vertical-align" => Some("top"),
            _ => None,
        }
    }

    fn attributes(&self) -> Option<&Map<String, String>> {
        Some(&self.element.attributes)
    }

    fn tag(&self) -> Option<&str> {
        Some(NAME)
    }

    fn header(&self) -> Ref<Header<'h>> {
        self.header.borrow()
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

    fn render(&self, opts: &RenderOptions) -> Result<String, Error> {
        let outlook_table = self
            .set_style_outlook_table(Tag::table_presentation())
            .add_attribute("align", "center")
            .maybe_add_attribute(
                "width",
                self.container_width.as_ref().map(|v| v.value().to_string()),
            );
        let outlook_tr = Tag::tr();
        let outlook_td = self.set_style_outlook_td(Tag::td());
        let v_image = self
            .set_style_outlook_image(Tag::new("v:image"))
            .maybe_add_attribute("src", self.attribute("background-url"))
            .add_attribute("xmlns:v", "urn:schemas-microsoft-com:vml");
        let div = self
            .set_style_div(Tag::div())
            .maybe_add_attribute("align", self.attribute("align"))
            .maybe_add_class(self.attribute("css-class"));
        let table = self.set_style_table(Tag::table_presentation());
        let tr = self.set_style_tr(Tag::tr());
        let content = self.render_mode(opts)?;
        let content = div.render(table.render(Tag::tbody().render(tr.render(content))));
        let before = conditional_tag(
            outlook_table.open() + &outlook_tr.open() + &outlook_td.open() + &v_image.closed(),
        );
        let after =
            conditional_tag(outlook_td.close() + &outlook_tr.close() + &outlook_table.close());
        Ok(before + &content + &after)
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MjHero {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'h> + 'r> {
        Box::new(MjHeroRender::<'e, 'h> {
            element: self,
            header,
            container_width: None,
            siblings: 1,
            raw_siblings: 0,
        })
    }
}

#[cfg(test)]
mod tests {
    crate::should_render!(basic, "mj-hero");
    crate::should_render!(background_color, "mj-hero-background-color");
    crate::should_render!(background_height, "mj-hero-background-height");
    crate::should_render!(background_position, "mj-hero-background-position");
    crate::should_render!(background_url, "mj-hero-background-url");
    crate::should_render!(background_width, "mj-hero-background-width");
    crate::should_render!(class, "mj-hero-class");
    crate::should_render!(height, "mj-hero-height");
    crate::should_render!(mode, "mj-hero-mode");
    crate::should_render!(vertical_align, "mj-hero-vertical-align");
    crate::should_render!(width, "mj-hero-width");
}
