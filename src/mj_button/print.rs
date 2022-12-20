use super::{MJButton, NAME};
use crate::print_attrs_children;

print_attrs_children!(MJButton, NAME);

#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let mut item = crate::mj_button::MJButton::default();
        item.attributes
            .insert("href".to_string(), "http://localhost".into());
        assert_eq!(
            "<mj-button href=\"http://localhost\"></mj-button>",
            item.dense_print()
        );
        assert_eq!(
            "<mj-button href=\"http://localhost\">\n</mj-button>\n",
            item.pretty_print()
        );
    }
}
