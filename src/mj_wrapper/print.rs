use super::MJWrapper;
use crate::print_attrs_children;

print_attrs_children!(MJWrapper, super::NAME);

#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let item = crate::mj_wrapper::MJWrapper::default();
        assert_eq!("<mj-wrapper></mj-wrapper>", item.dense_print());
    }
}
