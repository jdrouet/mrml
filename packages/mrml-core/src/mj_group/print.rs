#[cfg(test)]
mod tests {
    use crate::prelude::print::Printable;

    #[test]
    fn empty() {
        let item = crate::mj_group::MjGroup::default();
        assert_eq!("<mj-group />", item.print_dense().unwrap());
    }
}
