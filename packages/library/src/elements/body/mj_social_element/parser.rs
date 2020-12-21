use super::{MJSocialElement, SocialNetwork};
use crate::elements::error::Error;
use crate::parser::Node;
use crate::util::attributes::*;
use crate::util::header::Header;

impl SocialNetwork {
    pub fn find(icon_origin: &str, name: &str) -> Option<Self> {
        let (name, noshare) = if name.ends_with("-noshare") {
            let (label, _noshare) = name.split_at(name.len() - 8);
            (label, true)
        } else {
            (name, false)
        };

        match name {
            "dribbble" => Some(Self::dribbble(icon_origin)),
            "facebook" => Some(Self::facebook(icon_origin, noshare)),
            "github" => Some(Self::github(icon_origin)),
            "google" => Some(Self::google(icon_origin, noshare)),
            "instagram" => Some(Self::instagram(icon_origin)),
            "linkedin" => Some(Self::linkedin(icon_origin, noshare)),
            "medium" => Some(Self::medium(icon_origin)),
            "pinterest" => Some(Self::pinterest(icon_origin, noshare)),
            "snapchat" => Some(Self::snapchat(icon_origin)),
            "soundcloud" => Some(Self::soundcloud(icon_origin)),
            "tumblr" => Some(Self::tumblr(icon_origin, noshare)),
            "twitter" => Some(Self::twitter(icon_origin, noshare)),
            "vimeo" => Some(Self::vimeo(icon_origin)),
            "web" => Some(Self::web(icon_origin)),
            "xing" => Some(Self::xing(icon_origin, noshare)),
            "youtube" => Some(Self::youtube(icon_origin)),
            _ => None,
        }
    }

    fn dribbble(icon_origin: &str) -> Self {
        Self {
            background_color: "#D95988".into(),
            share_url: None,
            src: format!("{}dribbble.png", icon_origin),
        }
    }

    fn facebook(icon_origin: &str, noshare: bool) -> Self {
        Self {
            background_color: "#3b5998".into(),
            share_url: if noshare {
                None
            } else {
                Some("https://www.facebook.com/sharer/sharer.php?u=[[URL]]".into())
            },
            src: format!("{}facebook.png", icon_origin),
        }
    }

    fn github(icon_origin: &str) -> Self {
        Self {
            background_color: "#000000".into(),
            share_url: None,
            src: format!("{}github.png", icon_origin),
        }
    }

    fn google(icon_origin: &str, noshare: bool) -> Self {
        Self {
            background_color: "#dc4e41".into(),
            share_url: if noshare {
                None
            } else {
                Some("https://plus.google.com/share?url=[[URL]]".into())
            },
            src: format!("{}google-plus.png", icon_origin),
        }
    }

    fn instagram(icon_origin: &str) -> Self {
        Self {
            background_color: "#3f729b".into(),
            share_url: None,
            src: format!("{}instagram.png", icon_origin),
        }
    }

    fn linkedin(icon_origin: &str, noshare: bool) -> Self {
        Self {
            background_color: "#0077b5".into(),
            share_url: if noshare {
                None
            } else {
                Some("https://www.linkedin.com/shareArticle?mini=true&url=[[URL]]&title=&summary=&source="
            .into())
            },
            src: format!("{}linkedin.png", icon_origin),
        }
    }

    fn medium(icon_origin: &str) -> Self {
        Self {
            background_color: "#000000".into(),
            share_url: None,
            src: format!("{}medium.png", icon_origin),
        }
    }

    fn pinterest(icon_origin: &str, noshare: bool) -> Self {
        Self {
            background_color: "#bd081c".into(),
            share_url: if noshare {
                None
            } else {
                Some(
                    "https://pinterest.com/pin/create/button/?url=[[URL]]&media=&description="
                        .into(),
                )
            },
            src: format!("{}pinterest.png", icon_origin),
        }
    }

    fn snapchat(icon_origin: &str) -> Self {
        Self {
            background_color: "#FFFA54".into(),
            share_url: None,
            src: format!("{}snapchat.png", icon_origin),
        }
    }

    fn soundcloud(icon_origin: &str) -> Self {
        Self {
            background_color: "#EF7F31".into(),
            share_url: None,
            src: format!("{}soundcloud.png", icon_origin),
        }
    }

    fn tumblr(icon_origin: &str, noshare: bool) -> Self {
        Self {
            background_color: "#344356".into(),
            share_url: if noshare {
                None
            } else {
                Some("https://www.tumblr.com/widgets/share/tool?canonicalUrl=[[URL]]".into())
            },
            src: format!("{}tumblr.png", icon_origin),
        }
    }

    fn twitter(icon_origin: &str, noshare: bool) -> Self {
        Self {
            background_color: "#55acee".into(),
            share_url: if noshare {
                None
            } else {
                Some("https://twitter.com/home?status=[[URL]]".into())
            },
            src: format!("{}twitter.png", icon_origin),
        }
    }

    fn vimeo(icon_origin: &str) -> Self {
        Self {
            background_color: "#53B4E7".into(),
            share_url: None,
            src: format!("{}vimeo.png", icon_origin),
        }
    }

    fn web(icon_origin: &str) -> Self {
        Self {
            background_color: "#4BADE9".into(),
            share_url: None,
            src: format!("{}web.png", icon_origin),
        }
    }

    fn xing(icon_origin: &str, noshare: bool) -> Self {
        Self {
            background_color: "#296366".into(),
            share_url: if noshare {
                None
            } else {
                Some("https://www.xing.com/app/user?op=share&url=[[URL]]".into())
            },
            src: format!("{}xing.png", icon_origin),
        }
    }

    fn youtube(icon_origin: &str) -> Self {
        Self {
            background_color: "#EB3323".into(),
            share_url: None,
            src: format!("{}youtube.png", icon_origin),
        }
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
        .add("text-decoration", "none")
        .add("vertical-align", "middle");
}

impl MJSocialElement {
    fn default_attributes<'a>(node: &Node<'a>, header: &Header) -> Attributes {
        header
            .default_attributes
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
            .and_then(|(_key, value)| {
                SocialNetwork::find(header.social_icon_origin.as_str(), value.as_str())
            });
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
}
