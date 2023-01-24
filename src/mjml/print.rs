#[cfg(test)]
mod tests {
    use crate::mjml::{Mjml, MjmlChildren};
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let item = Mjml::default();
        assert_eq!("<mjml />", format!("{item}"));
    }

    #[test]
    fn with_lang() {
        let mut item = Mjml::default();
        item.attributes.lang = Some("fr".to_string());
        assert_eq!("<mjml lang=\"fr\" />", format!("{item}"));
    }

    #[test]
    fn with_body() {
        let item = Mjml {
            attributes: Default::default(),
            children: MjmlChildren {
                head: None,
                body: Some(crate::mj_body::MjBody::default()),
            },
        };
        assert_eq!("<mjml><mj-body /></mjml>", item.dense_print());
        assert_eq!("<mjml>\n  <mj-body />\n</mjml>\n", item.pretty_print());
    }
}
