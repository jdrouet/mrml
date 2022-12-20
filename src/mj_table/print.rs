use super::MJTable;
use crate::print_attrs_children;

print_attrs_children!(MJTable, super::NAME);

#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let item = crate::mj_table::MJTable::default();
        assert_eq!("<mj-table></mj-table>", item.dense_print());
    }
}
