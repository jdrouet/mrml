// use super::prelude::PropertyMap;
// use std::collections::HashMap;

// #[derive(Clone, Debug)]
// pub struct Properties {
//     inner: HashMap<String, String>,
// }

// impl Properties {
//     pub fn new() -> Self {
//         Self {
//             inner: HashMap::new(),
//         }
//     }

//     pub fn from(other: &HashMap<String, String>) -> Self {
//         let mut new = Self::new();
//         for (key, value) in other.iter() {
//             new.inner.insert(key.to_string(), value.to_string());
//         }
//         new
//     }
// }

// impl PropertyMap for Properties {
//     fn inner(&self) -> &HashMap<String, String> {
//         &self.inner
//     }
//     fn inner_mut(&mut self) -> &mut HashMap<String, String> {
//         &mut self.inner
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn sort_style() {
//         let mut props = Properties::new();
//         props.set("border-right", "3px");
//         props.set("border", "1px");
//         props.set("border-left", "2px");
//         assert_eq!(
//             props.as_style(),
//             "border:1px;border-left:2px;border-right:3px;"
//         );
//     }
// }
