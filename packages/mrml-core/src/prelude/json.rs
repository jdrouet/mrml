use std::hash::Hash;
use std::marker::PhantomData;

use serde::de::{MapAccess, Unexpected};
use serde::ser::SerializeMap;

use super::hash::Map;
use super::{Component, StaticTag};

pub trait JsonAttributes: serde::Serialize {
    fn has_attributes(&self) -> bool;
    fn try_from_serde<Err: serde::de::Error>(this: Option<Self>) -> Result<Self, Err>
    where
        Self: Sized;
}

impl JsonAttributes for () {
    #[inline]
    fn has_attributes(&self) -> bool {
        false
    }

    fn try_from_serde<Err: serde::de::Error>(_: Option<Self>) -> Result<Self, Err>
    where
        Self: Sized,
    {
        Ok(())
    }
}

impl<K: Hash + Eq + Default + serde::Serialize, V: Default + serde::Serialize> JsonAttributes
    for Map<K, V>
{
    #[inline]
    fn has_attributes(&self) -> bool {
        !self.is_empty()
    }

    fn try_from_serde<Err: serde::de::Error>(this: Option<Self>) -> Result<Self, Err>
    where
        Self: Sized,
    {
        Ok(this.unwrap_or_default())
    }
}

pub trait JsonChildren: serde::Serialize {
    fn has_children(&self) -> bool;

    fn try_from_serde<Err: serde::de::Error>(this: Option<Self>) -> Result<Self, Err>
    where
        Self: Sized;
}

impl JsonChildren for () {
    #[inline]
    fn has_children(&self) -> bool {
        false
    }

    fn try_from_serde<Err: serde::de::Error>(_: Option<Self>) -> Result<Self, Err>
    where
        Self: Sized,
    {
        Ok(())
    }
}

impl<V: serde::Serialize> JsonChildren for Vec<V> {
    #[inline]
    fn has_children(&self) -> bool {
        !self.is_empty()
    }

    fn try_from_serde<Err: serde::de::Error>(this: Option<Self>) -> Result<Self, Err>
    where
        Self: Sized,
    {
        Ok(this.unwrap_or_default())
    }
}

impl JsonChildren for String {
    #[inline]
    fn has_children(&self) -> bool {
        !self.is_empty()
    }

    fn try_from_serde<Err: serde::de::Error>(this: Option<Self>) -> Result<Self, Err>
    where
        Self: Sized,
    {
        Ok(this.unwrap_or_default())
    }
}

pub(crate) struct DeserializableTag<T>(pub T);

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
    for super::Component<String, crate::prelude::AttributeMap, Vec<Child>>
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

impl<Tag: StaticTag, Attributes: JsonAttributes, Children: JsonChildren> serde::Serialize
    for super::Component<PhantomData<Tag>, Attributes, Children>
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

impl<'de, Tag, Attributes, Children> serde::de::Deserialize<'de>
    for super::Component<Tag, Attributes, Children>
where
    DeserializableTag<Tag>: serde::de::Deserialize<'de>,
    Attributes: JsonAttributes + serde::de::Deserialize<'de>,
    Children: JsonChildren + serde::de::Deserialize<'de>,
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

impl<'de, Tag, Attributes, Children> serde::de::Visitor<'de>
    for ComponentAsMapVisitor<Tag, Attributes, Children>
where
    DeserializableTag<Tag>: serde::de::Deserialize<'de>,
    Attributes: JsonAttributes + serde::de::Deserialize<'de>,
    Children: JsonChildren + serde::de::Deserialize<'de>,
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
                ComponentField::Type if rtype.is_some() => {
                    return Err(serde::de::Error::duplicate_field("type"));
                }
                ComponentField::Type => {
                    let DeserializableTag(tag) = map.next_value()?;
                    rtype = Some(tag);
                }
                ComponentField::Attributes if attributes.is_some() => {
                    return Err(serde::de::Error::duplicate_field("attributes"));
                }
                ComponentField::Attributes => {
                    attributes = Some(map.next_value()?);
                }
                ComponentField::Children if children.is_some() => {
                    return Err(serde::de::Error::duplicate_field("children"));
                }
                ComponentField::Children => {
                    children = Some(map.next_value()?);
                }
            }
        }

        let rtype = rtype.ok_or_else(|| serde::de::Error::missing_field("type"))?;
        Ok(Component {
            tag: rtype,
            attributes: Attributes::try_from_serde(attributes)?,
            children: Children::try_from_serde(children)?,
        })
    }
}

impl<T> JsonChildren for super::OneOrMany<T>
where
    T: serde::Serialize,
{
    fn has_children(&self) -> bool {
        match self {
            Self::One(_) => true,
            Self::Many(inner) => !inner.is_empty(),
        }
    }

    fn try_from_serde<Err: serde::de::Error>(this: Option<Self>) -> Result<Self, Err>
    where
        Self: Sized,
    {
        match this {
            Some(item) => Ok(item),
            None => Ok(super::OneOrMany::Many(Vec::default())),
        }
    }
}
