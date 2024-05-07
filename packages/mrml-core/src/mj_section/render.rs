use std::borrow::Cow;
use std::convert::TryFrom;

use super::{MjSection, NAME};
use crate::helper::size::{Percent, Pixel};
use crate::prelude::render::*;

fn is_horizontal_position(value: &str) -> bool {
    value == "left" || value == "right" || value == "center"
}

fn is_vertical_position(value: &str) -> bool {
    value == "top" || value == "bottom" || value == "center"
}

pub trait WithMjSectionBackground<'root>: Render<'root> {
    fn has_background(&self) -> bool {
        self.attribute_exists("background-url")
    }

    fn parse_background_position<'a>(&'a self) -> (&'a str, &'a str)
    where
        'root: 'a,
    {
        // can be unwraped because has default value
        let position = self.attribute("background-position").unwrap();
        let mut positions = position.split_whitespace();
        if let Some(first) = positions.next() {
            if let Some(second) = positions.next() {
                if is_vertical_position(first) && is_horizontal_position(second) {
                    (second, first)
                } else {
                    (first, second)
                }
            } else if is_vertical_position(first) {
                ("center", first)
            } else {
                (first, "center")
            }
        } else {
            ("center", "top")
        }
    }

    fn get_background_position<'a>(&'a self) -> (&'a str, &'a str)
    where
        'root: 'a,
    {
        let (x, y) = self.parse_background_position();
        (
            self.attribute("background-position-x").unwrap_or(x),
            self.attribute("background-position-y").unwrap_or(y),
        )
    }

    fn get_background_position_str(&self) -> String {
        let position = self.get_background_position();
        format!("{} {}", position.0, position.1)
    }

    fn get_background(&self) -> Option<String> {
        let mut res: Vec<Cow<'_, str>> = vec![];
        if let Some(color) = self.attribute("background-color") {
            res.push(color.into());
        }
        if let Some(url) = self.attribute("background-url") {
            res.push(format!("url('{url}')").into());
            // has default value
            res.push(
                format!(
                    "{} / {}",
                    self.get_background_position_str(),
                    self.attribute("background-size").unwrap()
                )
                .into(),
            );
            // has default value
            res.push(self.attribute("background-repeat").unwrap().into());
        }

        if res.is_empty() {
            None
        } else {
            Some(res.join(" "))
        }
    }

    fn set_background_style<'a, 't>(&'a self, tag: Tag<'t>) -> Tag<'t>
    where
        'root: 'a,
        'a: 't,
    {
        if self.has_background() {
            tag.maybe_add_style("background", self.get_background())
                .add_style("background-position", self.get_background_position_str())
                .maybe_add_style("background-repeat", self.attribute("background-repeat"))
                .maybe_add_style("background-size", self.attribute("background-size"))
        } else {
            tag.maybe_add_style("background", self.attribute("background-color"))
                .maybe_add_style("background-color", self.attribute("background-color"))
        }
    }

    fn get_vfill_position(&self) -> (Cow<'root, str>, Cow<'root, str>) {
        if self.attribute_equals("background-size", "auto") {
            return ("0.5, 0".into(), "0.5, 0".into());
        }
        let (bg_position_x, bg_position_y) = self.get_background_position();
        let bg_repeat = self.attribute_equals("background-repeat", "repeat");
        let bg_position_x = match bg_position_x {
            "left" => "0%",
            "center" => "50%",
            "right" => "100%",
            _ => {
                if bg_position_x.ends_with('%') {
                    bg_position_x
                } else {
                    "50%"
                }
            }
        };
        let bg_position_y = match bg_position_y {
            "top" => "0%",
            "center" => "50%",
            "bottom" => "100%",
            _ => {
                if bg_position_y.ends_with('%') {
                    bg_position_y
                } else {
                    "0%"
                }
            }
        };
        let position_x = if let Ok(position) = Percent::try_from(bg_position_x) {
            if bg_repeat {
                position.value() * 0.01
            } else {
                (position.value() - 50.0) * 0.01
            }
        } else if bg_repeat {
            0.5
        } else {
            0.0
        };
        let position_y = if let Ok(position) = Percent::try_from(bg_position_y) {
            if bg_repeat {
                position.value() * 0.01
            } else {
                (position.value() - 50.0) * 0.01
            }
        } else if bg_repeat {
            0.5
        } else {
            0.0
        };
        (
            format!("{position_x}, {position_y}").into(),
            format!("{position_x}, {position_y}").into(),
        )
    }

    fn get_vfill_tag<'a>(&'a self) -> Tag<'a>
    where
        'root: 'a,
    {
        let bg_no_repeat = self.attribute_equals("background-repeat", "no-repeat");
        let bg_size = self.attribute("background-size");
        let bg_size_auto = bg_size
            .as_ref()
            .map(|value| *value == "auto")
            .unwrap_or(false);
        let vml_type = if bg_no_repeat && !bg_size_auto {
            "frame"
        } else {
            "tile"
        };
        let vsize = match bg_size {
            Some("cover") | Some("contain") => Some("1,1".to_string()),
            Some("auto") => None,
            Some(value) => Some(value.replace(' ', ",")),
            None => None,
        };
        let aspect = match bg_size {
            Some("cover") => Some("atleast".to_string()),
            Some("contain") => Some("atmost".to_string()),
            Some("auto") => None,
            Some(other) => {
                if other.split(' ').count() == 1 {
                    Some("atmost".to_string())
                } else {
                    None
                }
            }
            None => None,
        };

        let (vfill_position, vfill_origin) = self.get_vfill_position();
        Tag::new("v:fill")
            .add_attribute("position", vfill_position)
            .add_attribute("origin", vfill_origin)
            .maybe_add_attribute("src", self.attribute("background-url"))
            .maybe_add_attribute("color", self.attribute("background-color"))
            .maybe_add_attribute("size", vsize)
            .add_attribute("type", vml_type)
            .maybe_add_attribute("aspect", aspect)
    }
}

pub trait SectionLikeRender<'root>: WithMjSectionBackground<'root> {
    fn container_width(&self) -> &Option<Pixel>;
    fn children(&self) -> &Vec<crate::mj_body::MjBodyChild>;

    fn is_full_width(&self) -> bool {
        self.attribute_exists("full-width")
    }

    fn render_with_background<F>(&self, cursor: &mut RenderCursor, content: F) -> Result<(), Error>
    where
        F: Fn(&mut RenderCursor) -> Result<(), Error>,
    {
        let full_width = self.is_full_width();
        let vrect = Tag::new("v:rect")
            .maybe_add_attribute(
                "mso-width-percent",
                if full_width { Some("1000") } else { None },
            )
            .maybe_add_style(
                "width",
                if full_width {
                    None
                } else {
                    self.container_width().as_ref().map(|v| v.to_string())
                },
            )
            .add_attribute("xmlns:v", "urn:schemas-microsoft-com:vml")
            .add_attribute("fill", "true")
            .add_attribute("stroke", "false");
        let vfill = self.get_vfill_tag();
        let vtextbox = Tag::new("v:textbox")
            .add_attribute("inset", "0,0,0,0")
            .add_style("mso-fit-shape-to-text", "true");

        vrect.render_open(&mut cursor.buffer)?;
        vfill.render_closed(&mut cursor.buffer)?;
        vtextbox.render_open(&mut cursor.buffer)?;
        cursor.buffer.end_conditional_tag();
        content(cursor)?;
        cursor.buffer.start_conditional_tag();
        vtextbox.render_close(&mut cursor.buffer);
        vrect.render_close(&mut cursor.buffer);

        Ok(())
    }

    fn set_style_section_div<'a, 't>(&'a self, tag: Tag<'t>) -> Tag<'t>
    where
        'root: 'a,
        'a: 't,
    {
        let base = if self.is_full_width() {
            tag
        } else {
            self.set_background_style(tag)
        };
        base.add_style("margin", "0px auto")
            .maybe_add_style("border-radius", self.attribute("border-radius"))
            .maybe_add_style(
                "max-width",
                self.container_width().as_ref().map(|item| item.to_string()),
            )
    }

    fn render_wrap<F>(&self, cursor: &mut RenderCursor, content: F) -> Result<(), Error>
    where
        F: Fn(&mut RenderCursor) -> Result<(), Error>,
    {
        let table = Tag::table_presentation()
            .maybe_add_attribute("bgcolor", self.attribute("background-color"))
            .add_attribute("align", "center")
            .maybe_add_attribute(
                "width",
                self.container_width()
                    .as_ref()
                    .map(|p| p.value().to_string()),
            )
            .maybe_add_style(
                "width",
                self.container_width().as_ref().map(|v| v.to_string()),
            )
            .maybe_add_suffixed_class(self.attribute("css-class"), "outlook");
        let tr = Tag::tr();
        let td = Tag::td()
            .add_style("line-height", "0px")
            .add_style("font-size", "0px")
            .add_style("mso-line-height-rule", "exactly");

        cursor.buffer.start_conditional_tag();
        table.render_open(&mut cursor.buffer)?;
        tr.render_open(&mut cursor.buffer)?;
        td.render_open(&mut cursor.buffer)?;
        content(cursor)?;
        td.render_close(&mut cursor.buffer);
        tr.render_close(&mut cursor.buffer);
        table.render_close(&mut cursor.buffer);
        cursor.buffer.end_conditional_tag();

        Ok(())
    }

    fn get_siblings(&self) -> usize {
        self.children().len()
    }

    fn get_raw_siblings(&self) -> usize {
        self.children().iter().filter(|elt| elt.is_raw()).count()
    }

    fn render_wrapped_children(&self, cursor: &mut RenderCursor) -> Result<(), Error> {
        let siblings = self.get_siblings();
        let raw_siblings = self.get_raw_siblings();
        let tr = Tag::tr();

        tr.render_open(&mut cursor.buffer)?;
        for child in self.children().iter() {
            let mut renderer = child.renderer(self.context());
            renderer.set_siblings(siblings);
            renderer.set_raw_siblings(raw_siblings);
            renderer.set_container_width(self.container_width().clone());
            if child.is_raw() {
                cursor.buffer.end_conditional_tag();
                renderer.render(cursor)?;
                cursor.buffer.start_conditional_tag();
            } else {
                let td = renderer
                    .set_style("td-outlook", Tag::td())
                    .maybe_add_attribute("align", renderer.attribute("align"))
                    .maybe_add_suffixed_class(renderer.attribute("css-class"), "outlook");
                td.render_open(&mut cursor.buffer)?;
                cursor.buffer.end_conditional_tag();
                renderer.render(cursor)?;
                cursor.buffer.start_conditional_tag();
                td.render_close(&mut cursor.buffer);
            }
        }
        tr.render_close(&mut cursor.buffer);
        Ok(())
    }

    fn set_style_section_inner_div<'t>(&self, tag: Tag<'t>) -> Tag<'t> {
        tag.add_style("line-height", "0")
            .add_style("font-size", "0")
    }

    fn set_style_section_table<'a, 't>(&'a self, tag: Tag<'t>) -> Tag<'t>
    where
        'root: 'a,
        'a: 't,
    {
        let base = if self.is_full_width() {
            tag
        } else {
            self.set_background_style(tag)
        };
        base.add_style("width", "100%")
            .maybe_add_style("border-radius", self.attribute("border-radius"))
    }

    fn set_style_section_td<'a, 't>(&'a self, tag: Tag<'t>) -> Tag<'t>
    where
        'root: 'a,
        'a: 't,
    {
        tag.maybe_add_style("border", self.attribute("border"))
            .maybe_add_style("border-bottom", self.attribute("border-bottom"))
            .maybe_add_style("border-left", self.attribute("border-left"))
            .maybe_add_style("border-right", self.attribute("border-right"))
            .maybe_add_style("border-top", self.attribute("border-top"))
            .maybe_add_style("direction", self.attribute("direction"))
            .add_style("font-size", "0px")
            .maybe_add_style("padding", self.attribute("padding"))
            .maybe_add_style("padding-bottom", self.attribute("padding-bottom"))
            .maybe_add_style("padding-left", self.attribute("padding-left"))
            .maybe_add_style("padding-right", self.attribute("padding-right"))
            .maybe_add_style("padding-top", self.attribute("padding-top"))
            .maybe_add_style("text-align", self.attribute("text-align"))
    }

    fn render_section(&self, cursor: &mut RenderCursor) -> Result<(), Error> {
        let is_full_width = self.is_full_width();
        let div = self
            .set_style_section_div(Tag::div())
            .maybe_add_class(if is_full_width {
                None
            } else {
                self.attribute("css-class")
            });
        let inner_div = self.set_style_section_inner_div(Tag::div());
        let table = self.set_style_section_table(
            Tag::table_presentation()
                .add_attribute("align", "center")
                .maybe_add_attribute(
                    "background",
                    if is_full_width {
                        None
                    } else {
                        self.attribute("background-url")
                    },
                ),
        );
        let tbody = Tag::tbody();
        let tr = Tag::tr();
        let td = self.set_style_section_td(Tag::td());
        let inner_table = Tag::table_presentation();

        let has_bg = self.has_background();
        div.render_open(&mut cursor.buffer)?;
        if has_bg {
            inner_div.render_open(&mut cursor.buffer)?;
        }
        table.render_open(&mut cursor.buffer)?;
        tbody.render_open(&mut cursor.buffer)?;
        tr.render_open(&mut cursor.buffer)?;
        td.render_open(&mut cursor.buffer)?;
        cursor.buffer.start_conditional_tag();
        inner_table.render_open(&mut cursor.buffer)?;
        self.render_wrapped_children(cursor)?;
        inner_table.render_close(&mut cursor.buffer);
        cursor.buffer.end_conditional_tag();
        td.render_close(&mut cursor.buffer);
        tr.render_close(&mut cursor.buffer);
        tbody.render_close(&mut cursor.buffer);
        table.render_close(&mut cursor.buffer);
        if has_bg {
            inner_div.render_close(&mut cursor.buffer);
        }
        div.render_close(&mut cursor.buffer);

        Ok(())
    }

    fn set_style_table_full_width<'a, 't>(&'a self, tag: Tag<'t>) -> Tag<'t>
    where
        'root: 'a,
        'a: 't,
    {
        let base = if self.is_full_width() {
            self.set_background_style(tag)
        } else {
            tag
        };
        base.maybe_add_style("border-radius", self.attribute("border-radius"))
            .add_style("width", "100%")
    }

    fn get_full_width_table<'a>(&'a self) -> Tag<'a>
    where
        'root: 'a,
    {
        self.set_style_table_full_width(Tag::table_presentation())
            .add_attribute("align", "center")
            .maybe_add_class(self.attribute("css-class"))
            .maybe_add_attribute("background", self.attribute("background-url"))
    }

    fn render_full_width(&self, cursor: &mut RenderCursor) -> Result<(), Error> {
        let table = self.get_full_width_table();
        let tbody = Tag::tbody();
        let tr = Tag::tr();
        let td = Tag::td();

        table.render_open(&mut cursor.buffer)?;
        tbody.render_open(&mut cursor.buffer)?;
        tr.render_open(&mut cursor.buffer)?;
        td.render_open(&mut cursor.buffer)?;
        //
        if self.has_background() {
            self.render_with_background(cursor, |cursor| {
                self.render_wrap(cursor, |cursor| {
                    cursor.buffer.end_conditional_tag();
                    self.render_section(cursor)?;
                    cursor.buffer.start_conditional_tag();
                    Ok(())
                })
            })?;
        } else {
            self.render_wrap(cursor, |cursor| {
                cursor.buffer.end_conditional_tag();
                self.render_section(cursor)?;
                cursor.buffer.start_conditional_tag();
                Ok(())
            })?;
        }
        //
        td.render_close(&mut cursor.buffer);
        tr.render_close(&mut cursor.buffer);
        tbody.render_close(&mut cursor.buffer);
        table.render_close(&mut cursor.buffer);

        Ok(())
    }

    fn render_simple(&self, cursor: &mut RenderCursor) -> Result<(), Error> {
        self.render_wrap(cursor, |cursor| {
            if self.has_background() {
                self.render_with_background(cursor, |cursor| self.render_section(cursor))?;
            } else {
                cursor.buffer.end_conditional_tag();
                self.render_section(cursor)?;
                cursor.buffer.start_conditional_tag();
            }
            Ok(())
        })
    }
}

impl<'root> WithMjSectionBackground<'root> for Renderer<'root, MjSection, ()> {}
impl<'root> SectionLikeRender<'root> for Renderer<'root, MjSection, ()> {
    fn children(&self) -> &Vec<crate::mj_body::MjBodyChild> {
        &self.element.children
    }

    fn container_width(&self) -> &Option<Pixel> {
        &self.container_width
    }
}

impl<'root> Render<'root> for Renderer<'root, MjSection, ()> {
    fn default_attribute(&self, name: &str) -> Option<&'static str> {
        match name {
            "background-position" => Some("top center"),
            "background-repeat" => Some("repeat"),
            "background-size" => Some("auto"),
            "direction" => Some("ltr"),
            "padding" => Some("20px 0"),
            "text-align" => Some("center"),
            "text-padding" => Some("4px 4px 4px 0"),
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

    fn set_container_width(&mut self, width: Option<Pixel>) {
        self.container_width = width;
    }

    fn render(&self, cursor: &mut RenderCursor) -> Result<(), Error> {
        if self.is_full_width() {
            self.render_full_width(cursor)
        } else {
            self.render_simple(cursor)
        }
    }
}

impl<'render, 'root: 'render> Renderable<'render, 'root> for MjSection {
    fn renderer(
        &'root self,
        context: &'root RenderContext<'root>,
    ) -> Box<dyn Render<'root> + 'render> {
        Box::new(Renderer::new(context, self, ()))
    }
}

#[cfg(test)]
mod tests {
    // error reported in https://github.com/jdrouet/mrml/issues/370
    crate::should_render!(comment, "comment");

    crate::should_render!(basic, "mj-section");
    crate::should_render!(background_color, "mj-section-background-color");
    crate::should_render!(background_url_full, "mj-section-background-url-full");
    crate::should_render!(background_url, "mj-section-background-url");
    crate::should_render!(body_width, "mj-section-body-width");
    crate::should_render!(border, "mj-section-border");
    crate::should_render!(border_radius, "mj-section-border-radius");
    crate::should_render!(class, "mj-section-class");
    crate::should_render!(direction, "mj-section-direction");
    crate::should_render!(full_width, "mj-section-full-width");
    crate::should_render!(padding, "mj-section-padding");
    crate::should_render!(text_align, "mj-section-text-align");
}
