use super::MjIncludeBodyKind;
use crate::prelude::hash::Map;

impl super::MjIncludeBodyAttributes {
    pub fn as_map(&self) -> Map<String, String> {
        let mut res = Map::new();
        res.insert("path".to_string(), self.path.clone());
        if self.kind != MjIncludeBodyKind::default() {
            res.insert("type".into(), self.kind.to_string());
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_button::MjButton;
    use crate::mj_include::body::{MjIncludeBody, MjIncludeBodyChild, MjIncludeBodyKind};
    use crate::prelude::print::Print;

    #[test]
    fn kind_string() {
        assert_eq!(MjIncludeBodyKind::Html.to_string(), "html");
        assert_eq!(MjIncludeBodyKind::Mjml.to_string(), "mjml");
    }

    #[test]
    fn simple() {
        let mut elt = MjIncludeBody::default();
        elt.attributes.path = "memory:include.mjml".to_string();
        elt.children = vec![MjIncludeBodyChild::MjButton(MjButton::default())];
        assert_eq!(
            elt.dense_print(),
            "<mj-include path=\"memory:include.mjml\" />"
        );
    }

    #[test]
    fn html_kind() {
        let mut elt = MjIncludeBody::default();
        elt.attributes.kind = MjIncludeBodyKind::Html;
        elt.attributes.path = "memory:include.html".to_string();
        elt.children = vec![MjIncludeBodyChild::MjButton(MjButton::default())];
        assert_eq!(
            elt.dense_print(),
            "<mj-include path=\"memory:include.html\" type=\"html\" />"
        );
    }
}
