use super::{MJGroup, NAME};
use crate::print_attrs_children;

print_attrs_children!(MJGroup, NAME);

#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let item = crate::mj_group::MJGroup::default();
        assert_eq!("<mj-group></mj-group>", item.dense_print());
    }
}
