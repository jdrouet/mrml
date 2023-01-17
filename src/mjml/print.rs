#[cfg(test)]
mod tests {
    use crate::mjml::{MJMLChildren, MJML};
    use crate::prelude::hash::Map;
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let item = MJML::default();
        assert_eq!("<mjml />", format!("{item}"));
    }

    #[test]
    fn with_lang() {
        let mut item = MJML::default();
        item.attributes.insert("lang".to_string(), "fr".to_string());
        assert_eq!("<mjml lang=\"fr\" />", format!("{item}"));
    }

    #[test]
    fn with_body() {
        let item = MJML {
            attributes: Map::default(),
            children: MJMLChildren {
                head: None,
                body: Some(crate::mj_body::MJBody::default()),
            },
        };
        assert_eq!("<mjml><mj-body /></mjml>", item.dense_print());
        assert_eq!("<mjml>\n  <mj-body />\n</mjml>\n", item.pretty_print());
    }
}
