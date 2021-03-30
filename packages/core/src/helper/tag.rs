pub struct Tag {
    name: String,
    attributes: String,
    classes: String,
    styles: String,
}

impl Tag {
    pub fn table() -> Self {
        Self::new("table")
    }
    pub fn table_borderless() -> Self {
        Self::table()
            .add_attribute("border", "0")
            .add_attribute("cellpadding", "0")
            .add_attribute("cellspacing", "0")
    }
    pub fn table_presentation() -> Self {
        Self::table_borderless().add_attribute("role", "presentation")
    }
    pub fn tbody() -> Self {
        Self::new("tbody")
    }
    pub fn tr() -> Self {
        Self::new("tr")
    }
    pub fn td() -> Self {
        Self::new("td")
    }
    pub fn div() -> Self {
        Self::new("div")
    }

    pub fn new<T: ToString>(name: T) -> Self {
        Self {
            name: name.to_string(),
            attributes: String::new(),
            classes: String::new(),
            styles: String::new(),
        }
    }

    pub fn add_class<T: AsRef<str>>(mut self, value: T) -> Self {
        self.classes.push(' ');
        self.classes.push_str(value.as_ref());
        self
    }

    pub fn add_suffixed_class<T: AsRef<str>>(self, value: T, suffix: &str) -> Self {
        self.add_class(format!("{}-{}", value.as_ref(), suffix))
    }

    pub fn maybe_add_suffixed_class<T: AsRef<str>>(self, value: Option<T>, suffix: &str) -> Self {
        if let Some(value) = value {
            self.add_suffixed_class(value, suffix)
        } else {
            self
        }
    }

    pub fn maybe_add_class<T: AsRef<str>>(self, value: Option<T>) -> Self {
        if let Some(value) = value {
            self.add_class(value)
        } else {
            self
        }
    }

    pub fn add_attribute<T: AsRef<str>>(mut self, name: &str, value: T) -> Self {
        self.attributes.push(' ');
        self.attributes.push_str(name);
        self.attributes.push_str("=\"");
        self.attributes.push_str(value.as_ref());
        self.attributes.push('"');
        self
    }

    pub fn maybe_add_attribute<T: AsRef<str>>(self, name: &str, value: Option<T>) -> Self {
        if let Some(value) = value {
            self.add_attribute(name, value)
        } else {
            self
        }
    }

    pub fn add_style<T: AsRef<str>>(mut self, name: &str, value: T) -> Self {
        self.styles.push_str(name);
        self.styles.push(':');
        self.styles.push_str(value.as_ref());
        self.styles.push(';');
        self
    }

    pub fn maybe_add_style<T: AsRef<str>>(self, name: &str, value: Option<T>) -> Self {
        if let Some(value) = value {
            self.add_style(name, value)
        } else {
            self
        }
    }

    fn opening(&self) -> String {
        let mut res = String::from("<");
        res.push_str(&self.name);
        res.push_str(&self.attributes);
        if !self.classes.is_empty() {
            res.push_str(" class=\"");
            res.push_str(&self.classes);
            res.push('"');
        }
        if !self.styles.is_empty() {
            res.push_str(" style=\"");
            res.push_str(&self.styles);
            res.push('"');
        }
        res
    }

    pub fn open(&self) -> String {
        self.opening() + ">"
    }

    pub fn close(&self) -> String {
        format!("</{}>", self.name)
    }

    pub fn closed(&self) -> String {
        self.opening() + " />"
    }

    pub fn render<T: AsRef<str>>(&self, input: T) -> String {
        self.open() + input.as_ref() + &self.close()
    }
}
