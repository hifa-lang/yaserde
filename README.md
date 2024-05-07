# hifa yaserde &emsp; [![Build Status]][travis] [![Latest Version]][crates.io] [![Coverage Status]][coveralls]

[Build Status]: https://travis-ci.com/hifa-lang/yaserde.svg?branch=master
[travis]: https://travis-ci.com/hifa-lang/yaserde
[Latest Version]: https://img.shields.io/crates/v/hifa_yaserde.svg
[crates.io]: https://crates.io/crates/hifa_yaserde
[Coverage Status]: https://coveralls.io/repos/github/hifa-lang/yaserde/badge.svg?branch=master
[coveralls]: https://coveralls.io/github/hifa-lang/yaserde?branch=master

**Yet Another Serializer/Deserializer specialized for XML**

This is a modified version of the project in [media-io/yaserde](https://github.com/media-io/xml-schema).

## Goal

This library will support XML de/ser-ializing with all specific features.

## Supported types

- [x] Struct
- [x] Vec<AnyType>
- [x] Enum
- [x] Enum with complex types
- [x] Option
- [x] String
- [x] bool
- [x] number (u8, i8, u32, i32, f32, f64)

## Attributes

- [x] **attribute**: this field is defined as an attribute
- [x] **default**: defines the default function to init the field
- [x] **flatten**: Flatten the contents of the field
- [x] **namespace**: defines the namespace of the field
- [x] **rename**: be able to rename a field
- [x] **root**: rename the based element. Used only at the XML root.
- [x] **skip_serializing**: Exclude this field from the serialized output. [More details...](doc/skip_serializing.md)
- [x] **skip_serializing_if**: Skip the serialisation for this field if the condition is true. [More details...](doc/skip_serializing.md)
- [x] **text**: this field match to the text content

## Custom De/Ser-rializer

Any type can define a custom deserializer and/or serializer.
To implement it, define the implementation of YaDeserialize/YaSerialize

```rust
impl YaDeserialize for MyType {
  fn deserialize<R: Read>(reader: &mut hifa_yaserde::de::Deserializer<R>) -> Result<Self, String> {
    // deserializer code
  }
}
```

```rust

impl YaSerialize for MyType {
  fn serialize<W: Write>(&self, writer: &mut hifa_yaserde::ser::Serializer<W>) -> Result<(), String> {
    // serializer code
  }
}
```
