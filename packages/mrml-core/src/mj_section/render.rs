use std::cell::{Ref, RefCell};
use std::convert::TryFrom;
use std::rc::Rc;

use super::{MjSection, NAME};
use crate::helper::condition::{conditional_tag, END_CONDITIONAL_TAG, START_CONDITIONAL_TAG};
use crate::helper::size::{Percent, Pixel};
use crate::helper::tag::Tag;
use crate::prelude::hash::Map;
use crate::prelude::render::{Error, Header, Render, RenderOptions, Renderable};

fn is_horizontal_position(value: &str) -> bool {
    value == "left" || value == "right" || value == "center"
}

fn is_vertical_position(value: &str) -> bool {
    value == "top" || value == "bottom" || value == "center"
}

pub trait WithMjSectionBackground<'h>: Render<'h> {
    fn has_background(&self) -> bool {
        self.attribute_exists("background-url")
    }

    fn parse_background_position(&self) -> (String, String) {
        // can be unwraped because has default value
        let position = self.attribute("background-position").unwrap();
        let positions = position.split_whitespace().collect::<Vec<_>>();
        let first = positions.first();
        let second = positions.get(1);
        if let Some(first) = first {
            if let Some(second) = second {
                if is_vertical_position(first) && is_horizontal_position(second) {
                    (second.to_string(), first.to_string())
                } else {
                    (first.to_string(), second.to_string())
                }
            } else if is_vertical_position(first) {
                ("center".to_string(), first.to_string())
            } else {
                (first.to_string(), "center".to_string())
            }
        } else {
            ("center".to_string(), "top".to_string())
        }
    }

    fn get_background_position(&self) -> (String, String) {
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
        let mut res = vec![];
        if let Some(color) = self.attribute("background-color") {
            res.push(color);
        }
        if let Some(url) = self.attribute("background-url") {
            res.push(format!("url('{url}')"));
            // has default value
            res.push(format!(
                "{} / {}",
                self.get_background_position_str(),
                self.attribute("background-size").unwrap()
            ));
            // has default value
            res.push(self.attribute("background-repeat").unwrap());
        }

        if res.is_empty() {
            None
        } else {
            Some(res.join(" "))
        }
    }

    fn set_background_style(&self, tag: Tag) -> Tag {
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

    fn get_vfill_position(&self) -> (String, String) {
        if self.attribute_equals("background-size", "auto") {
            return ("0.5, 0".to_string(), "0.5, 0".to_string());
        }
        let (bg_position_x, bg_position_y) = self.get_background_position();
        let bg_repeat = self.attribute_equals("background-repeat", "repeat");
        let bg_position_x = match bg_position_x.as_str() {
            "left" => "0%".to_string(),
            "center" => "50%".to_string(),
            "right" => "100%".to_string(),
            _ => {
                if bg_position_x.ends_with('%') {
                    bg_position_x
                } else {
                    "50%".to_string()
                }
            }
        };
        let bg_position_y = match bg_position_y.as_str() {
            "top" => "0%".to_string(),
            "center" => "50%".to_string(),
            "bottom" => "100%".to_string(),
            _ => {
                if bg_position_y.ends_with('%') {
                    bg_position_y
                } else {
                    "0%".to_string()
                }
            }
        };
        let position_x = if let Ok(position) = Percent::try_from(bg_position_x.as_str()) {
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
        let position_y = if let Ok(position) = Percent::try_from(bg_position_y.as_str()) {
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
            format!("{position_x}, {position_y}"),
            format!("{position_x}, {position_y}"),
        )
    }

    fn get_vfill_tag(&self) -> Tag {
        let bg_no_repeat = self.attribute_equals("background-repeat", "no-repeat");
        let bg_size = self.attribute("background-size");
        let bg_size_auto = bg_size
            .as_ref()
            .map(|value| value == "auto")
            .unwrap_or(false);
        let vml_type = if bg_no_repeat && !bg_size_auto {
            "frame"
        } else {
            "tile"
        };
        let vsize = match bg_size.as_deref() {
            Some("cover") | Some("contain") => Some("1,1".to_string()),
            Some("auto") => None,
            Some(value) => Some(value.replace(' ', ",")),
            None => None,
        };
        let aspect = match bg_size.as_deref() {
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

pub trait SectionLikeRender<'h>: WithMjSectionBackground<'h> {
    fn clone_header(&self) -> Rc<RefCell<Header<'h>>>;
    fn container_width(&self) -> &Option<Pixel>;
    fn children(&self) -> &Vec<crate::mj_body::MjBodyChild>;

    fn is_full_width(&self) -> bool {
        self.attribute_exists("full-width")
    }

    fn render_with_background<T: AsRef<str>>(&self, content: T) -> String {
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
        let before = vrect.open() + &vfill.closed() + &vtextbox.open();
        let after = vtextbox.close() + &vrect.close();
        before + END_CONDITIONAL_TAG + content.as_ref() + START_CONDITIONAL_TAG + &after
    }

    fn set_style_section_div(&self, tag: Tag) -> Tag {
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

    fn render_wrap<T: AsRef<str>>(&self, content: T) -> String {
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
        let before = table.open() + &tr.open() + &td.open();
        let after = td.close() + &tr.close() + &table.close();
        conditional_tag(before + content.as_ref() + &after)
    }

    fn get_siblings(&self) -> usize {
        self.children().len()
    }

    fn get_raw_siblings(&self) -> usize {
        self.children().iter().filter(|elt| elt.is_raw()).count()
    }

    fn render_wrapped_children(&self, opts: &RenderOptions) -> Result<String, Error> {
        let siblings = self.get_siblings();
        let raw_siblings = self.get_raw_siblings();
        let tr = Tag::tr();
        let mut result = tr.open();
        let children = self.children();
        if !children.is_empty() {
            for child in self.children().iter() {
                let mut renderer = child.renderer(self.clone_header());
                renderer.set_siblings(siblings);
                renderer.set_raw_siblings(raw_siblings);
                renderer.set_container_width(self.container_width().clone());
                if child.is_raw() {
                    result.push_str(END_CONDITIONAL_TAG);
                    result.push_str(&renderer.render(opts)?);
                    result.push_str(START_CONDITIONAL_TAG);
                } else {
                    let td = renderer
                        .set_style("td-outlook", Tag::td())
                        .maybe_add_attribute("align", renderer.attribute("align"))
                        .maybe_add_suffixed_class(renderer.attribute("css-class"), "outlook");
                    result.push_str(&td.open());
                    result.push_str(END_CONDITIONAL_TAG);
                    result.push_str(&renderer.render(opts)?);
                    result.push_str(START_CONDITIONAL_TAG);
                    result.push_str(&td.close());
                }
            }
        }
        result.push_str(&tr.close());
        Ok(result)
    }

    fn set_style_section_inner_div(&self, tag: Tag) -> Tag {
        tag.add_style("line-height", "0")
            .add_style("font-size", "0")
    }

    fn set_style_section_table(&self, tag: Tag) -> Tag {
        let base = if self.is_full_width() {
            tag
        } else {
            self.set_background_style(tag)
        };
        base.add_style("width", "100%")
            .maybe_add_style("border-radius", self.attribute("border-radius"))
    }

    fn set_style_section_td(&self, tag: Tag) -> Tag {
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

    fn render_section(&self, opts: &RenderOptions) -> Result<String, Error> {
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

        let content = self.render_wrapped_children(opts)?;
        let content = conditional_tag(inner_table.open() + &content + &inner_table.close());
        let content = td.render(content);
        let content = tr.render(content);
        let content = tbody.render(content);
        let content = table.render(content);
        Ok(div.render(if self.has_background() {
            inner_div.render(content)
        } else {
            content
        }))
    }

    fn set_style_table_full_width(&self, tag: Tag) -> Tag {
        let base = if self.is_full_width() {
            self.set_background_style(tag)
        } else {
            tag
        };
        base.maybe_add_style("border-radius", self.attribute("border-radius"))
            .add_style("width", "100%")
    }

    fn get_full_width_table(&self) -> Tag {
        self.set_style_table_full_width(Tag::table_presentation())
            .add_attribute("align", "center")
            .maybe_add_class(self.attribute("css-class"))
            .maybe_add_attribute("background", self.attribute("background-url"))
    }

    fn render_full_width(&self, opts: &RenderOptions) -> Result<String, Error> {
        let table = self.get_full_width_table();
        let tbody = Tag::tbody();
        let tr = Tag::tr();
        let td = Tag::td();
        let content = self.render_section(opts)?;
        let content =
            self.render_wrap(END_CONDITIONAL_TAG.to_string() + &content + START_CONDITIONAL_TAG);
        let content = if self.has_background() {
            self.render_with_background(content)
        } else {
            content
        };
        Ok(table.render(tbody.render(tr.render(td.render(content)))))
    }

    fn render_simple(&self, opts: &RenderOptions) -> Result<String, Error> {
        let section = self.render_section(opts)?;

        let section = if self.has_background() {
            self.render_with_background(section)
        } else {
            END_CONDITIONAL_TAG.to_string() + &section + START_CONDITIONAL_TAG
        };
        Ok(self.render_wrap(section))
    }
}

struct MjSectionRender<'e, 'h> {
    header: Rc<RefCell<Header<'h>>>,
    element: &'e MjSection,
    container_width: Option<Pixel>,
}

impl<'e, 'h> WithMjSectionBackground<'h> for MjSectionRender<'e, 'h> {}
impl<'e, 'h> SectionLikeRender<'h> for MjSectionRender<'e, 'h> {
    fn clone_header(&self) -> Rc<RefCell<Header<'h>>> {
        Rc::clone(&self.header)
    }

    fn children(&self) -> &Vec<crate::mj_body::MjBodyChild> {
        &self.element.children
    }

    fn container_width(&self) -> &Option<Pixel> {
        &self.container_width
    }
}

impl<'e, 'h> Render<'h> for MjSectionRender<'e, 'h> {
    fn default_attribute(&self, name: &str) -> Option<&str> {
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

    fn render(&self, opts: &RenderOptions) -> Result<String, Error> {
        if self.is_full_width() {
            self.render_full_width(opts)
        } else {
            self.render_simple(opts)
        }
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MjSection {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'h> + 'r> {
        Box::new(MjSectionRender::<'e, 'h> {
            element: self,
            header,
            container_width: None,
        })
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
