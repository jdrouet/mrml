pub struct SocialNetwork {
    background_color: &'static str,
    share_url: Option<&'static str>,
    icon: &'static str,
}

impl SocialNetwork {
    pub fn background_color(&self) -> &str {
        self.background_color
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
            background_color: "#D95988",
            share_url: None,
            icon: "dribbble.png",
        }
    }

    fn facebook(noshare: bool) -> Self {
        Self {
            background_color: "#3b5998",
            share_url: if noshare {
                None
            } else {
                Some("https://www.facebook.com/sharer/sharer.php?u=[[URL]]")
            },
            icon: "facebook.png",
        }
    }

    fn github() -> Self {
        Self {
            background_color: "#000000",
            share_url: None,
            icon: "github.png",
        }
    }

    fn google(noshare: bool) -> Self {
        Self {
            background_color: "#dc4e41",
            share_url: if noshare {
                None
            } else {
                Some("https://plus.google.com/share?url=[[URL]]")
            },
            icon: "google-plus.png",
        }
    }

    fn instagram() -> Self {
        Self {
            background_color: "#3f729b",
            share_url: None,
            icon: "instagram.png",
        }
    }

    fn linkedin(noshare: bool) -> Self {
        Self {
            background_color: "#0077b5",
            share_url: if noshare {
                None
            } else {
                Some("https://www.linkedin.com/shareArticle?mini=true&url=[[URL]]&title=&summary=&source=")
            },
            icon: "linkedin.png",
        }
    }

    fn medium() -> Self {
        Self {
            background_color: "#000000",
            share_url: None,
            icon: "medium.png",
        }
    }

    fn pinterest(noshare: bool) -> Self {
        Self {
            background_color: "#bd081c",
            share_url: if noshare {
                None
            } else {
                Some("https://pinterest.com/pin/create/button/?url=[[URL]]&media=&description=")
            },
            icon: "pinterest.png",
        }
    }

    fn snapchat() -> Self {
        Self {
            background_color: "#FFFA54",
            share_url: None,
            icon: "snapchat.png",
        }
    }

    fn soundcloud() -> Self {
        Self {
            background_color: "#EF7F31",
            share_url: None,
            icon: "soundcloud.png",
        }
    }

    fn tumblr(noshare: bool) -> Self {
        Self {
            background_color: "#344356",
            share_url: if noshare {
                None
            } else {
                Some("https://www.tumblr.com/widgets/share/tool?canonicalUrl=[[URL]]")
            },
            icon: "tumblr.png",
        }
    }

    fn twitter(noshare: bool) -> Self {
        Self {
            background_color: "#55acee",
            share_url: if noshare {
                None
            } else {
                Some("https://twitter.com/home?status=[[URL]]")
            },
            icon: "twitter.png",
        }
    }

    fn vimeo() -> Self {
        Self {
            background_color: "#53B4E7",
            share_url: None,
            icon: "vimeo.png",
        }
    }

    fn web() -> Self {
        Self {
            background_color: "#4BADE9",
            share_url: None,
            icon: "web.png",
        }
    }

    fn xing(noshare: bool) -> Self {
        Self {
            background_color: "#296366",
            share_url: if noshare {
                None
            } else {
                Some("https://www.xing.com/app/user?op=share&url=[[URL]]")
            },
            icon: "xing.png",
        }
    }

    fn youtube() -> Self {
        Self {
            background_color: "#EB3323",
            share_url: None,
            icon: "youtube.png",
        }
    }
}
