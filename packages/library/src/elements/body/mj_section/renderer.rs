use super::{MJSection, DEFAULT_BACKGROUND_POSITION};
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
use crate::util::tag::Tag;

impl MJSection {
    fn get_background_position(&self) -> String {
        let positions = self
            .get_attribute("background-position")
            .map(|value| value.split_whitespace().collect::<Vec<_>>());
        let first = positions.as_ref().and_then(|list| list.get(0));
        let second = positions.as_ref().and_then(|list| list.get(1));
        if let Some(first) = first {
            if let Some(second) = second {
                if first == &"top"
                    || first == &"bottom"
                    || (first == &"center" && (second == &"left" || second == &"right"))
                {
                    format!("{} {}", second, first)
                } else {
                    format!("{} {}", first, second)
                }
            } else if first == &"top" || first == &"bottom" {
                format!("center {}", first)
            } else {
                format!("{} center", first)
            }
        } else {
            String::from(DEFAULT_BACKGROUND_POSITION)
        }
    }

    fn get_background(&self) -> Option<String> {
        let mut res: Vec<String> = vec![];
        if let Some(color) = self.get_attribute("background-color") {
            res.push(color.to_string());
        }
        if let Some(url) = self.get_attribute("background-url") {
            res.push(format!("url({})", url));
            // has default value
            res.push(format!(
                "{} / {}",
                self.get_background_position(),
                self.get_attribute("background-size").unwrap()
            ));
            // has default value
            res.push(self.get_attribute("background-repeat").unwrap().to_string());
        }
        if res.is_empty() {
            None
        } else {
            Some(res.join(" "))
        }
    }

    fn set_background_style(&self, tag: Tag) -> Tag {
        if self.get_attribute("background-url").is_some() {
            tag.maybe_set_style("background", self.get_background())
                .set_style("background-position", self.get_background_position())
                .maybe_set_style("background-repeat", self.get_attribute("background-repeat"))
                .maybe_set_style("background-size", self.get_attribute("background-size"))
        } else {
            tag.maybe_set_style("background", self.get_attribute("background-color"))
                .maybe_set_style("background-color", self.get_attribute("background-color"))
        }
    }

    fn set_style_div(&self, tag: Tag) -> Tag {
        let base = if self.is_full_width() {
            tag
        } else {
            self.set_background_style(tag)
        };
        base.set_style("margin", "0px auto")
            .maybe_set_style("border-radius", self.get_attribute("border-radius"))
            .maybe_set_style("max-width", self.get_container_width())
    }

    fn set_style_inner_div(&self, tag: Tag) -> Tag {
        tag.set_style("line-height", 0).set_style("font-size", 0)
    }

    fn set_style_table_full_width(&self, tag: Tag) -> Tag {
        let base = if self.is_full_width() {
            self.set_background_style(tag)
        } else {
            tag
        };
        base.maybe_set_style("border-radius", self.get_attribute("border-radius"))
            .set_style("width", "100%")
    }

    fn set_style_table(&self, tag: Tag) -> Tag {
        let base = if self.is_full_width() {
            tag
        } else {
            self.set_background_style(tag)
        };
        base.maybe_set_style("border-radius", self.get_attribute("border-radius"))
            .set_style("width", "100%")
    }

    fn set_style_td(&self, tag: Tag) -> Tag {
        tag.maybe_set_style("border", self.get_attribute("border"))
            .maybe_set_style("border-bottom", self.get_attribute("border-bottom"))
            .maybe_set_style("border-left", self.get_attribute("border-left"))
            .maybe_set_style("border-right", self.get_attribute("border-right"))
            .maybe_set_style("border-top", self.get_attribute("border-top"))
            .maybe_set_style("direction", self.get_attribute("direction"))
            .set_style("font-size", "0px")
            .maybe_set_style("padding", self.get_attribute("padding"))
            .maybe_set_style("padding-bottom", self.get_attribute("padding-bottom"))
            .maybe_set_style("padding-left", self.get_attribute("padding-left"))
            .maybe_set_style("padding-right", self.get_attribute("padding-right"))
            .maybe_set_style("padding-top", self.get_attribute("padding-top"))
            .maybe_set_style("text-align", self.get_attribute("text-align"))
    }

    fn has_background(&self) -> bool {
        self.attributes.has("background-url")
    }

    fn is_full_width(&self) -> bool {
        self.attributes.has("full-width")
    }

    fn render_wrap(&self, content: String) -> String {
        let table = Tag::table_borderless()
            .set_attribute("align", "center")
            .maybe_set_attribute("width", self.get_container_width_value())
            .maybe_set_style("width", self.get_container_width())
            .maybe_set_class(suffix_css_classes(
                self.get_attribute("css-class"),
                "outlook",
            ));
        let tr = Tag::tr();
        let td = Tag::td()
            .set_style("font-size", "0px")
            .set_style("line-height", "0px")
            .set_style("mso-line-height-rule", "exactly");
        let before = conditional_tag(table.open() + &tr.open() + &td.open());
        let after = conditional_tag(td.close() + &tr.close() + &table.close());
        before + &content + &after
    }

    fn render_full_width(&self, header: &Header) -> Result<String, Error> {
        let content = self.render_wrap(self.render_section(header)?);
        let content = if self.has_background() {
            self.render_with_background(content)
        } else {
            content
        };
        let table = Tag::table_presentation()
            .set_attribute("align", "center")
            .maybe_set_class(self.get_attribute("css-class"))
            .maybe_set_attribute("background", self.get_attribute("background-url"));
        let table = self.set_style_table_full_width(table);
        Ok(table.render(Tag::tbody().render(Tag::tr().render(Tag::td().render(content)))))
    }

    fn render_wrapped_children(&self, header: &Header) -> Result<String, Error> {
        let tr = Tag::tr();
        let content = self
            .get_children()
            .try_fold(String::default(), |res, child| {
                if child.is_raw() {
                    Ok(res + &child.render(header)?)
                } else {
                    let td = Tag::td()
                        .maybe_set_attribute("align", child.get_attribute("align"))
                        .maybe_set_class(suffix_css_classes(
                            child.get_attribute("css-class"),
                            "outlook",
                        ));
                    let td = child.set_style("td-outlook", td);
                    Ok(res
                        + &conditional_tag(td.open())
                        + &child.render(header)?
                        + &conditional_tag(td.close()))
                }
            })?;
        Ok(conditional_tag(tr.open()) + &content + &conditional_tag(tr.close()))
    }

    fn render_section(&self, header: &Header) -> Result<String, Error> {
        let div = Tag::div().maybe_set_class(if self.is_full_width() {
            None
        } else {
            self.get_attribute("css-class")
        });
        let div = self.set_style_div(div);
        let inner_div = self.set_style_inner_div(Tag::div());
        let table = Tag::table_presentation()
            .set_attribute("align", "center")
            .maybe_set_attribute(
                "background",
                if self.is_full_width() {
                    None
                } else {
                    self.get_attribute("background-url")
                },
            );
        let table = self.set_style_table(table);
        let tbody = Tag::tbody();
        let tr = Tag::tr();
        let td = self.set_style_td(Tag::td());
        let inner_table = Tag::table_presentation();
        let content = conditional_tag(inner_table.open())
            + &self.render_wrapped_children(header)?
            + &conditional_tag(inner_table.close());
        let content = table.render(tbody.render(tr.render(td.render(content))));
        Ok(div.render(if self.has_background() {
            inner_div.render(content)
        } else {
            content
        }))
    }

    fn render_with_background(&self, content: String) -> String {
        let full_width = self.is_full_width();
        let vrect = Tag::new("v:rect")
            .maybe_set_attribute(
                "mso-width-percent",
                if full_width { Some(1000) } else { None },
            )
            .maybe_set_style(
                "width",
                if full_width {
                    None
                } else {
                    self.get_container_width()
                },
            )
            .set_attribute("xmlns:v", "urn:schemas-microsoft-com:vml")
            .set_attribute("fill", "true")
            .set_attribute("stroke", "false");
        let vfill = Tag::new("v:fill")
            .set_attribute("origin", "0.5, 0")
            .set_attribute("position", "0.5, 0")
            .maybe_set_attribute("src", self.get_attribute("background-url"))
            .maybe_set_attribute("color", self.get_attribute("background-color"))
            .set_attribute("type", "tile");
        let vtextbox = Tag::new("v:textbox")
            .set_attribute("inset", "0,0,0,0")
            .set_style("mso-fit-shape-to-text", "true");
        let before = conditional_tag(vrect.open() + &vfill.closed() + &vtextbox.open());
        let after = conditional_tag(vtextbox.close() + &vrect.close());
        before + &content + &after
    }

    fn render_simple(&self, header: &Header) -> Result<String, Error> {
        let section = self.render_section(header)?;

        Ok(self.render_wrap(if self.has_background() {
            self.render_with_background(section)
        } else {
            section
        }))
    }
}

impl Component for MJSection {
    fn update_header(&self, header: &mut Header) {
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
        if self.is_full_width() {
            self.render_full_width(header)
        } else {
            self.render_simple(header)
        }
    }
}

impl BodyComponent for MJSection {
    fn attributes(&self) -> Option<&Attributes> {
        Some(&self.attributes)
    }

    fn get_children(&self) -> BodyComponentChildIterator {
        to_children_iterator(&self.children)
    }

    fn get_children_len(&self) -> usize {
        self.children.len()
    }

    fn get_current_width(&self) -> Option<Size> {
        self.context().and_then(|ctx| ctx.container_width())
    }

    fn set_style(&self, name: &str, tag: Tag) -> Tag {
        match name {
            "div" => self.set_style_div(tag),
            "inner-div" => self.set_style_inner_div(tag),
            "table-full-width" => self.set_style_table_full_width(tag),
            "table" => self.set_style_table(tag),
            "td" => self.set_style_td(tag),
            _ => tag,
        }
    }
}
