use super::MJSection;
use crate::print_attrs_children;

print_attrs_children!(MJSection, super::NAME);

#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let item = crate::mj_section::MJSection::default();
        assert_eq!("<mj-section></mj-section>", item.dense_print());
    }
}
