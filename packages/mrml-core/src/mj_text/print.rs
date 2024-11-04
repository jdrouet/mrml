#[cfg(test)]
mod tests {
    use crate::prelude::print::Printable;

    #[test]
    fn empty() {
        let mut item = crate::mj_text::MjText::default();
        item.attributes
            .insert("href".to_string(), Some("http://localhost".into()));
        item.children
            .push(crate::text::Text::from(String::from("test")).into());
        assert_eq!(
            "<mj-text href=\"http://localhost\">test</mj-text>",
            item.print_dense().unwrap()
        );
    }
}
