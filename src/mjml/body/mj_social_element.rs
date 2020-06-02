use crate::mjml::body::prelude::*;
use crate::mjml::error::Error;
use crate::mjml::prelude::*;
use crate::util::prelude::PropertyMap;
use crate::util::{Attributes, Context, Header, Size, Style};
use crate::Options;
use crate::{close_tag, closed_tag, open_tag, to_attributes};
use log::debug;
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
            "dribble" => Some(Self::dribble()),
            "facebook" => Some(Self::facebook()),
            "github" => Some(Self::github()),
            "google" => Some(Self::google()),
            "instagram" => Some(Self::instagram()),
            "linkedin" => Some(Self::linkedin()),
            "medium" => Some(Self::medium()),
            "pinterest" => Some(Self::pinterest()),
            "snapchat" => Some(Self::snapchat()),
            "soundcloud" => Some(Self::soundcloud()),
            "tumblr" => Some(Self::twitter()),
            "twitter" => Some(Self::twitter()),
            "vimeo" => Some(Self::vimeo()),
            "web" => Some(Self::web()),
            "xing" => Some(Self::xing()),
            "youtube" => Some(Self::youtube()),
            _ => None,
        }
    }

    fn dribble() -> Self {
        Self {
            background_color: "#D95988".into(),
            share_url: None,
            src: format!("{}dribble.png", IMAGE_ORIGIN),
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

#[derive(Clone, Debug)]
pub struct MJSocialElement {
    attributes: HashMap<String, String>,
    context: Option<Context>,
    content: Option<String>,
    social_network: Option<SocialNetwork>,
}

impl MJSocialElement {
    pub fn parse<'a, 'b>(node: Node<'a, 'b>, opts: &Options) -> Result<MJSocialElement, Error> {
        let mut attrs = Attributes::new();
        attrs.set("text-padding", "4px 4px 4px 0");
        Self::parse_with_attributes(node, opts, &attrs)
    }

    pub fn parse_with_attributes<'a, 'b>(
        node: Node<'a, 'b>,
        _opts: &Options,
        extra: &Attributes,
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
        let mut attributes = extra.inner().clone();
        add_node_attributes(&mut attributes, &node);
        println!("content: {:?}", content);
        Ok(MJSocialElement {
            attributes,
            context: None,
            content,
            social_network,
        })
    }

    fn get_background_color(&self) -> Option<String> {
        if let Some(bg) = self.get_attribute("background-color") {
            return Some(bg);
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
            return Some(src);
        }
        self.social_network
            .as_ref()
            .and_then(|net| Some(net.src.clone()))
    }

    fn get_style_img(&self) -> Style {
        let mut res = Style::new();
        res.set("display", "block");
        res.maybe_set("border-radius", self.get_attribute("border-radius"));
        res
    }

    fn get_style_icon(&self) -> Style {
        let mut res = Style::new();
        res.maybe_set("padding", self.get_attribute("icon-padding"));
        res.set("font-size", "0");
        res.maybe_set(
            "height",
            self.get_icon_height().or_else(|| self.get_icon_size()),
        );
        res.set("vertical-align", "middle");
        res.maybe_set("width", self.get_icon_size());
        res
    }

    fn get_style_table(&self) -> Style {
        let mut res = Style::new();
        res.maybe_set("background", self.get_background_color());
        res.maybe_set("border-radius", self.get_attribute("border-radius"));
        res.maybe_set("width", self.get_icon_size());
        res
    }

    fn get_style_td(&self) -> Style {
        let mut res = Style::new();
        res.maybe_set("padding", self.get_attribute("padding"));
        res
    }

    fn get_style_td_text(&self) -> Style {
        let mut res = Style::new();
        res.maybe_set("padding", self.get_attribute("text-padding"));
        res.set("vertical-align", "middle");
        res
    }

    fn get_style_text(&self) -> Style {
        let mut res = Style::new();
        res.maybe_set("color", self.get_attribute("color"));
        res.maybe_set("font-size", self.get_attribute("font-size"));
        res.maybe_set("font-weight", self.get_attribute("font-weight"));
        res.maybe_set("font-style", self.get_attribute("font-style"));
        res.maybe_set("font-family", self.get_attribute("font-family"));
        res.maybe_set("line-height", self.get_attribute("line-height"));
        res.maybe_set("text-decoration", self.get_attribute("text-decoration"));
        res
    }

    fn get_href(&self) -> Option<String> {
        if let Some(href) = self.get_attribute("href").as_ref() {
            self.social_network
                .as_ref()
                .and_then(|net| net.share_url.clone())
                .and_then(move |url| Some(url.replace("[[URL]]", href)))
        } else {
            None
        }
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
        let mut res = vec![];
        {
            let mut attrs = Attributes::new();
            attrs.maybe_set("class", self.get_attribute("css-class"));
            res.push(open_tag!("tr", attrs.to_string()));
        }
        res.push(open_tag!(
            "td",
            to_attributes!(("style", self.get_style_td().to_string()))
        ));
        res.push(open_tag!(
            "table",
            to_attributes!(
                ("border", "0"),
                ("cellpadding", "0"),
                ("cellspacing", "0"),
                ("role", "presentation"),
                ("style", self.get_style_table().to_string())
            )
        ));
        res.push(open_tag!("tr"));
        res.push(open_tag!(
            "td",
            to_attributes!(("style", self.get_style_icon().to_string()))
        ));
        if let Some(href) = href.as_ref() {
            let mut attrs = Attributes::new();
            attrs.set("href", href);
            attrs.maybe_set("rel", self.get_attribute("rel"));
            attrs.maybe_set("target", self.get_attribute("target"));
            res.push(open_tag!("a", attrs.to_string()));
        }
        {
            let mut attrs = Attributes::new();
            attrs.maybe_set("alt", self.get_attribute("alt"));
            attrs.maybe_set("title", self.get_attribute("title"));
            attrs.maybe_set(
                "height",
                self.get_icon_height()
                    .or_else(|| self.get_icon_size())
                    .and_then(|size| Some(size.value())),
            );
            attrs.maybe_set("src", self.get_icon_src());
            attrs.set("style", self.get_style_img());
            attrs.maybe_set(
                "width",
                self.get_icon_size().and_then(|size| Some(size.value())),
            );
            res.push(closed_tag!("img", attrs.to_string()));
        }
        if href.is_some() {
            res.push(close_tag!("a"));
        }
        res.push(close_tag!("td"));
        res.push(close_tag!("tr"));
        res.push(close_tag!("table"));
        res.push(close_tag!("td"));
        if let Some(content) = self.content.as_ref() {
            res.push(open_tag!(
                "td",
                to_attributes!(("style", self.get_style_td_text().to_string()))
            ));
            if let Some(href) = href.as_ref() {
                let mut attrs = Attributes::new();
                attrs.set("href", href);
                attrs.set("style", self.get_style_text());
                attrs.maybe_set("rel", self.get_attribute("rel"));
                attrs.maybe_set("target", self.get_attribute("target"));
                res.push(open_tag!("a", attrs.to_string()));
            } else {
                res.push(open_tag!(
                    "span",
                    to_attributes!(("style", self.get_style_text().to_string()))
                ));
            }
            res.push(content.to_string());
            if href.is_some() {
                res.push(close_tag!("a"));
            } else {
                res.push(close_tag!("span"));
            }
            res.push(close_tag!("td"));
        }
        res.push(close_tag!("tr"));
        Ok(res.join(""))
    }
}

impl ComponentWithAttributes for MJSocialElement {
    fn default_attribute(&self, key: &str) -> Option<String> {
        debug!("default_attribute {}", key);
        match key {
            "align" => Some("lef".into()),
            "color" => Some("#000".into()),
            "border-radius" => Some("3px".into()),
            "font-family" => Some("Ubuntu, Helvetica, Arial, sans-serif".into()),
            "font-size" => Some("13px".into()),
            "line-height" => Some("1".into()),
            "padding" => Some("4px".into()),
            "target" => Some("_blank".into()),
            "text-decoration" => Some("none".into()),
            _ => None,
        }
    }

    fn source_attributes(&self) -> Option<&HashMap<String, String>> {
        Some(&self.attributes)
    }
}

impl BodyComponent for MJSocialElement {
    fn get_style(&self, name: &str) -> Style {
        match name {
            "table" => self.get_style_table(),
            "td" => self.get_style_td(),
            "icon" => self.get_style_icon(),
            "img" => self.get_style_img(),
            "td-text" => self.get_style_td_text(),
            "text" => self.get_style_text(),
            _ => Style::new(),
        }
    }
}

impl BodyContainedComponent for MJSocialElement {}
impl ComponentWithSizeAttribute for MJSocialElement {}
