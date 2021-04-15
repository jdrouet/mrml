use super::{MJHead, NAME};
use crate::print_children;

print_children!(MJHead, NAME);

#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let item = crate::mj_head::MJHead::default();
        assert_eq!("<mj-head></mj-head>", item.dense_print());
    }

    #[test]
    fn with_title() {
        let mut item = crate::mj_head::MJHead::default();
        item.children.push(crate::mj_head::MJHeadChild::MJTitle(
            crate::mj_title::MJTitle::from("Hello World!"),
        ));
        assert_eq!(
            "<mj-head><mj-title>Hello World!</mj-title></mj-head>",
            item.dense_print()
        );
    }
}
