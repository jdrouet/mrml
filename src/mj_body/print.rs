use super::{MJBody, NAME};
use crate::print_attrs_children;

print_attrs_children!(MJBody, NAME);

#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let item = crate::mj_body::MJBody::default();
        assert_eq!("<mj-body></mj-body>", item.dense_print());
    }

    /*
    #[test]
    fn with_children() {
        let mut item = crate::mj_body::MJBody::default();
        item.attributes
            .insert("background-color".to_string(), "red".to_string());
        item.children
            .push(crate::mj_body::MJBodyChild::from(crate::node::Node::from(
                "span",
            )));
        assert_eq!(
            "<mj-body background-color=\"red\"><span></span></mj-body>",
            item.dense_print()
        );
    }
    */
}
