#[derive(Default)]
pub struct Style {
    selectors: Vec<String>,
    content: Vec<String>,
}

impl Style {
    pub fn add_content(mut self, value: String) -> Self {
        self.content.push(value);
        self
    }

    pub fn add_str_content(self, value: &str) -> Self {
        self.add_content(value.to_string())
    }

    pub fn add_selector(mut self, name: String) -> Self {
        self.selectors.push(name);
        self
    }

    pub fn add_str_selector(self, name: &str) -> Self {
        self.add_selector(name.to_string())
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
            .add_selector("body".into())
            .add_str_selector("main")
            .add_content("background: red;".into())
            .add_str_content("color: blue;")
            .to_string();
        let expected = "body,\nmain { background: red;\ncolor: blue; }";
        assert_eq!(result, expected);
    }
}
