#[cfg(test)]
mod tests {
    use crate::prelude::print::Printable;

    #[test]
    fn empty() {
        let mut item = crate::mj_divider::MjDivider::default();
        item.attributes
            .insert("src".to_string(), Some("http://localhost".into()));
        assert_eq!(
            "<mj-divider src=\"http://localhost\" />",
            item.print_dense().unwrap()
        );
    }
}
