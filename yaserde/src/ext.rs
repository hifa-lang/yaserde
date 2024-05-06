use crate::{YaDeserialize, YaSerialize};
use std::{
  io::{Read, Write},
  ops::{Deref, DerefMut},
};
use xml::{attribute::OwnedAttribute, namespace::Namespace};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Boxed<T> {
  inner: Box<T>,
}

impl<T: YaDeserialize> YaDeserialize for Boxed<T> {
  fn deserialize<R: Read>(reader: &mut crate::de::Deserializer<R>) -> Result<Self, String> {
    Ok(Self {
      inner: Box::new(T::deserialize(reader)?),
    })
  }
}

impl<T: YaSerialize> YaSerialize for Boxed<T> {
  fn serialize<W: Write>(&self, writer: &mut crate::ser::Serializer<W>) -> Result<(), String> {
    self.inner.as_ref().serialize(writer)
  }

  fn serialize_attributes(
    &self,
    attributes: Vec<OwnedAttribute>,
    namespace: Namespace,
  ) -> Result<(Vec<OwnedAttribute>, Namespace), String> {
    self
      .inner
      .as_ref()
      .serialize_attributes(attributes, namespace)
  }
}

impl<T> Deref for Boxed<T> {
  type Target = T;
  fn deref(&self) -> &Self::Target {
    &self.inner
  }
}

impl<T> DerefMut for Boxed<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.inner
  }
}

impl<T> From<T> for Boxed<T> {
  fn from(t: T) -> Self {
    Self { inner: Box::new(t) }
  }
}
