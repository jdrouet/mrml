use std::{borrow::Cow, collections::HashMap};

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize, tsify::Tsify)]
#[serde(rename_all = "camelCase")]
#[tsify(into_wasm_abi, from_wasm_abi)]
/// Rendering options
pub struct RenderOptions {
    /// If disabled, the comments won't be kept in the result. Disabled by default.
    pub disable_comments: bool,
    /// Base url of the server to fetch the social icons.
    pub social_icon_origin: Option<String>,
    /// Map of fonts that can be used.
    pub fonts: HashMap<String, String>,
}

impl From<RenderOptions> for mrml::prelude::render::Options {
    fn from(value: RenderOptions) -> Self {
        Self {
            disable_comments: value.disable_comments,
            social_icon_origin: value.social_icon_origin.map(Cow::Owned),
            fonts: value
                .fonts
                .into_iter()
                .map(|(key, value)| (key, Cow::Owned(value)))
                .collect(),
        }
    }
}
