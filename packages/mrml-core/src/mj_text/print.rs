use crate::prelude::print::{PrintableAttributes, PrintableChildren, PrintableElement};

impl PrintableElement for super::MjText {
    fn tag(&self) -> &str {
        super::NAME
    }

    fn attributes(&self) -> &impl PrintableAttributes {
        &self.attributes
    }

    fn children(&self) -> &impl PrintableChildren {
        &self.children
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::print::Printable;

    #[test]
    fn empty() {
        let mut item = crate::mj_text::MjText::default();
        item.attributes
            .insert("href".to_string(), "http://localhost".into());
        item.children
            .push(crate::text::Text::from(String::from("test")).into());
        assert_eq!(
            "<mj-text href=\"http://localhost\">test</mj-text>",
            item.print_dense().unwrap()
        );
    }
}
