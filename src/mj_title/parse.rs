#[cfg(test)]
mod tests {
    #[test]
    fn success() {
        let res = crate::mjml::MJML::parse(
            r#"<mjml><mj-head><mj-title>Hello World!</mj-title></mj-head></mjml>"#,
        );
        assert!(res.is_ok());
    }
}
