use crate::elements::body::prelude::*;
use crate::elements::body::BodyElement;
use crate::elements::error::Error;
use crate::elements::prelude::*;
use crate::parser::Node;
use crate::util::attributes::*;
use crate::util::context::Context;
use crate::util::header::Header;
use crate::util::size::Size;
use crate::util::tag::Tag;
use std::collections::HashMap;
use std::str::FromStr;

const IMAGE_ORIGIN: &str = "https://www.mailjet.com/images/theme/v1/icons/ico-social/";
const FACEBOOK_SHARE_URL: &str = "https://www.facebook.com/sharer/sharer.php?u=[[URL]]";
const GOOGLE_SHARE_URL: &str = "https://plus.google.com/share?url=[[URL]]";
const LINKEDIN_SHARE_URL: &str =
    "https://www.linkedin.com/shareArticle?mini=true&url=[[URL]]&title=&summary=&source=";
const PINTEREST_SHARE_URL: &str =
    "https://pinterest.com/pin/create/button/?url=[[URL]]&media=&description=";
const TUMBLR_SHARE_URL: &str = "https://www.tumblr.com/widgets/share/tool?canonicalUrl=[[URL]]";
const TWITTER_SHARE_URL: &str = "https://twitter.com/home?status=[[URL]]";
const XING_SHARE_URL: &str = "https://www.xing.com/app/user?op=share&url=[[URL]]";

lazy_static! {
    static ref SOCIAL_NETWORKS: HashMap<&'static str, (&'static str, &'static str, Option<&'static str>)> =
        vec![
            ("dribbble", ("dribbble.png", "#D95988", None)),
            (
                "facebook",
                ("facebook.png", "#3b5998", Some(FACEBOOK_SHARE_URL))
            ),
            ("github", ("github.png", "#000000", None)),
            (
                "google",
                ("google-plus.png", "#dc4e41", Some(GOOGLE_SHARE_URL))
            ),
            ("instagram", ("instagram.png", "#3f729b", None)),
            (
                "linkedin",
                ("linkedin.png", "#0077b5", Some(LINKEDIN_SHARE_URL))
            ),
            ("medium", ("medium.png", "#000000", None)),
            (
                "pinterest",
                ("pinterest.png", "#bd081c", Some(PINTEREST_SHARE_URL))
            ),
            ("snapchat", ("snapchat.png", "#FFFA54", None)),
            ("soundcloud", ("soundcloud.png", "#EF7F31", None)),
            ("tumblr", ("tumblr.png", "#344356", Some(TUMBLR_SHARE_URL))),
            (
                "twitter",
                ("twitter.png", "#55acee", Some(TWITTER_SHARE_URL))
            ),
            ("vimeo", ("vimeo.png", "#53B4E7", None)),
            ("web", ("web.png", "#4BADE9", None)),
            ("xing", ("xing.png", "#296366", Some(XING_SHARE_URL))),
            ("youtube", ("youtube.png", "#EB3323", None)),
        ]
        .into_iter()
        .collect();
}

#[derive(Debug, Clone)]
struct SocialNetwork {
    background_color: &'static str,
    share_url: Option<&'static str>,
    image_name: &'static str,
}

impl SocialNetwork {
    fn get_img_src(&self) -> String {
        format!("{}{}", IMAGE_ORIGIN, self.image_name)
    }
}

impl FromStr for SocialNetwork {
    type Err = String;

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        let (name, no_share) = if name.ends_with("-noshare") {
            let (label, _noshare) = name.split_at(name.len() - 8);
            (label, true)
        } else {
            (name, false)
        };

        SOCIAL_NETWORKS
            .get(name)
            .map(|(image_name, background_color, share_url)| {
                let share_url = if no_share { None } else { *share_url };

                SocialNetwork {
                    background_color,
                    share_url,
                    image_name,
                }
            })
            .ok_or(format!("unable to found a social network named {}", name))
    }
}

lazy_static! {
    static ref DEFAULT_ATTRIBUTES: Attributes = Attributes::default()
        .add("align", "left")
        .add("color", "#000")
        .add("border-radius", "3px")
        .add("font-family", "Ubuntu, Helvetica, Arial, sans-serif")
        .add("font-size", "13px")
        .add("line-height", "1")
        .add("padding", "4px")
        .add("target", "_blank")
        .add("text-decoration", "none");
}

#[derive(Clone, Debug)]
pub struct MJSocialElement {
    attributes: Attributes,
    context: Option<Context>,
    content: Option<String>,
    social_network: Option<SocialNetwork>,
}

impl MJSocialElement {
    fn default_attributes<'a>(node: &Node<'a>, header: &Header) -> Attributes {
        header
            .default_attributes()
            .get_attributes(node, DEFAULT_ATTRIBUTES.clone())
    }

    pub fn parse_social_child<'a>(
        node: &Node<'a>,
        header: &Header,
        extra: Option<&Attributes>,
    ) -> Result<MJSocialElement, Error> {
        if node.name.as_str() != "mj-social-element" {
            return Err(Error::UnexpectedElement(node.name.as_str().into()));
        }
        let content = node
            .children
            .iter()
            .filter_map(|child| child.as_text())
            .map(|child| child.as_str())
            .collect::<String>();
        let social_network = node
            .attributes
            .iter()
            .find(|(key, _value)| key.as_str() == "name")
            .and_then(|(_key, value)| SocialNetwork::from_str(value.as_str()).ok());
        let mut attributes = Self::default_attributes(node, header);
        if let Some(extra) = extra {
            attributes.merge(extra);
        }
        Ok(MJSocialElement {
            attributes: attributes.concat(node),
            context: None,
            content: if content.is_empty() {
                None
            } else {
                Some(content)
            },
            social_network,
        })
    }

    pub fn parse<'a>(
        node: &Node<'a>,
        header: &Header,
        extra: Option<&Attributes>,
    ) -> Result<MJSocialElement, Error> {
        let mut attrs = match extra {
            Some(value) => value.into(),
            None => Attributes::default(),
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
            .map(|net| net.background_color.to_string())
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
        self.social_network.as_ref().map(|net| net.get_img_src())
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
        self.get_attribute("href")
            .map(|href| {
                self.social_network
                    .as_ref()
                    .and_then(|net| net.share_url)
                    .map(move |url| url.replace("[[URL]]", href))
                    .or_else(move || Some(href.to_string()))
            })
            .unwrap_or_default()
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
                    .map(|size| size.value()),
            )
            .maybe_set_attribute("src", self.get_icon_src())
            .maybe_set_attribute("width", self.get_icon_size().map(|size| size.value()));

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
        td.render(wrapper.render(self.content.as_deref().unwrap_or("")))
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
        self.context = Some(ctx);
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

impl BodyComponent for MJSocialElement {
    fn attributes(&self) -> Option<&Attributes> {
        Some(&self.attributes)
    }

    fn get_children(&self) -> &Vec<BodyElement> {
        &EMPTY_CHILDREN
    }

    fn get_current_width(&self) -> Option<Size> {
        None
    }

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
