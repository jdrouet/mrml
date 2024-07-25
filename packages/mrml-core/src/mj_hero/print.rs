#[cfg(test)]
mod tests {
    use crate::prelude::print::Printable;

    #[test]
    fn empty() {
        let item = crate::mj_hero::MjHero::default();
        assert_eq!("<mj-hero />", item.print_dense().unwrap());
    }
}
