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
