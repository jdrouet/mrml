use super::BodyElement;
use crate::elements::body::prelude::*;
use crate::elements::error::Error;
use crate::elements::prelude::*;
use crate::parser::Node;
use crate::util::attributes::*;
use crate::util::condition::*;
use crate::util::{Context, Header, Size, Tag};
use std::collections::HashMap;

lazy_static! {
    static ref DEFAULT_ATTRIBUTES: Attributes = Attributes::new()
        .add("background-color", "#ffffff")
        .add("background-position", "center center")
        .add("height", "0px")
        .add("mode", "fixed-height")
        .add("padding", "0px")
        .add("vertical-align", "top");
}

#[derive(Clone, Debug)]
pub struct MJHero {
    attributes: Attributes,
    context: Option<Context>,
    children: Vec<BodyElement>,
}

impl MJHero {
    fn default_attributes<'a>(node: &Node<'a>, header: &Header) -> Attributes {
        header
            .default_attributes()
            .get_attributes(node, DEFAULT_ATTRIBUTES.clone())
    }

    pub fn parse<'a>(node: &Node<'a>, header: &Header) -> Result<MJHero, Error> {
        let mut children = vec![];
        for child in node.children.iter() {
            children.push(BodyElement::parse(&child, header, None::<&Attributes>)?);
        }
        Ok(MJHero {
            attributes: Self::default_attributes(node, header).concat(node),
            context: None,
            children,
        })
    }

    fn set_style_div(&self, tag: Tag) -> Tag {
        tag.set_style("margin", "0 auto")
            .maybe_set_style("max-width", self.get_container_width_str())
    }

    fn set_style_table(&self, tag: Tag) -> Tag {
        tag.set_style("width", "100%")
    }

    fn set_style_tr(&self, tag: Tag) -> Tag {
        tag.set_style("vertical-align", "top")
    }

    fn set_style_td_fluid(&self, tag: Tag) -> Tag {
        let bg_ratio = self
            .get_size_attribute("background-height")
            .and_then(|height| {
                self.get_size_attribute("background-width")
                    .and_then(|width| Some(height.value() * 100.0 / width.value()))
            });
        tag.set_style("mso-padding-bottom-alt", "0")
            .maybe_set_style("padding-bottom", bg_ratio)
            .set_style("width", "0.01%")
    }

    fn set_style_outlook_table(&self, tag: Tag) -> Tag {
        tag.maybe_set_style("width", self.get_container_width())
    }

    fn set_style_outlook_inner_table(&self, tag: Tag) -> Tag {
        self.set_style_outlook_table(tag)
    }

    fn set_style_outlook_inner_td(&self, tag: Tag) -> Tag {
        tag.maybe_set_style(
            "background-color",
            self.get_attribute("inner-background-color"),
        )
        .maybe_set_style("padding", self.get_attribute("inner-padding"))
        .maybe_set_style("padding-top", self.get_attribute("inner-padding-top"))
        .maybe_set_style("padding-right", self.get_attribute("inner-padding-right"))
        .maybe_set_style("padding-bottom", self.get_attribute("inner-padding-bottom"))
        .maybe_set_style("padding-left", self.get_attribute("inner-padding-left"))
    }

    fn set_style_inner_div(&self, tag: Tag) -> Tag {
        tag.maybe_set_style(
            "background-color",
            self.get_attribute("inner-background-color"),
        )
        .maybe_set_style("float", self.get_attribute("align"))
        .set_style("margin", "0px auto")
        .maybe_set_style("width", self.get_attribute("width"))
    }

    fn set_style_inner_table(&self, tag: Tag) -> Tag {
        tag.set_style("width", "100%").set_style("margin", "0px")
    }

    fn set_style_outlook_image(&self, tag: Tag) -> Tag {
        tag.set_style("border", "0")
            .maybe_set_style("height", self.get_attribute("background-height"))
            .set_style("mso-position-horizontal", "center")
            .set_style("position", "absolute")
            .set_style("top", "0")
            .maybe_set_style(
                "width",
                self.get_attribute("background-width")
                    .and_then(|value| Some(value.to_string()))
                    .or_else(|| self.get_container_width_str()),
            )
            .set_style("z-index", "-3")
    }

    fn set_style_outlook_td(&self, tag: Tag) -> Tag {
        tag.set_style("line-height", "0")
            .set_style("font-size", "0")
            .set_style("mso-line-height-rule", "exactly")
    }

    fn get_background(&self) -> Option<String> {
        let bg_color = self.get_attribute("background-color");
        match self.get_attribute("background-url") {
            Some(url) => Some(format!(
                "{} url({}) no-repeat {} / cover",
                // has default value
                self.get_attribute("background-color").unwrap(),
                url,
                // has default value
                self.get_attribute("background-position").unwrap()
            )),
            None => bg_color.cloned(),
        }
    }

    fn set_style_hero(&self, tag: Tag) -> Tag {
        tag.maybe_set_style("background", self.get_background())
            .maybe_set_style(
                "background-position",
                self.get_attribute("background-position"),
            )
            .set_style("background-repeat", "no-repeat")
            .maybe_set_style("padding", self.get_attribute("padding"))
            .maybe_set_style("padding-top", self.get_attribute("padding-top"))
            .maybe_set_style("padding-right", self.get_attribute("padding-right"))
            .maybe_set_style("padding-bottom", self.get_attribute("padding-bottom"))
            .maybe_set_style("padding-left", self.get_attribute("padding-left"))
            .maybe_set_style("vertical-align", self.get_attribute("vertical-align"))
    }

    fn render_child(&self, header: &Header, child: &BodyElement) -> Result<String, Error> {
        let tr = Tag::new("tr");
        let td = Tag::new("td")
            .maybe_set_style(
                "background",
                child.get_attribute("container-background-color"),
            )
            .set_style("font-size", "0px")
            .maybe_set_style("padding", child.get_attribute("padding"))
            .maybe_set_style("padding-top", child.get_attribute("padding-top"))
            .maybe_set_style("padding-right", child.get_attribute("padding-right"))
            .maybe_set_style("padding-bottom", child.get_attribute("padding-bottom"))
            .maybe_set_style("padding-left", child.get_attribute("padding-left"))
            .set_style("word-break", "break-word")
            .maybe_set_attribute("align", child.get_attribute("align"))
            .maybe_set_attribute(
                "background",
                child.get_attribute("container-background-color"),
            )
            .maybe_set_attribute("class", child.get_attribute("css-class"));
        Ok(tr.render(td.render(child.render(header)?)))
    }

    fn render_children(&self, header: &Header) -> Result<String, Error> {
        let mut res = String::from("");
        for child in self.get_children().iter() {
            let result = match child {
                BodyElement::Raw(_) => child.render(header)?,
                _ => self.render_child(header, child)?,
            };
            res.push_str(result.as_str());
        }
        Ok(res)
    }

    fn render_content(&self, header: &Header) -> Result<String, Error> {
        let table = self
            .set_style_outlook_inner_table(Tag::new("table"))
            .maybe_set_attribute("align", self.get_attribute("align"))
            .set_attribute("border", 0)
            .set_attribute("cellpadding", 0)
            .set_attribute("cellspacing", 0)
            .maybe_set_attribute("width", self.get_container_width_value());
        let tr = Tag::tr();
        let outlook_inner_td = self.set_style_outlook_inner_td(Tag::td());
        let outlook_inner_div = self
            .set_style_inner_div(Tag::new("div"))
            .maybe_set_attribute("width", self.get_attribute("align"))
            .set_class("mj-hero-content");
        let inner_table = self.set_style_inner_table(Tag::table_presentation());
        let mut res = vec![];
        res.push(START_CONDITIONAL_TAG.into());
        res.push(table.open());
        res.push(tr.open());
        res.push(outlook_inner_td.open());
        res.push(END_CONDITIONAL_TAG.into());
        res.push(outlook_inner_div.render(inner_table.render(
            tr.render(Tag::td().render(inner_table.render(self.render_children(header)?))),
        )));
        res.push(START_CONDITIONAL_TAG.into());
        res.push(outlook_inner_td.close());
        res.push(tr.close());
        res.push(table.close());
        res.push(END_CONDITIONAL_TAG.into());
        Ok(res.join(""))
    }

    fn render_mode_fluid(&self, header: &Header) -> Result<String, Error> {
        let td_fluid = self.set_style_td_fluid(Tag::td());
        let td = self
            .set_style_hero(Tag::td())
            .maybe_set_attribute("background", self.get_attribute("background-url"));
        let mut res = vec![];
        res.push(td_fluid.closed());
        res.push(td.render(self.render_content(header)?));
        res.push(td_fluid.closed());
        Ok(res.join(""))
    }

    fn render_mode_fixed(&self, header: &Header) -> Result<String, Error> {
        // has a default value
        let height = self.get_size_attribute("height").unwrap();
        let padding = match self.get_padding_vertical() {
            Some(value) => value.value(),
            None => 0.0,
        };
        let height = Size::Pixel(height.value() - padding);
        let td = self
            .set_style_hero(Tag::td())
            .maybe_set_attribute("background", self.get_attribute("background-url"))
            .set_attribute("height", height.value());
        Ok(td.render(self.render_content(header)?))
    }

    fn render_mode(&self, header: &Header) -> Result<String, Error> {
        match self.get_attribute("mode") {
            Some(mode) => match mode.as_str() {
                "fluid" => self.render_mode_fluid(header),
                _ => self.render_mode_fixed(header),
            },
            None => self.render_mode_fixed(header),
        }
    }
}

impl Component for MJHero {
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
        let outlook_table = self
            .set_style_outlook_table(Tag::table_presentation())
            .set_attribute("align", "center")
            .maybe_set_attribute("width", self.get_container_width_value());
        let outlook_tr = Tag::tr();
        let outlook_td = self.set_style_outlook_td(Tag::td());
        let v_image = self
            .set_style_outlook_image(Tag::new("v:image"))
            .maybe_set_attribute("src", self.get_attribute("background-url"))
            .set_attribute("xmlns:v", "urn:schemas-microsoft-com:vml");
        let div = self
            .set_style_div(Tag::div())
            .maybe_set_attribute("align", self.get_attribute("align"))
            .maybe_set_class(self.get_attribute("css-class"));
        let table = self.set_style_table(Tag::table_presentation());
        let tr = self.set_style_tr(Tag::tr());
        let mut res = vec![];
        res.push(START_CONDITIONAL_TAG.into());
        res.push(outlook_table.open());
        res.push(outlook_tr.open());
        res.push(outlook_td.open());
        res.push(v_image.closed());
        res.push(END_CONDITIONAL_TAG.into());
        res.push(div.open());
        res.push(table.open());
        res.push(tr.open());
        res.push(self.render_mode(header)?);
        res.push(tr.close());
        res.push(table.close());
        res.push(div.close());
        res.push(START_CONDITIONAL_TAG.into());
        res.push(outlook_td.close());
        res.push(outlook_tr.close());
        res.push(outlook_table.close());
        res.push(END_CONDITIONAL_TAG.into());
        Ok(res.join(""))
    }
}

impl ComponentWithAttributes for MJHero {
    fn attributes(&self) -> Option<&HashMap<String, String>> {
        Some(self.attributes.inner())
    }
}

impl BodyComponent for MJHero {
    fn set_style(&self, name: &str, tag: Tag) -> Tag {
        match name {
            "div" => self.set_style_div(tag),
            "table" => self.set_style_table(tag),
            "tr" => self.set_style_tr(tag),
            "td-fluid" => self.set_style_td_fluid(tag),
            "hero" => self.set_style_hero(tag),
            "outlook-table" => self.set_style_outlook_table(tag),
            "outlook-td" => self.set_style_outlook_td(tag),
            "outlook-image" => self.set_style_outlook_image(tag),
            "outlook-inner-table" => self.set_style_outlook_inner_table(tag),
            // "outlook-inner-td" => self.get_style_outlook_inner_td(),
            "inner-div" => self.set_style_inner_div(tag),
            "inner-table" => self.set_style_inner_table(tag),
            _ => tag,
        }
    }
}

impl ComponentWithChildren for MJHero {
    fn get_children(&self) -> &Vec<BodyElement> {
        &self.children
    }

    fn get_current_width(&self) -> Option<Size> {
        self.context().and_then(|ctx| ctx.container_width())
    }
}

impl BodyContainedComponent for MJHero {}
impl ComponentWithSizeAttribute for MJHero {}
impl BodyComponentWithPadding for MJHero {}

#[cfg(test)]
pub mod tests {
    use crate::tests::compare_render;

    #[test]
    fn base() {
        compare_render(
            include_str!("../../../test/mj-hero.mjml"),
            include_str!("../../../test/mj-hero.html"),
        );
    }

    #[test]
    fn with_background_color() {
        compare_render(
            include_str!("../../../test/mj-hero-background-color.mjml"),
            include_str!("../../../test/mj-hero-background-color.html"),
        );
    }

    #[test]
    fn with_background_height() {
        compare_render(
            include_str!("../../../test/mj-hero-background-height.mjml"),
            include_str!("../../../test/mj-hero-background-height.html"),
        );
    }

    #[test]
    fn with_background_position() {
        compare_render(
            include_str!("../../../test/mj-hero-background-position.mjml"),
            include_str!("../../../test/mj-hero-background-position.html"),
        );
    }

    #[test]
    fn with_background_url() {
        compare_render(
            include_str!("../../../test/mj-hero-background-url.mjml"),
            include_str!("../../../test/mj-hero-background-url.html"),
        );
    }

    #[test]
    fn with_background_width() {
        compare_render(
            include_str!("../../../test/mj-hero-background-width.mjml"),
            include_str!("../../../test/mj-hero-background-width.html"),
        );
    }

    #[test]
    fn with_class() {
        compare_render(
            include_str!("../../../test/mj-hero-class.mjml"),
            include_str!("../../../test/mj-hero-class.html"),
        );
    }

    #[test]
    fn with_height() {
        compare_render(
            include_str!("../../../test/mj-hero-height.mjml"),
            include_str!("../../../test/mj-hero-height.html"),
        );
    }

    #[test]
    fn with_mode() {
        compare_render(
            include_str!("../../../test/mj-hero-mode.mjml"),
            include_str!("../../../test/mj-hero-mode.html"),
        );
    }

    #[test]
    fn with_vertical_align() {
        compare_render(
            include_str!("../../../test/mj-hero-vertical-align.mjml"),
            include_str!("../../../test/mj-hero-vertical-align.html"),
        );
    }

    #[test]
    fn with_width() {
        compare_render(
            include_str!("../../../test/mj-hero-width.mjml"),
            include_str!("../../../test/mj-hero-width.html"),
        );
    }
}
