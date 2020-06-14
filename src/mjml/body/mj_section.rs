use super::BodyElement;
use crate::mjml::body::prelude::*;
use crate::mjml::error::Error;
use crate::mjml::prelude::*;
use crate::util::condition::*;
use crate::util::{suffix_css_classes, Context, Header, Size, Tag};
use crate::Options;
use log::debug;
use roxmltree::Node;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct MJSection {
    attributes: HashMap<String, String>,
    context: Option<Context>,
    children: Vec<BodyElement>,
}

impl MJSection {
    pub fn parse<'a, 'b>(node: &Node<'a, 'b>, opts: &Options) -> Result<MJSection, Error> {
        let mut children = vec![];
        for child in node.children() {
            children.push(BodyElement::parse(&child, opts, None)?);
        }
        Ok(MJSection {
            attributes: get_node_attributes(&node),
            context: None,
            children,
        })
    }

    fn get_background(&self) -> Option<String> {
        let mut res = vec![];
        if let Some(color) = self.get_attribute("background-color") {
            res.push(color);
        }
        if let Some(url) = self.get_attribute("background-url") {
            res.push(format!("url({})", url));
            // has default value
            res.push(format!(
                "top center / {}",
                self.get_attribute("background-size").unwrap()
            ));
            // has default value
            res.push(self.get_attribute("background-repeat").unwrap());
        }
        if res.len() > 0 {
            Some(res.join(" "))
        } else {
            None
        }
    }

    fn set_background_style(&self, tag: Tag) -> Tag {
        if self.get_attribute("background-url").is_some() {
            tag.maybe_set_style("background", self.get_background())
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
        self.attributes.contains_key("background-url")
    }

    fn is_full_width(&self) -> bool {
        self.attributes.contains_key("full-width")
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
        let mut res = vec![];
        res.push(START_CONDITIONAL_TAG.into());
        res.push(table.open());
        res.push(tr.open());
        res.push(td.open());
        res.push(END_CONDITIONAL_TAG.into());
        res.push(content);
        res.push(START_CONDITIONAL_TAG.into());
        res.push(td.close());
        res.push(tr.close());
        res.push(table.close());
        res.push(END_CONDITIONAL_TAG.into());
        res.join("")
    }

    fn render_full_width(&self, header: &Header) -> Result<String, Error> {
        let mut content: Vec<String> = vec![];
        content.push(self.render_wrap(self.render_section(header)?));
        let content = if self.has_background() {
            self.render_with_background(content.join(""))?
        } else {
            content.join("")
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
        let mut res = vec![];
        res.push(conditional_tag(tr.open()));
        for child in self.children.iter() {
            match child {
                BodyElement::Raw(element) => res.push(element.render(header)?),
                _ => {
                    let td = Tag::td()
                        .maybe_set_attribute("align", child.get_attribute("align"))
                        .maybe_set_class(suffix_css_classes(
                            child.get_attribute("css-class"),
                            "outlook",
                        ));
                    let td = child.set_style("td-outlook", td);
                    res.push(conditional_tag(td.open()));
                    res.push(child.render(header)?);
                    res.push(conditional_tag(td.close()));
                }
            }
        }
        res.push(conditional_tag(tr.close()));
        Ok(res.join(""))
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
        let has_bg = self.has_background();
        let mut res = vec![];
        res.push(div.open());
        if has_bg {
            res.push(inner_div.open());
        }
        res.push(table.open());
        res.push(tbody.open());
        res.push(tr.open());
        res.push(td.open());
        res.push(conditional_tag(inner_table.open()));
        // renderWrappedChildren()
        res.push(self.render_wrapped_children(header)?);
        //
        res.push(conditional_tag(inner_table.close()));
        res.push(td.close());
        res.push(tr.close());
        res.push(tbody.close());
        res.push(table.close());
        if has_bg {
            res.push(inner_div.close());
        }
        res.push(div.close());
        Ok(res.join(""))
    }

    fn render_with_background(&self, content: String) -> Result<String, Error> {
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
        let mut res = vec![];
        res.push(START_CONDITIONAL_TAG.into());
        res.push(vrect.open());
        res.push(vfill.closed());
        res.push(vtextbox.open());
        res.push(END_CONDITIONAL_TAG.into());
        res.push(content);
        res.push(START_CONDITIONAL_TAG.into());
        res.push(vtextbox.close());
        res.push(vrect.close());
        res.push(END_CONDITIONAL_TAG.into());
        Ok(res.join(""))
    }

    fn render_simple(&self, header: &Header) -> Result<String, Error> {
        let section = self.render_section(header)?;

        Ok(self.render_wrap(if self.has_background() {
            self.render_with_background(section)?
        } else {
            section
        }))
    }
}

impl Component for MJSection {
    fn update_header(&self, header: &mut Header) {
        for child in self.children.iter() {
            child.update_header(header);
        }
    }

    fn context(&self) -> Option<&Context> {
        self.context.as_ref()
    }

    fn set_context(&mut self, ctx: Context) {
        self.context = Some(ctx.clone());
        let child_base = Context::new(
            self.get_container_width(),
            self.get_siblings(),
            self.get_raw_siblings(),
            0,
        );
        for (idx, child) in self.children.iter_mut().enumerate() {
            child.set_context(child_base.clone().set_index(idx));
        }
    }

    fn render(&self, header: &Header) -> Result<String, Error> {
        if self.is_full_width() {
            self.render_full_width(header)
        } else {
            self.render_simple(header)
        }
    }
}

impl ComponentWithAttributes for MJSection {
    fn default_attribute(&self, key: &str) -> Option<String> {
        debug!("default_attribute {}", key);
        match key {
            "background-repeat" => Some("repeat".into()),
            "background-size" => Some("auto".into()),
            "direction" => Some("ltr".into()),
            "padding" => Some("20px 0".into()),
            "text-align" => Some("center".into()),
            "text-padding" => Some("4px 4px 4px 0".into()),
            _ => None,
        }
    }

    fn source_attributes(&self) -> Option<&HashMap<String, String>> {
        Some(&self.attributes)
    }
}

impl BodyComponent for MJSection {
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

impl ComponentWithChildren for MJSection {
    fn get_children(&self) -> &Vec<BodyElement> {
        &self.children
    }

    fn get_current_width(&self) -> Option<Size> {
        self.context().and_then(|ctx| ctx.container_width())
    }
}

impl BodyContainedComponent for MJSection {}

#[cfg(test)]
pub mod tests {
    use crate::tests::compare_render;

    #[test]
    fn with_body_width() {
        compare_render(
            include_str!("../../../test/mj-section-body-width.mjml"),
            include_str!("../../../test/mj-section-body-width.html"),
        );
    }

    #[test]
    fn base() {
        compare_render(
            include_str!("../../../test/mj-section.mjml"),
            include_str!("../../../test/mj-section.html"),
        );
    }

    #[test]
    fn with_background_color() {
        compare_render(
            include_str!("../../../test/mj-section-background-color.mjml"),
            include_str!("../../../test/mj-section-background-color.html"),
        );
    }

    #[test]
    fn with_background_url() {
        compare_render(
            include_str!("../../../test/mj-section-background-url.mjml"),
            include_str!("../../../test/mj-section-background-url.html"),
        );
    }

    #[test]
    fn with_background_url_full() {
        compare_render(
            include_str!("../../../test/mj-section-background-url-full.mjml"),
            include_str!("../../../test/mj-section-background-url-full.html"),
        );
    }

    #[test]
    fn with_border() {
        compare_render(
            include_str!("../../../test/mj-section-border.mjml"),
            include_str!("../../../test/mj-section-border.html"),
        );
    }

    #[test]
    fn with_border_radius() {
        compare_render(
            include_str!("../../../test/mj-section-border-radius.mjml"),
            include_str!("../../../test/mj-section-border-radius.html"),
        );
    }

    #[test]
    fn with_css_class() {
        compare_render(
            include_str!("../../../test/mj-section-class.mjml"),
            include_str!("../../../test/mj-section-class.html"),
        );
    }

    #[test]
    fn with_direction() {
        compare_render(
            include_str!("../../../test/mj-section-direction.mjml"),
            include_str!("../../../test/mj-section-direction.html"),
        );
    }

    #[test]
    fn with_full_width() {
        compare_render(
            include_str!("../../../test/mj-section-full-width.mjml"),
            include_str!("../../../test/mj-section-full-width.html"),
        );
    }

    #[test]
    fn with_padding() {
        compare_render(
            include_str!("../../../test/mj-section-padding.mjml"),
            include_str!("../../../test/mj-section-padding.html"),
        );
    }

    #[test]
    fn with_text_align() {
        compare_render(
            include_str!("../../../test/mj-section-text-align.mjml"),
            include_str!("../../../test/mj-section-text-align.html"),
        );
    }
}
