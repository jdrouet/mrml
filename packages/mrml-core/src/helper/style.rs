use std::borrow::Cow;

#[derive(Default)]
pub struct Style {
    selectors: Vec<Cow<'static, str>>,
    content: Vec<Cow<'static, str>>,
}

impl Style {
    pub fn add_content<V: Into<Cow<'static, str>>>(mut self, value: V) -> Self {
        self.content.push(value.into());
        self
    }

    pub fn add_selector<V: Into<Cow<'static, str>>>(mut self, name: V) -> Self {
        self.selectors.push(name.into());
        self
    }
}

impl ToString for Style {
    fn to_string(&self) -> String {
        let selectors = self.selectors.join(",\n");
        let content = self.content.join("\n");
        format!("{selectors} {{ {content} }}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let result = Style::default()
            .add_selector("body".to_string())
            .add_selector("main")
            .add_content("background: red;".to_string())
            .add_content("color: blue;")
            .to_string();
        let expected = "body,\nmain { background: red;\ncolor: blue; }";
        assert_eq!(result, expected);
    }
}
