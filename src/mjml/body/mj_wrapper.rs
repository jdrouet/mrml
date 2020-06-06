use super::BodyElement;
use crate::mjml::body::prelude::*;
use crate::mjml::error::Error;
use crate::mjml::prelude::*;
use crate::util::condition::*;
use crate::util::prelude::*;
use crate::util::{suffix_css_classes, Context, Header, Size, Style, Tag};
use crate::Options;
use log::debug;
use roxmltree::Node;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct MJWrapper {
    attributes: HashMap<String, String>,
    context: Option<Context>,
    children: Vec<BodyElement>,
}

impl MJWrapper {
    pub fn parse<'a, 'b>(node: Node<'a, 'b>, opts: &Options) -> Result<MJWrapper, Error> {
        let mut children = vec![];
        for child in node.children() {
            children.push(BodyElement::parse(child, opts, None)?);
        }
        Ok(MJWrapper {
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

    fn get_background_style(&self) -> Style {
        let mut res = Style::new();
        if self.get_attribute("background-url").is_some() {
            res.maybe_set("background", self.get_background());
        } else {
            res.maybe_set("background", self.get_attribute("background-color"));
            res.maybe_set("background-color", self.get_attribute("background-color"));
        }
        res
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
            .maybe_set_style("width", self.get_container_width())
            .maybe_set_attribute("width", self.get_container_width_value())
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
            .maybe_set_attribute("background", self.get_attribute("background-url"))
            .maybe_set_class(self.get_attribute("css-class"))
            .insert_style(self.get_style("table-full-width").inner());
        Ok(table.render(Tag::tbody().render(Tag::tr().render(Tag::td().render(content)))))
    }

    fn render_wrapped_children(&self, header: &Header) -> Result<String, Error> {
        let container_width = self.get_container_width();
        let tr = Tag::tr();
        let mut res = vec![];
        for child in self.children.iter() {
            match child {
                BodyElement::Raw(element) => res.push(element.render(header)?),
                _ => {
                    let td = Tag::td()
                        .maybe_set_attribute("align", child.get_attribute("align"))
                        .maybe_set_class(suffix_css_classes(
                            child.get_attribute("css-class"),
                            "outlook",
                        ))
                        .maybe_set_attribute("width", container_width.clone())
                        .insert_style(child.get_style("td-outlook").inner());
                    res.push(START_CONDITIONAL_TAG.into());
                    res.push(tr.open());
                    res.push(td.open());
                    res.push(END_CONDITIONAL_TAG.into());
                    res.push(child.render(header)?);
                    res.push(START_CONDITIONAL_TAG.into());
                    res.push(td.close());
                    res.push(tr.close());
                    res.push(END_CONDITIONAL_TAG.into());
                }
            }
        }
        Ok(res.join(""))
    }

    fn render_section(&self, header: &Header) -> Result<String, Error> {
        let has_bg = self.has_background();
        let div = Tag::div()
            .maybe_set_class(if self.is_full_width() {
                None
            } else {
                self.get_attribute("css-class")
            })
            .insert_style(self.get_style("div").inner());
        let inner_div = Tag::div().insert_style(self.get_style("inner-div").inner());
        let table = Tag::table_presentation()
            .set_attribute("align", "center")
            .maybe_set_attribute("background", if self.is_full_width() {
                None
            } else {
                self.get_attribute("background-url")
            })
            .insert_style(self.get_style("table").inner());
        let tbody = Tag::tbody();
        let tr = Tag::tr();
        let td = Tag::td().insert_style(self.get_style("td").inner());
        let inner_table = Tag::table_presentation();
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
        res.push(self.render_wrapped_children(header)?);
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

impl Component for MJWrapper {
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
        let sibling = self.get_siblings();
        let raw_sibling = self.get_raw_siblings();
        let container_width = self.get_box_widths();
        for (idx, child) in self.children.iter_mut().enumerate() {
            let mut child_ctx =
                Context::from(&ctx, container_width.clone(), sibling, raw_sibling, idx);
            child_ctx.set("index", idx);
            child.set_context(child_ctx);
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

impl ComponentWithAttributes for MJWrapper {
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

impl BodyComponent for MJWrapper {
    fn get_style(&self, name: &str) -> Style {
        let mut res = Style::new();
        let bg_style = self.get_background_style();
        match name {
            "div" => {
                if !self.is_full_width() {
                    res.merge(&bg_style);
                }
                res.set("margin", "0px auto");
                res.maybe_set("border-radius", self.get_attribute("border-radius"));
                res.maybe_set("max-width", self.get_container_width_str());
            }
            "inner-div" => {
                res.set("line-height", 0);
                res.set("font-size", 0);
            }
            "table-full-width" => {
                if self.is_full_width() {
                    res.merge(&bg_style);
                }
                res.maybe_set("border-radius", self.get_attribute("border-radius"));
                res.set("width", "100%");
            }
            "table" => {
                if !self.is_full_width() {
                    res.merge(&bg_style);
                }
                res.maybe_set("border-radius", self.get_attribute("border-radius"));
                res.set("width", "100%");
            }
            "td" => {
                res.maybe_set("border", self.get_attribute("border"));
                res.maybe_set("border-bottom", self.get_attribute("border-bottom"));
                res.maybe_set("border-left", self.get_attribute("border-left"));
                res.maybe_set("border-right", self.get_attribute("border-right"));
                res.maybe_set("border-top", self.get_attribute("border-top"));
                res.maybe_set("direction", self.get_attribute("direction"));
                res.set("font-size", "0px");
                res.maybe_set("padding", self.get_attribute("padding"));
                res.maybe_set("padding-bottom", self.get_attribute("padding-bottom"));
                res.maybe_set("padding-left", self.get_attribute("padding-left"));
                res.maybe_set("padding-right", self.get_attribute("padding-right"));
                res.maybe_set("padding-top", self.get_attribute("padding-top"));
                res.maybe_set("text-align", self.get_attribute("text-align"));
            }
            _ => (),
        };
        res
    }
}

impl ComponentWithChildren for MJWrapper {
    fn get_children(&self) -> &Vec<BodyElement> {
        &self.children
    }

    fn get_current_width(&self) -> Option<Size> {
        self.context().and_then(|ctx| ctx.container_width())
    }
}

impl BodyContainedComponent for MJWrapper {}
impl ComponentWithSizeAttribute for MJWrapper {}
impl BodyComponentWithBorder for MJWrapper {}
impl BodyComponentWithPadding for MJWrapper {}
impl BodyComponentWithBoxWidths for MJWrapper {}

#[cfg(test)]
pub mod tests {
    use crate::tests::compare_render;

    #[test]
    fn base() {
        compare_render(
            include_str!("../../../test/mj-wrapper.mjml"),
            include_str!("../../../test/mj-wrapper.html"),
        );
    }

    #[test]
    fn with_background() {
        compare_render(
            include_str!("../../../test/mj-wrapper-background.mjml"),
            include_str!("../../../test/mj-wrapper-background.html"),
        );
    }

    #[test]
    fn with_border() {
        compare_render(
            include_str!("../../../test/mj-wrapper-border.mjml"),
            include_str!("../../../test/mj-wrapper-border.html"),
        );
    }

    #[test]
    fn with_padding() {
        compare_render(
            include_str!("../../../test/mj-wrapper-padding.mjml"),
            include_str!("../../../test/mj-wrapper-padding.html"),
        );
    }

    #[test]
    fn with_other() {
        compare_render(
            include_str!("../../../test/mj-wrapper-other.mjml"),
            include_str!("../../../test/mj-wrapper-other.html"),
        );
    }
}
