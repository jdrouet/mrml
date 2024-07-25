use serde::{de::MapAccess, ser::SerializeMap};
use std::marker::PhantomData;

use super::{hash::Map, Component};

impl<Child: serde::Serialize> serde::Serialize
    for super::Component<String, Map<String, String>, Vec<Child>>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(3))?;
        map.serialize_entry("type", self.tag.as_str())?;
        if !self.attributes.is_empty() {
            map.serialize_entry("attributes", &self.attributes)?;
        }
        if !self.children.is_empty() {
            map.serialize_entry("children", &self.children)?;
        }
        map.end()
    }
}

struct ComponentAsMapVisitor<Attributes, Children> {
    attributes: PhantomData<Attributes>,
    children: PhantomData<Children>,
}

impl<Attributes, Children> Default for ComponentAsMapVisitor<Attributes, Children> {
    fn default() -> Self {
        Self {
            attributes: PhantomData::<Attributes>,
            children: PhantomData::<Children>,
        }
    }
}

impl<
        'de,
        Attributes: Default + serde::de::Deserialize<'de>,
        Children: Default + serde::de::Deserialize<'de>,
    > serde::de::Deserialize<'de> for super::Component<String, Attributes, Children>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(ComponentAsMapVisitor::<Attributes, Children>::default())
    }
}

#[derive(serde::Deserialize)]
#[serde(field_identifier, rename_all = "lowercase")]
enum ComponentField {
    Type,
    Attributes,
    Children,
}

impl<
        'de,
        Attributes: Default + serde::de::Deserialize<'de>,
        Children: Default + serde::de::Deserialize<'de>,
    > serde::de::Visitor<'de> for ComponentAsMapVisitor<Attributes, Children>
{
    type Value = super::Component<String, Attributes, Children>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("struct Component")
    }

    fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
    where
        V: MapAccess<'de>,
    {
        let mut rtype: Option<String> = None;
        let mut attributes: Option<Attributes> = None;
        let mut children: Option<Children> = None;
        while let Some(key) = map.next_key()? {
            match key {
                ComponentField::Type => {
                    if rtype.is_some() {
                        return Err(serde::de::Error::duplicate_field("type"));
                    }
                    rtype = Some(map.next_value()?);
                }
                ComponentField::Attributes => {
                    if attributes.is_some() {
                        return Err(serde::de::Error::duplicate_field("attributes"));
                    }
                    attributes = Some(map.next_value()?);
                }
                ComponentField::Children => {
                    if children.is_some() {
                        return Err(serde::de::Error::duplicate_field("children"));
                    }
                    children = Some(map.next_value()?);
                }
            }
        }
        let rtype = rtype.ok_or_else(|| serde::de::Error::missing_field("type"))?;
        Ok(Component {
            tag: rtype,
            attributes: attributes.unwrap_or_default(),
            children: children.unwrap_or_default(),
        })
    }
}
