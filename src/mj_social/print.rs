#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let item = crate::mj_social::MJSocial::default();
        assert_eq!("<mj-social />", item.dense_print());
    }
}
