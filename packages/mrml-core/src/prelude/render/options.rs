use std::borrow::Cow;
use std::collections::HashMap;

pub fn default_fonts() -> HashMap<String, Cow<'static, str>> {
    HashMap::from([
        (
            "Open Sans".into(),
            "https://fonts.googleapis.com/css?family=Open+Sans:300,400,500,700".into(),
        ),
        (
            "Droid Sans".into(),
            "https://fonts.googleapis.com/css?family=Droid+Sans:300,400,500,700".into(),
        ),
        (
            "Lato".into(),
            "https://fonts.googleapis.com/css?family=Lato:300,400,500,700".into(),
        ),
        (
            "Roboto".into(),
            "https://fonts.googleapis.com/css?family=Roboto:300,400,500,700".into(),
        ),
        (
            "Ubuntu".into(),
            "https://fonts.googleapis.com/css?family=Ubuntu:300,400,500,700".into(),
        ),
    ])
}

#[derive(Debug)]
pub struct RenderOptions {
    pub disable_comments: bool,
    pub social_icon_origin: Option<Cow<'static, str>>,
    pub fonts: HashMap<String, Cow<'static, str>>,
}

impl Default for RenderOptions {
    fn default() -> Self {
        Self {
            disable_comments: false,
            social_icon_origin: None,
            fonts: default_fonts(),
        }
    }
}
