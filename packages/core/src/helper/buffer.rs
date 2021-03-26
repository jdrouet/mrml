#[derive(Default)]
pub struct Buffer(String);

impl Buffer {
    pub fn tag_open(&mut self, name: &str) {
        self.0.push('<');
        self.0.push_str(name);
    }

    pub fn tag_close(&mut self, name: &str) {
        self.0.push_str("</");
        self.0.push_str(name);
        self.0.push('>');
    }

    pub fn push_attribute(&mut self, name: &str, value: &str) {
        self.0.push(' ');
        self.0.push_str(name);
        self.0.push_str("=\"");
        self.0.push_str(value);
        self.0.push('"');
    }

    pub fn push_optional_attribute<T: AsRef<str>>(&mut self, name: &str, value: Option<T>) {
        if let Some(val) = value {
            self.push_attribute(name, val.as_ref());
        }
    }

    pub fn push_style(&mut self, name: &str, value: &str) {
        self.0.push_str(name);
        self.0.push(':');
        self.0.push_str(value);
        self.0.push(';');
    }

    pub fn push_optional_style<T: AsRef<str>>(&mut self, name: &str, value: Option<T>) {
        if let Some(val) = value {
            self.push_style(name, val.as_ref());
        }
    }

    pub fn push(&mut self, value: char) {
        self.0.push(value);
    }

    pub fn push_str<T: AsRef<str>>(&mut self, value: T) {
        self.0.push_str(value.as_ref());
    }

    pub fn content(self) -> String {
        self.0
    }
}
