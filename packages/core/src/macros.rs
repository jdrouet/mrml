#[macro_export]
macro_rules! from_child {
    ($enum_name:ident, $child_name:ident) => {
        impl From<$child_name> for $enum_name {
            fn from(value: $child_name) -> Self {
                Self::$child_name(value)
            }
        }
    };
}

#[macro_export]
macro_rules! as_child {
    ($enum_name:ident, $child_name:ident, $func_name:ident) => {
        impl $enum_name {
            pub fn $func_name(&self) -> Option<&$child_name> {
                match self {
                    Self::$child_name(elt) => Some(elt),
                    _ => None,
                }
            }
        }
    };
}

#[macro_export]
macro_rules! parse_attribute {
    () => {
        fn parse_attribute<'a>(
            &mut self,
            name: xmlparser::StrSpan<'a>,
            value: xmlparser::StrSpan<'a>,
        ) -> Result<(), Error> {
            self.0
                .attributes
                .insert(name.to_string(), value.to_string());
            Ok(())
        }
    };
}

#[macro_export]
macro_rules! parse_child {
    ($child_parser:ident) => {
        fn parse_child_element<'a>(
            &mut self,
            tag: xmlparser::StrSpan<'a>,
            tokenizer: &mut xmlparser::Tokenizer<'a>,
        ) -> Result<(), Error> {
            self.0.children.push($child_parser::parse(tag, tokenizer)?);
            Ok(())
        }
    };
}

#[macro_export]
macro_rules! parse_comment {
    () => {
        fn parse_child_comment(&mut self, value: xmlparser::StrSpan) -> Result<(), Error> {
            self.0
                .children
                .push(crate::comment::Comment::from(value.as_str()).into());
            Ok(())
        }
    };
}

#[macro_export]
macro_rules! parse_text {
    () => {
        fn parse_child_text(&mut self, value: xmlparser::StrSpan) -> Result<(), Error> {
            self.0
                .children
                .push(crate::text::Text::from(value.as_str()).into());
            Ok(())
        }
    };
}

#[macro_export]
macro_rules! write_attribute {
    ($buf:ident, $name:expr, $value:expr) => {
        $buf.push(' ');
        $buf.push_str($name);
        $buf.push_str("=\"");
        $buf.push_str($value);
        $buf.push('"');
    };
}

#[macro_export]
macro_rules! write_optional_attribute {
    ($buf:ident, $name:expr, $value:expr) => {
        if let Some(ref value) = $value {
            write_attribute!($buf, $name, value);
        }
    };
}

#[macro_export]
macro_rules! write_style {
    ($buf:ident, $name:expr, $value:expr) => {
        $buf.push_str($name);
        $buf.push(':');
        $buf.push_str($value);
        $buf.push(';');
    };
}

#[macro_export]
macro_rules! write_optional_style {
    ($buf:ident, $name:expr, $value:expr) => {
        if let Some(ref value) = $value {
            write_style!($buf, $name, value);
        }
    };
}
