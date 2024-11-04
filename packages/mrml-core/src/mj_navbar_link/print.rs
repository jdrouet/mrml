#[cfg(test)]
mod tests {
    use crate::prelude::print::Printable;

    #[test]
    fn empty() {
        let mut item = crate::mj_navbar_link::MjNavbarLink::default();
        item.attributes
            .insert("src".to_string(), Some("http://localhost".into()));
        assert_eq!(
            "<mj-navbar-link src=\"http://localhost\" />",
            item.print_dense().unwrap()
        );
    }
}
