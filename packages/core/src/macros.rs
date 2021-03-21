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
macro_rules! parse_child {
    ($child_parser:ident) => {
        fn parse_child_element<'a>(
            &mut self,
            tag: StrSpan<'a>,
            tokenizer: &mut Tokenizer<'a>,
        ) -> Result<(), Error> {
            self.0.children.push($child_parser::parse(tag, tokenizer)?);
            Ok(())
        }
    };
}

#[macro_export]
macro_rules! parse_comment {
    () => {
        fn parse_child_comment(&mut self, value: StrSpan) -> Result<(), Error> {
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
        fn parse_child_text(&mut self, value: StrSpan) -> Result<(), Error> {
            self.0
                .children
                .push(crate::text::Text::from(value.as_str()).into());
            Ok(())
        }
    };
}
