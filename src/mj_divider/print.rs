use super::{MJDivider, NAME};
use crate::print_attrs;

print_attrs!(MJDivider, NAME);

#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let mut item = crate::mj_divider::MJDivider::default();
        item.attributes
            .insert("src".to_string(), "http://localhost".into());
        assert_eq!(
            "<mj-divider src=\"http://localhost\" />",
            item.dense_print()
        );
    }
}
