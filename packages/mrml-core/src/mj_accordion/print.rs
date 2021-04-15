use super::{MJAccordion, NAME};
use crate::print_attrs_children;

print_attrs_children!(MJAccordion, NAME);

#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let item = crate::mj_accordion::MJAccordion::default();
        assert_eq!("<mj-accordion></mj-accordion>", item.dense_print());
    }
}
