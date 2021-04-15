use super::MJSpacer;
use crate::print_attrs;

print_attrs!(MJSpacer, super::NAME);

#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let mut item = crate::mj_spacer::MJSpacer::default();
        item.attributes
            .insert("src".to_string(), "http://localhost".into());
        assert_eq!("<mj-spacer src=\"http://localhost\" />", item.dense_print());
    }
}
