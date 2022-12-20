use super::{MJRaw, NAME};
use crate::json_children_deserializer;
use crate::json_children_serializer;
use serde::de::{Error, MapAccess};
use serde::ser::SerializeMap;
use std::fmt;

json_children_serializer!(MJRaw, NAME);
json_children_deserializer!(MJRaw, MJRawVisitor, NAME);

#[cfg(test)]
mod tests {
    use crate::mj_raw::MJRaw;
    use crate::text::Text;

    #[test]
    fn serialize() {
        let mut elt = MJRaw::default();
        elt.children.push(Text::from("Hello").into());
        elt.children.push(Text::from("World").into());
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-raw","children":["Hello","World"]}"#
        );
    }

    #[test]
    fn deserialize() {
        let json = r#"{"type":"mj-raw","children":["Hello","World"]}"#;
        let res: MJRaw = serde_json::from_str(json).unwrap();
        assert_eq!(res.children.len(), 2);
        let next = serde_json::to_string(&res).unwrap();
        assert_eq!(next, json);
    }
}
