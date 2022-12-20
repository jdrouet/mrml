use super::{MJHero, NAME};
use crate::print_attrs_children;

print_attrs_children!(MJHero, NAME);

#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let item = crate::mj_hero::MJHero::default();
        assert_eq!("<mj-hero></mj-hero>", item.dense_print());
    }
}
