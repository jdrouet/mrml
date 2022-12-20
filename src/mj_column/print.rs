use super::{MJColumn, NAME};
use crate::print_attrs_children;

print_attrs_children!(MJColumn, NAME);

#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let item = crate::mj_column::MJColumn::default();
        assert_eq!("<mj-column></mj-column>", item.dense_print());
    }
}
