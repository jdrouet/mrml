use crate::elements::body::prelude::*;
use crate::elements::body::BodyElement;
use crate::elements::error::Error;
use crate::elements::prelude::*;
use crate::parser::Node;
use crate::util::attributes::*;
use crate::util::condition::*;
use crate::util::context::Context;
use crate::util::header::Header;
use crate::util::size::Size;
use crate::util::tag::Tag;

lazy_static! {
    static ref DEFAULT_ATTRIBUTES: Attributes = Attributes::default()
        .add("font-size", "13px")
        .add("padding", "16px");
}

#[derive(Clone, Debug)]
pub struct MJAccordionTitle {
    attributes: Attributes,
    context: Option<Context>,
    content: String,
}

impl MJAccordionTitle {
    fn default_attributes<'a>(node: &Node<'a>, header: &Header) -> Attributes {
        header
            .default_attributes
            .get_attributes(node, DEFAULT_ATTRIBUTES.clone())
    }

    pub fn parse<'a>(
        node: &Node<'a>,
        header: &Header,
        attrs: &Attributes,
    ) -> Result<MJAccordionTitle, Error> {
        if node.name.as_str() != "mj-accordion-title" {
            return Err(Error::UnexpectedElement(node.name.as_str().into()));
        }
        let content: String = node
            .children
            .iter()
            .filter_map(|child| child.as_text())
            .map(|value| value.as_str())
            .collect::<String>();
        let attributes = Self::default_attributes(node, header)
            .concat(attrs)
            .concat(node);
        Ok(MJAccordionTitle {
            attributes,
            context: None,
            content,
        })
    }

    pub fn new(attributes: Attributes) -> Self {
        MJAccordionTitle {
            attributes,
            context: None,
            content: "".into(),
        }
    }

    fn set_style_img(&self, tag: Tag) -> Tag {
        tag.set_style("display", "none")
            .maybe_set_style("height", self.get_attribute("icon-height"))
            .maybe_set_style("width", self.get_attribute("icon-width"))
    }

    fn render_title(&self) -> String {
        Tag::td()
            .maybe_set_style("background-color", self.get_attribute("background-color"))
            .maybe_set_style("color", self.get_attribute("color"))
            .maybe_set_style("font-size", self.get_attribute("font-size"))
            .maybe_set_style("font-family", self.get_attribute("font-family"))
            .maybe_set_style("padding", self.get_attribute("padding"))
            .maybe_set_style("padding-top", self.get_attribute("padding-top"))
            .maybe_set_style("padding-right", self.get_attribute("padding-right"))
            .maybe_set_style("padding-bottom", self.get_attribute("padding-bottom"))
            .maybe_set_style("padding-left", self.get_attribute("padding-left"))
            .set_style("width", "100%")
            .maybe_set_class(self.get_attribute("css-class"))
            .render(&self.content)
    }

    fn render_icons(&self) -> String {
        let img_more = self
            .set_style_img(Tag::new("img"))
            .maybe_set_attribute("src", self.get_attribute("icon-wrapped-url"))
            .maybe_set_attribute("alt", self.get_attribute("icon-wrapped-alt"))
            .set_class("mj-accordion-more")
            .closed();
        let img_less = self
            .set_style_img(Tag::new("img"))
            .maybe_set_attribute("src", self.get_attribute("icon-unwrapped-url"))
            .maybe_set_attribute("alt", self.get_attribute("icon-unwrapped-alt"))
            .set_class("mj-accordion-less")
            .closed();
        let td = Tag::new("td")
            .set_style("padding", "16px")
            .maybe_set_style("background", self.get_attribute("background-color"))
            .maybe_set_style("vertical-align", self.get_attribute("icon-align"))
            .set_class("mj-accordion-ico")
            .render(img_more + &img_less);
        negation_conditional_tag(td)
    }
}

impl Component for MJAccordionTitle {
    fn update_header(&self, header: &mut Header) {
        header.maybe_add_font_families(self.get_attribute("font-family"));
    }

    fn context(&self) -> Option<&Context> {
        self.context.as_ref()
    }

    fn set_context(&mut self, ctx: Context) {
        self.context = Some(ctx);
    }

    fn render(&self, _header: &Header) -> Result<String, Error> {
        let mut content = vec![self.render_title(), self.render_icons()];
        let icon_position = self
            .get_attribute("icon-position")
            .map(|value| value.as_str());
        if icon_position != Some("right") {
            content.reverse();
        }
        let content = content.join("");
        let tr = Tag::tr().render(content);
        let tbody = Tag::tbody().render(tr);
        let table = Tag::table()
            .set_attribute("cellspacing", 0)
            .set_attribute("cellpadding", 0)
            .set_style("width", "100%")
            .maybe_set_style("border-bottom", self.get_attribute("border"))
            .render(tbody);
        Ok(Tag::div().set_class("mj-accordion-title").render(table))
    }
}

impl BodyComponent for MJAccordionTitle {
    fn attributes(&self) -> Option<&Attributes> {
        Some(&self.attributes)
    }

    fn get_current_width(&self) -> Option<Size> {
        None
    }

    fn get_children(&self) -> &Vec<BodyElement> {
        &EMPTY_CHILDREN
    }
}
