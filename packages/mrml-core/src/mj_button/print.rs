use crate::prelude::print::{PrintableAttributes, PrintableChildren, PrintableElement};

impl PrintableElement for super::MjButton {
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
        let mut item = crate::mj_button::MjButton::default();
        item.attributes
            .insert("href".to_string(), "http://localhost".into());
        item.children
            .push(crate::mj_body::MjBodyChild::Text(crate::text::Text::from(
                "Hello World!".to_string(),
            )));
        assert_eq!(
            "<mj-button href=\"http://localhost\">Hello World!</mj-button>",
            item.print_dense().unwrap()
        );
    }
}
