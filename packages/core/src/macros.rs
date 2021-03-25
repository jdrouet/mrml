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
