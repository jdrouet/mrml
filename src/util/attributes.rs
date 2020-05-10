pub fn suffix_css_classes(input: Option<String>, suffix: &str) -> Option<String> {
    if let Some(value) = input {
        let value: Vec<String> = value
            .split(" ")
            .filter(|v| v.len() > 0)
            .map(|v| format!("{}-{}", suffix, v))
            .collect();
        if value.is_empty() {
            None
        } else {
            Some(value.join(" "))
        }
    } else {
        None
    }
}

pub fn suffix_unit(input: Option<String>, suffix: &str) -> Option<String> {
    input.and_then(|v| Some(format!("{}{}", v, suffix)))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn suffix_css_classes_none() {
        assert_eq!(suffix_css_classes(None, "whatever"), None);
    }

    #[test]
    fn suffix_css_classes_some_empty() {
        assert_eq!(suffix_css_classes(Some("".into()), "whatever"), None);
    }

    #[test]
    fn suffix_css_classes_with_values() {
        assert_eq!(
            suffix_css_classes(Some("toto tutu".into()), "whatever"),
            Some("whatever-toto whatever-tutu".into())
        );
    }
}
