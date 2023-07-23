use super::MjIncludeHeadKind;
use crate::prelude::hash::Map;

impl super::MjIncludeHeadAttributes {
    pub fn as_map(&self) -> Map<String, String> {
        let mut res = Map::new();
        res.insert("path".to_string(), self.path.clone());
        match self.kind {
            MjIncludeHeadKind::Html => {
                res.insert("type".into(), "html".into());
            }
            MjIncludeHeadKind::Css { inline } => {
                res.insert("type".into(), "css".into());
                if inline {
                    res.insert("css-inline".into(), "inline".into());
                }
            }
            _ => {}
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_include::head::{MjIncludeHead, MjIncludeHeadChild, MjIncludeHeadKind};
    use crate::mj_title::MjTitle;
    use crate::prelude::print::Print;

    #[test]
    fn simple() {
        let mut elt = MjIncludeHead::default();
        elt.attributes.path = "memory:include.mjml".to_string();
        elt.children = vec![MjIncludeHeadChild::MjTitle(MjTitle::from(
            "Hello World!".to_owned(),
        ))];
        assert_eq!(
            elt.dense_print(),
            "<mj-include path=\"memory:include.mjml\" />"
        );
    }

    #[test]
    fn html_kind() {
        let mut elt = MjIncludeHead::default();
        elt.attributes.kind = MjIncludeHeadKind::Html;
        elt.attributes.path = "memory:include.html".to_string();
        elt.children = vec![MjIncludeHeadChild::MjTitle(MjTitle::from(
            "Hello World!".to_owned(),
        ))];
        assert_eq!(
            elt.dense_print(),
            "<mj-include path=\"memory:include.html\" type=\"html\" />"
        );
    }
}
