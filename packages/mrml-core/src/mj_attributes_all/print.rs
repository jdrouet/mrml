#[cfg(test)]
mod tests {
    use crate::prelude::print::Printable;

    #[test]
    fn empty() {
        let item = crate::mj_attributes_all::MjAttributesAll::default();
        assert_eq!("<mj-all />", item.print_dense().unwrap());
    }
}
