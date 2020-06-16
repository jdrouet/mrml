use crate::mjml::body::prelude::*;
use crate::mjml::error::Error;
use crate::mjml::prelude::*;
use crate::util::attributes::*;
use crate::util::{Context, Header, Size, Tag};
use roxmltree::Node;
use std::collections::HashMap;

const IMAGE_ORIGIN: &'static str = "https://www.mailjet.com/images/theme/v1/icons/ico-social/";

#[derive(Clone, Debug)]
struct SocialNetwork {
    pub background_color: String,
    pub share_url: Option<String>,
    pub src: String,
}

impl SocialNetwork {
    pub fn find(name: &str) -> Option<Self> {
        match name {
            "dribbble" => Some(Self::dribbble()),
            "facebook" => Some(Self::facebook()),
            "github" => Some(Self::github()),
            "google" => Some(Self::google()),
            "instagram" => Some(Self::instagram()),
            "linkedin" => Some(Self::linkedin()),
            "medium" => Some(Self::medium()),
            "pinterest" => Some(Self::pinterest()),
            "snapchat" => Some(Self::snapchat()),
            "soundcloud" => Some(Self::soundcloud()),
            "tumblr" => Some(Self::tumblr()),
            "twitter" => Some(Self::twitter()),
            "vimeo" => Some(Self::vimeo()),
            "web" => Some(Self::web()),
            "xing" => Some(Self::xing()),
            "youtube" => Some(Self::youtube()),
            _ => None,
        }
    }

    fn dribbble() -> Self {
        Self {
            background_color: "#D95988".into(),
            share_url: None,
            src: format!("{}dribbble.png", IMAGE_ORIGIN),
        }
    }

    fn facebook() -> Self {
        Self {
            background_color: "#3b5998".into(),
            share_url: Some("https://www.facebook.com/sharer/sharer.php?u=[[URL]]".into()),
            src: format!("{}facebook.png", IMAGE_ORIGIN),
        }
    }

    fn github() -> Self {
        Self {
            background_color: "#000000".into(),
            share_url: None,
            src: format!("{}github.png", IMAGE_ORIGIN),
        }
    }

    fn google() -> Self {
        Self {
            background_color: "#dc4e41".into(),
            share_url: Some("https://plus.google.com/share?url=[[URL]]".into()),
            src: format!("{}google-plus.png", IMAGE_ORIGIN),
        }
    }

    fn instagram() -> Self {
        Self {
            background_color: "#3f729b".into(),
            share_url: None,
            src: format!("{}instagram.png", IMAGE_ORIGIN),
        }
    }

    fn linkedin() -> Self {
        Self {
            background_color: "#0077b5".into(),
            share_url: Some("https://www.linkedin.com/shareArticle?mini=true&url=[[URL]]&title=&summary=&source=".into()),
            src: format!("{}linkedin.png", IMAGE_ORIGIN),
        }
    }

    fn medium() -> Self {
        Self {
            background_color: "#000000".into(),
            share_url: None,
            src: format!("{}medium.png", IMAGE_ORIGIN),
        }
    }

    fn pinterest() -> Self {
        Self {
            background_color: "#bd081c".into(),
            share_url: Some(
                "https://pinterest.com/pin/create/button/?url=[[URL]]&media=&description=".into(),
            ),
            src: format!("{}pinterest.png", IMAGE_ORIGIN),
        }
    }

    fn snapchat() -> Self {
        Self {
            background_color: "#FFFA54".into(),
            share_url: None,
            src: format!("{}snapchat.png", IMAGE_ORIGIN),
        }
    }

    fn soundcloud() -> Self {
        Self {
            background_color: "#EF7F31".into(),
            share_url: None,
            src: format!("{}soundcloud.png", IMAGE_ORIGIN),
        }
    }

    fn tumblr() -> Self {
        Self {
            background_color: "#344356".into(),
            share_url: Some(
                "https://www.tumblr.com/widgets/share/tool?canonicalUrl=[[URL]]".into(),
            ),
            src: format!("{}tumblr.png", IMAGE_ORIGIN),
        }
    }

    fn twitter() -> Self {
        Self {
            background_color: "#55acee".into(),
            share_url: Some("https://twitter.com/home?status=[[URL]]".into()),
            src: format!("{}twitter.png", IMAGE_ORIGIN),
        }
    }

    fn vimeo() -> Self {
        Self {
            background_color: "#53B4E7".into(),
            share_url: None,
            src: format!("{}vimeo.png", IMAGE_ORIGIN),
        }
    }

    fn web() -> Self {
        Self {
            background_color: "#4BADE9".into(),
            share_url: None,
            src: format!("{}web.png", IMAGE_ORIGIN),
        }
    }

    fn xing() -> Self {
        Self {
            background_color: "#296366".into(),
            share_url: Some("https://www.xing.com/app/user?op=share&url=[[URL]]".into()),
            src: format!("{}xing.png", IMAGE_ORIGIN),
        }
    }

    fn youtube() -> Self {
        Self {
            background_color: "#EB3323".into(),
            share_url: None,
            src: format!("{}youtube.png", IMAGE_ORIGIN),
        }
    }
}

fn create_default_attributes() -> Attributes {
    Attributes::new()
        .add("align", "left")
        .add("color", "#000")
        .add("border-radius", "3px")
        .add("font-family", "Ubuntu, Helvetica, Arial, sans-serif")
        .add("font-size", "13px")
        .add("line-height", "1")
        .add("padding", "4px")
        .add("target", "_blank")
        .add("text-decoration", "none")
}

#[derive(Clone, Debug)]
pub struct MJSocialElement {
    attributes: Attributes,
    context: Option<Context>,
    content: Option<String>,
    social_network: Option<SocialNetwork>,
}

impl MJSocialElement {
    fn default_attributes<'a, 'b>(node: &Node<'a, 'b>, header: &Header) -> Attributes {
        header
            .default_attributes()
            .get_attributes(node, create_default_attributes())
    }

    pub fn parse_social_child<'a, 'b>(
        node: &Node<'a, 'b>,
        header: &Header,
        extra: Option<&Attributes>,
    ) -> Result<MJSocialElement, Error> {
        if node.tag_name().name() != "mj-social-element" {
            return Err(Error::ParseError(format!(
                "element should be 'mj-social-element' no '{}'",
                node.tag_name().name()
            )));
        }
        let content: Vec<&str> = node
            .children()
            .filter(|child| child.is_text())
            .map(|child| child.text())
            .filter(|child| child.is_some())
            .map(|child| child.unwrap())
            .collect();
        let content = if content.len() == 0 {
            None
        } else {
            Some(content.join(""))
        };
        let social_network = node
            .attribute("name")
            .and_then(|name| SocialNetwork::find(name));
        let mut attributes = Self::default_attributes(node, header);
        if let Some(extra) = extra {
            attributes.merge(extra);
        }
        Ok(MJSocialElement {
            attributes: attributes.concat(node),
            context: None,
            content,
            social_network,
        })
    }

    pub fn parse<'a, 'b>(
        node: &Node<'a, 'b>,
        header: &Header,
        extra: Option<&Attributes>,
    ) -> Result<MJSocialElement, Error> {
        let mut attrs = match extra {
            Some(value) => value.into(),
            None => Attributes::new(),
        };
        if attrs.get("text-padding").is_none() {
            attrs.set("text-padding", "4px 4px 4px 0");
        }
        Self::parse_social_child(node, header, Some(&attrs))
    }

    fn get_background_color(&self) -> Option<String> {
        if let Some(bg) = self.get_attribute("background-color") {
            return Some(bg.to_string());
        }
        self.social_network
            .as_ref()
            .and_then(|net| Some(net.background_color.clone()))
    }

    fn get_icon_size(&self) -> Option<Size> {
        self.get_size_attribute("icon-size")
    }

    fn get_icon_height(&self) -> Option<Size> {
        self.get_size_attribute("icon-height")
    }

    fn get_icon_src(&self) -> Option<String> {
        if let Some(src) = self.get_attribute("src") {
            return Some(src.to_string());
        }
        self.social_network
            .as_ref()
            .and_then(|net| Some(net.src.clone()))
    }

    fn set_style_img(&self, tag: Tag) -> Tag {
        tag.set_style("display", "block")
            .maybe_set_style("border-radius", self.get_attribute("border-radius"))
    }

    fn set_style_icon(&self, tag: Tag) -> Tag {
        tag.maybe_set_style("padding", self.get_attribute("icon-padding"))
            .set_style("font-size", "0")
            .maybe_set_style(
                "height",
                self.get_icon_height().or_else(|| self.get_icon_size()),
            )
            .set_style("vertical-align", "middle")
            .maybe_set_style("width", self.get_icon_size())
    }

    fn set_style_table(&self, tag: Tag) -> Tag {
        tag.maybe_set_style("background", self.get_background_color())
            .maybe_set_style("border-radius", self.get_attribute("border-radius"))
            .maybe_set_style("width", self.get_icon_size())
    }

    fn set_style_td(&self, tag: Tag) -> Tag {
        tag.maybe_set_style("padding", self.get_attribute("padding"))
    }

    fn set_style_td_text(&self, tag: Tag) -> Tag {
        tag.maybe_set_style("padding", self.get_attribute("text-padding"))
            .set_style("vertical-align", "middle")
    }

    fn set_style_text(&self, tag: Tag) -> Tag {
        tag.maybe_set_style("color", self.get_attribute("color"))
            .maybe_set_style("font-size", self.get_attribute("font-size"))
            .maybe_set_style("font-weight", self.get_attribute("font-weight"))
            .maybe_set_style("font-style", self.get_attribute("font-style"))
            .maybe_set_style("font-family", self.get_attribute("font-family"))
            .maybe_set_style("line-height", self.get_attribute("line-height"))
            .maybe_set_style("text-decoration", self.get_attribute("text-decoration"))
    }

    fn get_href(&self) -> Option<String> {
        if let Some(href) = self.get_attribute("href").as_ref() {
            self.social_network
                .as_ref()
                .and_then(|net| net.share_url.clone())
                .and_then(move |url| Some(url.replace("[[URL]]", href)))
                .or_else(move || Some(href.to_string()))
        } else {
            None
        }
    }

    fn render_icon(&self, href: &Option<String>) -> String {
        let table = self.set_style_table(Tag::table_presentation());
        let tr = Tag::tr();
        let td = self.set_style_icon(Tag::td());
        let a = Tag::new("a")
            .maybe_set_attribute("href", href.as_ref())
            .maybe_set_attribute("rel", self.get_attribute("rel"))
            .maybe_set_attribute("target", self.get_attribute("target"));
        let img = self
            .set_style_img(Tag::new("img"))
            .maybe_set_attribute("alt", self.get_attribute("alt"))
            .maybe_set_attribute("title", self.get_attribute("title"))
            .maybe_set_attribute(
                "height",
                self.get_icon_height()
                    .or_else(|| self.get_icon_size())
                    .and_then(|size| Some(size.value())),
            )
            .maybe_set_attribute("src", self.get_icon_src())
            .maybe_set_attribute(
                "width",
                self.get_icon_size().and_then(|size| Some(size.value())),
            );

        table.render(tr.render(td.render(if href.is_some() {
            a.render(img.closed())
        } else {
            img.closed()
        })))
    }

    fn render_text(&self, href: &Option<String>) -> String {
        let td = self.set_style_td_text(Tag::new("td"));
        let wrapper = if href.is_some() {
            Tag::new("a")
                .maybe_set_attribute("href", href.as_ref())
                .maybe_set_attribute("rel", self.get_attribute("rel"))
                .maybe_set_attribute("target", self.get_attribute("target"))
        } else {
            Tag::new("span")
        };
        let wrapper = self.set_style_text(wrapper);
        td.render(
            wrapper.render(
                self.content
                    .as_ref()
                    .and_then(|v| Some(v.as_str()))
                    .unwrap_or(""),
            ),
        )
    }
}

impl Component for MJSocialElement {
    fn update_header(&self, header: &mut Header) {
        header.maybe_add_font_families(self.get_attribute("font-family"));
    }

    fn context(&self) -> Option<&Context> {
        self.context.as_ref()
    }

    fn set_context(&mut self, ctx: Context) {
        self.context = Some(ctx.clone());
    }

    fn render(&self, _header: &Header) -> Result<String, Error> {
        let href = self.get_href();
        let tr = Tag::tr().maybe_set_class(self.get_attribute("css-class"));
        let td = self.set_style_td(Tag::td());

        let mut res = vec![];
        res.push(tr.open());
        res.push(td.render(self.render_icon(&href)));
        if self.content.is_some() {
            res.push(self.render_text(&href));
        }
        res.push(tr.close());
        Ok(res.join(""))
    }
}

impl ComponentWithAttributes for MJSocialElement {
    fn attributes(&self) -> Option<&HashMap<String, String>> {
        Some(self.attributes.inner())
    }
}

impl BodyComponent for MJSocialElement {
    fn set_style(&self, name: &str, tag: Tag) -> Tag {
        match name {
            "table" => self.set_style_table(tag),
            "td" => self.set_style_td(tag),
            "icon" => self.set_style_icon(tag),
            "img" => self.set_style_img(tag),
            "td-text" => self.set_style_td_text(tag),
            "text" => self.set_style_text(tag),
            _ => tag,
        }
    }
}

impl BodyContainedComponent for MJSocialElement {}
impl ComponentWithSizeAttribute for MJSocialElement {}
