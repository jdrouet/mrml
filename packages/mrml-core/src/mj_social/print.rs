#[cfg(test)]
mod tests {
    use crate::prelude::print::Printable;

    #[test]
    fn empty() {
        let item = crate::mj_social::MjSocial::default();
        assert_eq!("<mj-social />", item.print_dense().unwrap());
    }
}
