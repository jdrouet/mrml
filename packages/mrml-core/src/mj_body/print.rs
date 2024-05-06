use crate::prelude::print::{PrintableAttributes, PrintableChildren, PrintableElement};

impl PrintableElement for super::MjBody {
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
        let item = crate::mj_body::MjBody::default();
        assert_eq!("<mj-body />", item.print_dense().unwrap());
    }

    #[test]
    fn with_children() {
        let mut item = crate::mj_body::MjBody::default();
        item.attributes
            .insert("background-color".to_string(), "red".to_string());
        item.children
            .push(crate::mj_body::MjBodyChild::from(crate::node::Node::from(
                "span",
            )));
        assert_eq!(
            "<mj-body background-color=\"red\"><span></span></mj-body>",
            item.print_dense().unwrap()
        );
    }
}
