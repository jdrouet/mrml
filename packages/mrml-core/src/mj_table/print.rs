#[cfg(test)]
mod tests {
    use crate::prelude::print::Printable;

    #[test]
    fn empty() {
        let item = crate::mj_table::MjTable::default();
        assert_eq!("<mj-table />", item.print_dense().unwrap());
    }
}
