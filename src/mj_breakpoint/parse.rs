#[cfg(test)]
mod tests {
    #[test]
    fn success() {
        let res = crate::mjml::MJML::parse(
            r#"<mjml><mj-head><mj-breakpoint width="42px" /></mj-head></mjml>"#,
        );
        assert!(res.is_ok());
    }

    #[test]
    fn unexpected_attributes() {
        let res = crate::mjml::MJML::parse(
            r#"<mjml><mj-head><mj-breakpoint whatever="42px" /></mj-head></mjml>"#,
        );
        assert!(res.is_err());
    }
}
