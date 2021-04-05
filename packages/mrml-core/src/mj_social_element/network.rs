pub struct SocialNetwork {
    background_color: String,
    share_url: Option<String>,
    icon: String,
}

impl SocialNetwork {
    pub fn background_color(&self) -> &str {
        &self.background_color
    }

    pub fn share_url(&self, url: &str) -> Option<String> {
        self.share_url
            .as_ref()
            .map(|share_url| share_url.replace("[[URL]]", url))
    }

    pub fn icon_src(&self, origin: &str) -> String {
        format!("{}{}", origin, self.icon)
    }
}

impl SocialNetwork {
    // convert to a TryFrom with proper error handling
    pub fn find(name: &str) -> Option<Self> {
        let (name, noshare) = if name.ends_with("-noshare") {
            let (label, _noshare) = name.split_at(name.len() - 8);
            (label, true)
        } else {
            (name, false)
        };

        match name {
            "dribbble" => Some(Self::dribbble()),
            "facebook" => Some(Self::facebook(noshare)),
            "github" => Some(Self::github()),
            "google" => Some(Self::google(noshare)),
            "instagram" => Some(Self::instagram()),
            "linkedin" => Some(Self::linkedin(noshare)),
            "medium" => Some(Self::medium()),
            "pinterest" => Some(Self::pinterest(noshare)),
            "snapchat" => Some(Self::snapchat()),
            "soundcloud" => Some(Self::soundcloud()),
            "tumblr" => Some(Self::tumblr(noshare)),
            "twitter" => Some(Self::twitter(noshare)),
            "vimeo" => Some(Self::vimeo()),
            "web" => Some(Self::web()),
            "xing" => Some(Self::xing(noshare)),
            "youtube" => Some(Self::youtube()),
            _ => None,
        }
    }

    fn dribbble() -> Self {
        Self {
            background_color: "#D95988".to_string(),
            share_url: None,
            icon: "dribbble.png".to_string(),
        }
    }

    fn facebook(noshare: bool) -> Self {
        Self {
            background_color: "#3b5998".to_string(),
            share_url: if noshare {
                None
            } else {
                Some("https://www.facebook.com/sharer/sharer.php?u=[[URL]]".to_string())
            },
            icon: "facebook.png".to_string(),
        }
    }

    fn github() -> Self {
        Self {
            background_color: "#000000".to_string(),
            share_url: None,
            icon: "github.png".to_string(),
        }
    }

    fn google(noshare: bool) -> Self {
        Self {
            background_color: "#dc4e41".to_string(),
            share_url: if noshare {
                None
            } else {
                Some("https://plus.google.com/share?url=[[URL]]".to_string())
            },
            icon: "google-plus.png".to_string(),
        }
    }

    fn instagram() -> Self {
        Self {
            background_color: "#3f729b".to_string(),
            share_url: None,
            icon: "instagram.png".to_string(),
        }
    }

    fn linkedin(noshare: bool) -> Self {
        Self {
            background_color: "#0077b5".to_string(),
            share_url: if noshare {
                None
            } else {
                Some("https://www.linkedin.com/shareArticle?mini=true&url=[[URL]]&title=&summary=&source=".to_string())
            },
            icon: "linkedin.png".to_string(),
        }
    }

    fn medium() -> Self {
        Self {
            background_color: "#000000".to_string(),
            share_url: None,
            icon: "medium.png".to_string(),
        }
    }

    fn pinterest(noshare: bool) -> Self {
        Self {
            background_color: "#bd081c".to_string(),
            share_url: if noshare {
                None
            } else {
                Some(
                    "https://pinterest.com/pin/create/button/?url=[[URL]]&media=&description="
                        .to_string(),
                )
            },
            icon: "pinterest.png".to_string(),
        }
    }

    fn snapchat() -> Self {
        Self {
            background_color: "#FFFA54".to_string(),
            share_url: None,
            icon: "snapchat.png".to_string(),
        }
    }

    fn soundcloud() -> Self {
        Self {
            background_color: "#EF7F31".to_string(),
            share_url: None,
            icon: "soundcloud.png".to_string(),
        }
    }

    fn tumblr(noshare: bool) -> Self {
        Self {
            background_color: "#344356".to_string(),
            share_url: if noshare {
                None
            } else {
                Some("https://www.tumblr.com/widgets/share/tool?canonicalUrl=[[URL]]".to_string())
            },
            icon: "tumblr.png".to_string(),
        }
    }

    fn twitter(noshare: bool) -> Self {
        Self {
            background_color: "#55acee".to_string(),
            share_url: if noshare {
                None
            } else {
                Some("https://twitter.com/home?status=[[URL]]".to_string())
            },
            icon: "twitter.png".to_string(),
        }
    }

    fn vimeo() -> Self {
        Self {
            background_color: "#53B4E7".to_string(),
            share_url: None,
            icon: "vimeo.png".to_string(),
        }
    }

    fn web() -> Self {
        Self {
            background_color: "#4BADE9".to_string(),
            share_url: None,
            icon: "web.png".to_string(),
        }
    }

    fn xing(noshare: bool) -> Self {
        Self {
            background_color: "#296366".to_string(),
            share_url: if noshare {
                None
            } else {
                Some("https://www.xing.com/app/user?op=share&url=[[URL]]".to_string())
            },
            icon: "xing.png".to_string(),
        }
    }

    fn youtube() -> Self {
        Self {
            background_color: "#EB3323".to_string(),
            share_url: None,
            icon: "youtube.png".to_string(),
        }
    }
}
