#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let item = crate::mj_attributes_all::MJAttributesAll::default();
        assert_eq!("<mj-all />", item.dense_print());
    }
}
