use serde::{
    de::{MapAccess, Unexpected},
    ser::SerializeMap,
};
use std::{hash::Hash, marker::PhantomData};

use super::{hash::Map, Component, StaticTag};

trait ComponentAttributes {
    fn has_attributes(&self) -> bool;
}

impl ComponentAttributes for () {
    #[inline]
    fn has_attributes(&self) -> bool {
        false
    }
}

impl<K: Hash + Eq, V> ComponentAttributes for Map<K, V> {
    #[inline]
    fn has_attributes(&self) -> bool {
        !self.is_empty()
    }
}

trait ComponentChildren {
    fn has_children(&self) -> bool;
}

impl ComponentChildren for () {
    #[inline]
    fn has_children(&self) -> bool {
        false
    }
}

impl<V> ComponentChildren for Vec<V> {
    #[inline]
    fn has_children(&self) -> bool {
        !self.is_empty()
    }
}

impl ComponentChildren for String {
    #[inline]
    fn has_children(&self) -> bool {
        !self.is_empty()
    }
}

struct DeserializableTag<T>(pub T);

impl<'de> serde::de::Deserialize<'de> for DeserializableTag<String> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self(String::deserialize(deserializer)?))
    }
}

impl<'de, T: StaticTag> serde::de::Deserialize<'de> for DeserializableTag<PhantomData<T>> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        let expected = T::static_tag();
        if value != expected {
            Err(serde::de::Error::invalid_value(
                Unexpected::Str(&value),
                &expected,
            ))
        } else {
            Ok(Self(PhantomData::<T>))
        }
    }
}

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

impl<
        Tag: StaticTag,
        Attributes: ComponentAttributes + serde::Serialize,
        Children: ComponentChildren + serde::Serialize,
    > serde::Serialize for super::Component<PhantomData<Tag>, Attributes, Children>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(3))?;
        map.serialize_entry("type", Tag::static_tag())?;
        if self.attributes.has_attributes() {
            map.serialize_entry("attributes", &self.attributes)?;
        }
        if self.children.has_children() {
            map.serialize_entry("children", &self.children)?;
        }
        map.end()
    }
}

struct ComponentAsMapVisitor<Tag, Attributes, Children> {
    tag: PhantomData<Tag>,
    attributes: PhantomData<Attributes>,
    children: PhantomData<Children>,
}

impl<Tag, Attributes, Children> Default for ComponentAsMapVisitor<Tag, Attributes, Children> {
    fn default() -> Self {
        Self {
            tag: PhantomData::<Tag>,
            attributes: PhantomData::<Attributes>,
            children: PhantomData::<Children>,
        }
    }
}

impl<
        'de,
        Tag,
        Attributes: Default + serde::de::Deserialize<'de>,
        Children: Default + serde::de::Deserialize<'de>,
    > serde::de::Deserialize<'de> for super::Component<Tag, Attributes, Children>
where
    DeserializableTag<Tag>: serde::de::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(ComponentAsMapVisitor::<Tag, Attributes, Children>::default())
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
        Tag,
        Attributes: Default + serde::de::Deserialize<'de>,
        Children: Default + serde::de::Deserialize<'de>,
    > serde::de::Visitor<'de> for ComponentAsMapVisitor<Tag, Attributes, Children>
where
    DeserializableTag<Tag>: serde::de::Deserialize<'de>,
{
    type Value = super::Component<Tag, Attributes, Children>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("struct Component")
    }

    fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
    where
        V: MapAccess<'de>,
    {
        let mut rtype: Option<Tag> = None;
        let mut attributes: Option<Attributes> = None;
        let mut children: Option<Children> = None;
        while let Some(key) = map.next_key()? {
            match key {
                ComponentField::Type => {
                    if rtype.is_some() {
                        return Err(serde::de::Error::duplicate_field("type"));
                    }
                    let DeserializableTag(tag) = map.next_value()?;
                    rtype = Some(tag);
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
