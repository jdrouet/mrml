use crate::prelude::print::{PrintableAttributes, PrintableElement};

impl PrintableElement for super::MjDivider {
    fn tag(&self) -> &str {
        super::NAME
    }

    fn attributes(&self) -> &impl PrintableAttributes {
        &self.attributes
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::print::Printable;

    #[test]
    fn empty() {
        let mut item = crate::mj_divider::MjDivider::default();
        item.attributes
            .insert("src".to_string(), "http://localhost".into());
        assert_eq!(
            "<mj-divider src=\"http://localhost\" />",
            item.print_dense().unwrap()
        );
    }
}
