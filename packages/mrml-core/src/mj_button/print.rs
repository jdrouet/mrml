#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

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
            item.dense_print()
        );
        assert_eq!(
            "<mj-button href=\"http://localhost\">\n  Hello World!\n</mj-button>\n",
            item.pretty_print()
        );
    }
}
