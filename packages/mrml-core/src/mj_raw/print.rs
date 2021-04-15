use super::MJRaw;
use crate::print_children;

print_children!(MJRaw, super::NAME);

#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let item = crate::mj_raw::MJRaw::default();
        assert_eq!("<mj-raw></mj-raw>", item.dense_print());
    }
}
