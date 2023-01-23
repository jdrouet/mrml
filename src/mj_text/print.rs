#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let mut item = crate::mj_text::MjText::default();
        item.attributes
            .insert("href".to_string(), "http://localhost".into());
        item.children
            .push(crate::text::Text::from(String::from("test")).into());
        assert_eq!(
            "<mj-text href=\"http://localhost\">test</mj-text>",
            item.dense_print()
        );
    }
}
