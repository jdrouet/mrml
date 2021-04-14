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

#[cfg(feature = "json")]
mod json {
    #[macro_export]
    macro_rules! json_attrs_serializer {
        ($structure:ident, $name:ident) => {
            impl serde::Serialize for $structure {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    let mut map = serializer.serialize_map(Some(2))?;
                    map.serialize_entry("type", $name)?;
                    if !self.attributes.is_empty() {
                        map.serialize_entry("attributes", &self.attributes)?;
                    }
                    map.end()
                }
            }
        };
    }
    #[macro_export]
    macro_rules! json_children_serializer {
        ($structure:ident, $name:ident) => {
            impl serde::Serialize for $structure {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    let mut map = serializer.serialize_map(Some(2))?;
                    map.serialize_entry("type", $name)?;
                    if !self.children.is_empty() {
                        map.serialize_entry("children", &self.children)?;
                    }
                    map.end()
                }
            }
        };
    }
    #[macro_export]
    macro_rules! json_attrs_and_children_serializer {
        ($structure:ident, $name:ident) => {
            impl serde::Serialize for $structure {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    let mut map = serializer.serialize_map(Some(3))?;
                    map.serialize_entry("type", $name)?;
                    if !self.attributes.is_empty() {
                        map.serialize_entry("attributes", &self.attributes)?;
                    }
                    if !self.children.is_empty() {
                        map.serialize_entry("children", &self.children)?;
                    }
                    map.end()
                }
            }
        };
    }
}
