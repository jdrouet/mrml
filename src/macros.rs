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

    #[macro_export]
    macro_rules! json_attrs_and_children_deserializer {
        ($structure:ident, $visitor:ident, $name:ident) => {
            const FIELDS: [&str; 3] = ["type", "attributes", "children"];

            #[derive(Default)]
            struct $visitor;

            impl<'de> serde::de::Visitor<'de> for $visitor {
                type Value = $structure;

                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_str("an map with properties type, attributes and children")
                }

                fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
                where
                    M: MapAccess<'de>,
                {
                    let mut result = $structure::default();
                    while let Some(key) = access.next_key::<String>()? {
                        if key == "type" {
                            if access.next_value::<String>()? != NAME {
                                return Err(M::Error::custom(format!(
                                    "expected type to equal {}",
                                    $name
                                )));
                            }
                        } else if key == "attributes" {
                            result.attributes = access.next_value()?;
                        } else if key == "children" {
                            result.children = access.next_value()?;
                        } else {
                            return Err(M::Error::unknown_field(&key, &FIELDS));
                        }
                    }
                    Ok(result)
                }
            }

            impl<'de> serde::Deserialize<'de> for $structure {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    deserializer.deserialize_map($visitor::default())
                }
            }
        };
    }
    #[macro_export]
    macro_rules! json_attrs_deserializer {
        ($structure:ident, $visitor:ident, $name:ident) => {
            const FIELDS: [&str; 2] = ["type", "attributes"];

            #[derive(Default)]
            struct $visitor;

            impl<'de> serde::de::Visitor<'de> for $visitor {
                type Value = $structure;

                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_str("an map with properties type and attributes")
                }

                fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
                where
                    M: MapAccess<'de>,
                {
                    let mut result = $structure::default();
                    while let Some(key) = access.next_key::<String>()? {
                        if key == "type" {
                            if access.next_value::<String>()? != NAME {
                                return Err(M::Error::custom(format!(
                                    "expected type to equal {}",
                                    $name
                                )));
                            }
                        } else if key == "attributes" {
                            result.attributes = access.next_value()?;
                        } else {
                            return Err(M::Error::unknown_field(&key, &FIELDS));
                        }
                    }
                    Ok(result)
                }
            }

            impl<'de> serde::Deserialize<'de> for $structure {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    deserializer.deserialize_map($visitor::default())
                }
            }
        };
    }
    #[macro_export]
    macro_rules! json_children_deserializer {
        ($structure:ident, $visitor:ident, $name:ident) => {
            const FIELDS: [&str; 2] = ["type", "children"];

            #[derive(Default)]
            struct $visitor;

            impl<'de> serde::de::Visitor<'de> for $visitor {
                type Value = $structure;

                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_str("an map with properties type and children")
                }

                fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
                where
                    M: MapAccess<'de>,
                {
                    let mut result = $structure::default();
                    while let Some(key) = access.next_key::<String>()? {
                        if key == "type" {
                            if access.next_value::<String>()? != NAME {
                                return Err(M::Error::custom(format!(
                                    "expected type to equal {}",
                                    $name
                                )));
                            }
                        } else if key == "children" {
                            result.children = access.next_value()?;
                        } else {
                            return Err(M::Error::unknown_field(&key, &FIELDS));
                        }
                    }
                    Ok(result)
                }
            }

            impl<'de> serde::Deserialize<'de> for $structure {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    deserializer.deserialize_map($visitor::default())
                }
            }
        };
    }
}

#[cfg(feature = "print")]
mod print {
    #[macro_export]
    macro_rules! print_display {
        ($structure:ident) => {
            use std::fmt;

            impl fmt::Display for $structure {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    f.write_str(self.dense_print().as_str())
                }
            }
        };
    }
    #[macro_export]
    macro_rules! print_attrs {
        ($structure:ident, $name:expr) => {
            use $crate::prelude::print::{self, Print};

            impl Print for $structure {
                fn print(&self, pretty: bool, level: usize, indent_size: usize) -> String {
                    print::open(
                        $name,
                        Some(&self.attributes),
                        true,
                        pretty,
                        level,
                        indent_size,
                    )
                }
            }

            $crate::print_display!($structure);
        };
    }
    #[macro_export]
    macro_rules! print_children {
        ($structure:ident, $name:expr) => {
            use $crate::prelude::print::{self, Print};

            impl Print for $structure {
                fn print(&self, pretty: bool, level: usize, indent_size: usize) -> String {
                    print::open($name, None, false, pretty, level, indent_size)
                        + &self
                            .children
                            .iter()
                            .map(|child| child.print(pretty, level + 1, indent_size))
                            .collect::<String>()
                        + &print::close(super::NAME, pretty, level, indent_size)
                }
            }

            $crate::print_display!($structure);
        };
    }
    #[macro_export]
    macro_rules! print_attrs_children {
        ($structure:ident, $name:expr) => {
            use $crate::prelude::print::{self, Print};

            impl Print for $structure {
                fn print(&self, pretty: bool, level: usize, indent_size: usize) -> String {
                    print::open(
                        $name,
                        Some(&self.attributes),
                        false,
                        pretty,
                        level,
                        indent_size,
                    ) + &self
                        .children
                        .iter()
                        .map(|child| child.print(pretty, level + 1, indent_size))
                        .collect::<String>()
                        + &print::close(super::NAME, pretty, level, indent_size)
                }
            }

            $crate::print_display!($structure);
        };
    }
}
