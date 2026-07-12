[bevy](../../index.html)::[reflect](../index.html)::[erased\_serde](index.html)

# Trait Deserializer 

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#91)

```rust
pub trait Deserializer<'de>: Sealed {
    // Required methods
    fn erased_deserialize_any(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> Result<Out, Error>;
    fn erased_deserialize_bool(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> Result<Out, Error>;
    fn erased_deserialize_i8(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> Result<Out, Error>;
    fn erased_deserialize_i16(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> Result<Out, Error>;
    fn erased_deserialize_i32(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> Result<Out, Error>;
    fn erased_deserialize_i64(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> Result<Out, Error>;
    fn erased_deserialize_i128(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> Result<Out, Error>;
    fn erased_deserialize_u8(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> Result<Out, Error>;
    fn erased_deserialize_u16(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> Result<Out, Error>;
    fn erased_deserialize_u32(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> Result<Out, Error>;
    fn erased_deserialize_u64(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> Result<Out, Error>;
    fn erased_deserialize_u128(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> Result<Out, Error>;
    fn erased_deserialize_f32(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> Result<Out, Error>;
    fn erased_deserialize_f64(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> Result<Out, Error>;
    fn erased_deserialize_char(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> Result<Out, Error>;
    fn erased_deserialize_str(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> Result<Out, Error>;
    fn erased_deserialize_string(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> Result<Out, Error>;
    fn erased_deserialize_bytes(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> Result<Out, Error>;
    fn erased_deserialize_byte_buf(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> Result<Out, Error>;
    fn erased_deserialize_option(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> Result<Out, Error>;
    fn erased_deserialize_unit(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> Result<Out, Error>;
    fn erased_deserialize_unit_struct(
        &mut self,
        name: &'static str,
        visitor: &mut dyn Visitor<'de>,
    ) -> Result<Out, Error>;
    fn erased_deserialize_newtype_struct(
        &mut self,
        name: &'static str,
        visitor: &mut dyn Visitor<'de>,
    ) -> Result<Out, Error>;
    fn erased_deserialize_seq(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> Result<Out, Error>;
    fn erased_deserialize_tuple(
        &mut self,
        len: usize,
        visitor: &mut dyn Visitor<'de>,
    ) -> Result<Out, Error>;
    fn erased_deserialize_tuple_struct(
        &mut self,
        name: &'static str,
        len: usize,
        visitor: &mut dyn Visitor<'de>,
    ) -> Result<Out, Error>;
    fn erased_deserialize_map(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> Result<Out, Error>;
    fn erased_deserialize_struct(
        &mut self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: &mut dyn Visitor<'de>,
    ) -> Result<Out, Error>;
    fn erased_deserialize_identifier(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> Result<Out, Error>;
    fn erased_deserialize_enum(
        &mut self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: &mut dyn Visitor<'de>,
    ) -> Result<Out, Error>;
    fn erased_deserialize_ignored_any(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> Result<Out, Error>;
    fn erased_is_human_readable(&self) -> bool;
}
```

An object-safe equivalent of Serde’s `Deserializer` trait.

Any implementation of Serde’s `Deserializer` can be converted to a `&dyn erased_serde::Deserializer` or `Box<dyn erased_serde::Deserializer>` trait object using `erased_serde::Deserializer::erase`.

```rust
use erased_serde::Deserializer;
use std::collections::BTreeMap as Map;

fn main() {
    static JSON: &'static [u8] = br#"{"A": 65, "B": 66}"#;
    static CBOR: &'static [u8] = &[162, 97, 65, 24, 65, 97, 66, 24, 66];

    // Construct some deserializers.
    let json = &mut serde_json::Deserializer::from_slice(JSON);
    let cbor = &mut serde_cbor::Deserializer::from_slice(CBOR);

    // The values in this map are boxed trait objects, which is not possible
    // with the normal serde::Deserializer because of object safety.
    let mut formats: Map<&str, Box<dyn Deserializer>> = Map::new();
    formats.insert("json", Box::new(<dyn Deserializer>::erase(json)));
    formats.insert("cbor", Box::new(<dyn Deserializer>::erase(cbor)));

    // Pick a Deserializer out of the formats map.
    let format = formats.get_mut("json").unwrap();

    let data: Map<String, usize> = erased_serde::deserialize(format).unwrap();

    println!("{}", data["A"] + data["B"]);
}
```

This trait is sealed and can only be implemented via a `serde::Deserializer<'de>` impl.

## Required Methods

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#92)

#### fn [erased\_deserialize\_any](#tymethod.erased_deserialize_any)( &mut self, visitor: &mut dyn Visitor<'de>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Out, [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#93)

#### fn [erased\_deserialize\_bool](#tymethod.erased_deserialize_bool)( &mut self, visitor: &mut dyn Visitor<'de>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Out, [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#94)

#### fn [erased\_deserialize\_i8](#tymethod.erased_deserialize_i8)( &mut self, visitor: &mut dyn Visitor<'de>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Out, [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#95)

#### fn [erased\_deserialize\_i16](#tymethod.erased_deserialize_i16)( &mut self, visitor: &mut dyn Visitor<'de>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Out, [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#96)

#### fn [erased\_deserialize\_i32](#tymethod.erased_deserialize_i32)( &mut self, visitor: &mut dyn Visitor<'de>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Out, [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#97)

#### fn [erased\_deserialize\_i64](#tymethod.erased_deserialize_i64)( &mut self, visitor: &mut dyn Visitor<'de>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Out, [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#98)

#### fn [erased\_deserialize\_i128](#tymethod.erased_deserialize_i128)( &mut self, visitor: &mut dyn Visitor<'de>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Out, [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#99)

#### fn [erased\_deserialize\_u8](#tymethod.erased_deserialize_u8)( &mut self, visitor: &mut dyn Visitor<'de>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Out, [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#100)

#### fn [erased\_deserialize\_u16](#tymethod.erased_deserialize_u16)( &mut self, visitor: &mut dyn Visitor<'de>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Out, [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#101)

#### fn [erased\_deserialize\_u32](#tymethod.erased_deserialize_u32)( &mut self, visitor: &mut dyn Visitor<'de>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Out, [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#102)

#### fn [erased\_deserialize\_u64](#tymethod.erased_deserialize_u64)( &mut self, visitor: &mut dyn Visitor<'de>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Out, [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#103)

#### fn [erased\_deserialize\_u128](#tymethod.erased_deserialize_u128)( &mut self, visitor: &mut dyn Visitor<'de>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Out, [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#104)

#### fn [erased\_deserialize\_f32](#tymethod.erased_deserialize_f32)( &mut self, visitor: &mut dyn Visitor<'de>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Out, [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#105)

#### fn [erased\_deserialize\_f64](#tymethod.erased_deserialize_f64)( &mut self, visitor: &mut dyn Visitor<'de>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Out, [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#106)

#### fn [erased\_deserialize\_char](#tymethod.erased_deserialize_char)( &mut self, visitor: &mut dyn Visitor<'de>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Out, [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#107)

#### fn [erased\_deserialize\_str](#tymethod.erased_deserialize_str)( &mut self, visitor: &mut dyn Visitor<'de>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Out, [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#108)

#### fn [erased\_deserialize\_string](#tymethod.erased_deserialize_string)( &mut self, visitor: &mut dyn Visitor<'de>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Out, [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#109)

#### fn [erased\_deserialize\_bytes](#tymethod.erased_deserialize_bytes)( &mut self, visitor: &mut dyn Visitor<'de>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Out, [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#110-111)

#### fn [erased\_deserialize\_byte\_buf](#tymethod.erased_deserialize_byte_buf)( &mut self, visitor: &mut dyn Visitor<'de>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Out, [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#112)

#### fn [erased\_deserialize\_option](#tymethod.erased_deserialize_option)( &mut self, visitor: &mut dyn Visitor<'de>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Out, [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#113)

#### fn [erased\_deserialize\_unit](#tymethod.erased_deserialize_unit)( &mut self, visitor: &mut dyn Visitor<'de>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Out, [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#114-118)

#### fn [erased\_deserialize\_unit\_struct](#tymethod.erased_deserialize_unit_struct)( &mut self, name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), visitor: &mut dyn Visitor<'de>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Out, [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#119-123)

#### fn [erased\_deserialize\_newtype\_struct](#tymethod.erased_deserialize_newtype_struct)( &mut self, name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), visitor: &mut dyn Visitor<'de>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Out, [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#124)

#### fn [erased\_deserialize\_seq](#tymethod.erased_deserialize_seq)( &mut self, visitor: &mut dyn Visitor<'de>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Out, [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#125-129)

#### fn [erased\_deserialize\_tuple](#tymethod.erased_deserialize_tuple)( &mut self, len: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), visitor: &mut dyn Visitor<'de>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Out, [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#130-135)

#### fn [erased\_deserialize\_tuple\_struct](#tymethod.erased_deserialize_tuple_struct)( &mut self, name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), len: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), visitor: &mut dyn Visitor<'de>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Out, [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#136)

#### fn [erased\_deserialize\_map](#tymethod.erased_deserialize_map)( &mut self, visitor: &mut dyn Visitor<'de>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Out, [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#137-142)

#### fn [erased\_deserialize\_struct](#tymethod.erased_deserialize_struct)( &mut self, name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), fields: &'static \[&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\], visitor: &mut dyn Visitor<'de>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Out, [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#143-146)

#### fn [erased\_deserialize\_identifier](#tymethod.erased_deserialize_identifier)( &mut self, visitor: &mut dyn Visitor<'de>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Out, [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#147-152)

#### fn [erased\_deserialize\_enum](#tymethod.erased_deserialize_enum)( &mut self, name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), variants: &'static \[&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\], visitor: &mut dyn Visitor<'de>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Out, [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#153-156)

#### fn [erased\_deserialize\_ignored\_any](#tymethod.erased_deserialize_ignored_any)( &mut self, visitor: &mut dyn Visitor<'de>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Out, [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#157)

#### fn [erased\_is\_human\_readable](#tymethod.erased_is_human_readable)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

## Implementations

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#226)

### impl<'de> dyn [Deserializer](trait.Deserializer.html "trait bevy::reflect::erased_serde::Deserializer")<'de>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#227-262)

#### pub fn [erase](#method.erase)<D>(deserializer: D) -> Deserializer<D>

where D: [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>,

Available on **non-`docsrs`** only.

Convert any Serde `Deserializer` to a trait object.

```rust
use erased_serde::Deserializer;
use std::collections::BTreeMap as Map;

fn main() {
    static JSON: &'static [u8] = br#"{"A": 65, "B": 66}"#;
    static CBOR: &'static [u8] = &[162, 97, 65, 24, 65, 97, 66, 24, 66];

    // Construct some deserializers.
    let json = &mut serde_json::Deserializer::from_slice(JSON);
    let cbor = &mut serde_cbor::Deserializer::from_slice(CBOR);

    // The values in this map are boxed trait objects, which is not possible
    // with the normal serde::Deserializer because of object safety.
    let mut formats: Map<&str, Box<dyn Deserializer>> = Map::new();
    formats.insert("json", Box::new(<dyn Deserializer>::erase(json)));
    formats.insert("cbor", Box::new(<dyn Deserializer>::erase(cbor)));

    // Pick a Deserializer out of the formats map.
    let format = formats.get_mut("json").unwrap();

    let data: Map<String, usize> = erased_serde::deserialize(format).unwrap();

    println!("{}", data["A"] + data["B"]);
}
```

## Trait Implementations

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1108)

### impl<'de> [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de> for &mut dyn [Deserializer](trait.Deserializer.html "trait bevy::reflect::erased_serde::Deserializer")<'de>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1108)

#### type [Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#associatedtype.Error) = [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")

The error type that can be returned if some error occurs during deserialization.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1108)

#### fn [deserialize\_any](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_any)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Require the `Deserializer` to figure out how to drive the visitor based on what data type is in the input. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_any)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1108)

#### fn [deserialize\_bool](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_bool)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a `bool` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1108)

#### fn [deserialize\_i8](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_i8)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting an `i8` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1108)

#### fn [deserialize\_i16](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_i16)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting an `i16` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1108)

#### fn [deserialize\_i32](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_i32)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting an `i32` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1108)

#### fn [deserialize\_i64](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_i64)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting an `i64` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1108)

#### fn [deserialize\_i128](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#method.deserialize_i128)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting an `i128` value. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#method.deserialize_i128)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1108)

#### fn [deserialize\_u8](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_u8)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a `u8` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1108)

#### fn [deserialize\_u16](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_u16)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a `u16` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1108)

#### fn [deserialize\_u32](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_u32)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a `u32` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1108)

#### fn [deserialize\_u64](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_u64)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a `u64` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1108)

#### fn [deserialize\_u128](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#method.deserialize_u128)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting an `u128` value. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#method.deserialize_u128)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1108)

#### fn [deserialize\_f32](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_f32)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a `f32` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1108)

#### fn [deserialize\_f64](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_f64)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a `f64` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1108)

#### fn [deserialize\_char](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_char)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a `char` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1108)

#### fn [deserialize\_str](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_str)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a string value and does not benefit from taking ownership of buffered data owned by the `Deserializer`. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_str)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1108)

#### fn [deserialize\_string](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_string)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a string value and would benefit from taking ownership of buffered data owned by the `Deserializer`. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_string)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1108)

#### fn [deserialize\_bytes](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_bytes)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a byte array and does not benefit from taking ownership of buffered data owned by the `Deserializer`. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_bytes)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1108)

#### fn [deserialize\_byte\_buf](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_byte_buf)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a byte array and would benefit from taking ownership of buffered data owned by the `Deserializer`. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_byte_buf)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1108)

#### fn [deserialize\_option](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_option)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting an optional value. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_option)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1108)

#### fn [deserialize\_unit](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_unit)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a unit value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1108)

#### fn [deserialize\_unit\_struct](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_unit_struct)<V>( self, name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a unit struct with a particular name.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1108)

#### fn [deserialize\_newtype\_struct](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_newtype_struct)<V>( self, name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a newtype struct with a particular name.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1108)

#### fn [deserialize\_seq](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_seq)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a sequence of values.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1108)

#### fn [deserialize\_tuple](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_tuple)<V>( self, len: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a sequence of values and knows how many values there are without looking at the serialized data.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1108)

#### fn [deserialize\_tuple\_struct](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_tuple_struct)<V>( self, name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), len: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a tuple struct with a particular name and number of fields.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1108)

#### fn [deserialize\_map](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_map)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a map of key-value pairs.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1108)

#### fn [deserialize\_struct](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_struct)<V>( self, name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), fields: &'static \[&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\], visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a struct with a particular name and fields.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1108)

#### fn [deserialize\_identifier](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_identifier)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting the name of a struct field or the discriminant of an enum variant.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1108)

#### fn [deserialize\_enum](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_enum)<V>( self, name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), variants: &'static \[&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\], visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting an enum value with a particular name and possible variants.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1108)

#### fn [deserialize\_ignored\_any](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_ignored_any)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type needs to deserialize a value whose type doesn’t matter because it is ignored. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_ignored_any)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1108)

#### fn [is\_human\_readable](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#method.is_human_readable)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Determine whether `Deserialize` implementations should expect to deserialize their human-readable form. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#method.is_human_readable)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1109)

### impl<'de> [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de> for &mut (dyn [Deserializer](trait.Deserializer.html "trait bevy::reflect::erased_serde::Deserializer")<'de> + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"))

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1109)

#### type [Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#associatedtype.Error) = [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")

The error type that can be returned if some error occurs during deserialization.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1109)

#### fn [deserialize\_any](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_any)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Require the `Deserializer` to figure out how to drive the visitor based on what data type is in the input. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_any)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1109)

#### fn [deserialize\_bool](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_bool)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a `bool` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1109)

#### fn [deserialize\_i8](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_i8)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting an `i8` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1109)

#### fn [deserialize\_i16](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_i16)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting an `i16` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1109)

#### fn [deserialize\_i32](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_i32)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting an `i32` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1109)

#### fn [deserialize\_i64](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_i64)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting an `i64` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1109)

#### fn [deserialize\_i128](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#method.deserialize_i128)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting an `i128` value. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#method.deserialize_i128)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1109)

#### fn [deserialize\_u8](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_u8)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a `u8` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1109)

#### fn [deserialize\_u16](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_u16)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a `u16` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1109)

#### fn [deserialize\_u32](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_u32)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a `u32` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1109)

#### fn [deserialize\_u64](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_u64)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a `u64` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1109)

#### fn [deserialize\_u128](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#method.deserialize_u128)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting an `u128` value. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#method.deserialize_u128)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1109)

#### fn [deserialize\_f32](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_f32)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a `f32` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1109)

#### fn [deserialize\_f64](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_f64)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a `f64` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1109)

#### fn [deserialize\_char](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_char)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a `char` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1109)

#### fn [deserialize\_str](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_str)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a string value and does not benefit from taking ownership of buffered data owned by the `Deserializer`. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_str)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1109)

#### fn [deserialize\_string](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_string)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a string value and would benefit from taking ownership of buffered data owned by the `Deserializer`. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_string)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1109)

#### fn [deserialize\_bytes](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_bytes)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a byte array and does not benefit from taking ownership of buffered data owned by the `Deserializer`. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_bytes)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1109)

#### fn [deserialize\_byte\_buf](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_byte_buf)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a byte array and would benefit from taking ownership of buffered data owned by the `Deserializer`. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_byte_buf)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1109)

#### fn [deserialize\_option](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_option)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting an optional value. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_option)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1109)

#### fn [deserialize\_unit](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_unit)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a unit value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1109)

#### fn [deserialize\_unit\_struct](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_unit_struct)<V>( self, name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a unit struct with a particular name.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1109)

#### fn [deserialize\_newtype\_struct](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_newtype_struct)<V>( self, name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a newtype struct with a particular name.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1109)

#### fn [deserialize\_seq](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_seq)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a sequence of values.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1109)

#### fn [deserialize\_tuple](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_tuple)<V>( self, len: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a sequence of values and knows how many values there are without looking at the serialized data.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1109)

#### fn [deserialize\_tuple\_struct](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_tuple_struct)<V>( self, name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), len: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a tuple struct with a particular name and number of fields.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1109)

#### fn [deserialize\_map](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_map)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a map of key-value pairs.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1109)

#### fn [deserialize\_struct](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_struct)<V>( self, name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), fields: &'static \[&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\], visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a struct with a particular name and fields.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1109)

#### fn [deserialize\_identifier](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_identifier)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting the name of a struct field or the discriminant of an enum variant.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1109)

#### fn [deserialize\_enum](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_enum)<V>( self, name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), variants: &'static \[&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\], visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting an enum value with a particular name and possible variants.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1109)

#### fn [deserialize\_ignored\_any](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_ignored_any)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type needs to deserialize a value whose type doesn’t matter because it is ignored. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_ignored_any)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1109)

#### fn [is\_human\_readable](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#method.is_human_readable)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Determine whether `Deserialize` implementations should expect to deserialize their human-readable form. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#method.is_human_readable)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1110)

### impl<'de> [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de> for &mut (dyn [Deserializer](trait.Deserializer.html "trait bevy::reflect::erased_serde::Deserializer")<'de> + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"))

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1110)

#### type [Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#associatedtype.Error) = [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")

The error type that can be returned if some error occurs during deserialization.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1110)

#### fn [deserialize\_any](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_any)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Require the `Deserializer` to figure out how to drive the visitor based on what data type is in the input. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_any)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1110)

#### fn [deserialize\_bool](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_bool)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a `bool` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1110)

#### fn [deserialize\_i8](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_i8)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting an `i8` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1110)

#### fn [deserialize\_i16](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_i16)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting an `i16` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1110)

#### fn [deserialize\_i32](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_i32)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting an `i32` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1110)

#### fn [deserialize\_i64](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_i64)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting an `i64` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1110)

#### fn [deserialize\_i128](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#method.deserialize_i128)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting an `i128` value. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#method.deserialize_i128)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1110)

#### fn [deserialize\_u8](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_u8)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a `u8` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1110)

#### fn [deserialize\_u16](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_u16)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a `u16` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1110)

#### fn [deserialize\_u32](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_u32)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a `u32` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1110)

#### fn [deserialize\_u64](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_u64)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a `u64` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1110)

#### fn [deserialize\_u128](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#method.deserialize_u128)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting an `u128` value. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#method.deserialize_u128)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1110)

#### fn [deserialize\_f32](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_f32)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a `f32` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1110)

#### fn [deserialize\_f64](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_f64)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a `f64` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1110)

#### fn [deserialize\_char](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_char)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a `char` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1110)

#### fn [deserialize\_str](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_str)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a string value and does not benefit from taking ownership of buffered data owned by the `Deserializer`. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_str)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1110)

#### fn [deserialize\_string](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_string)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a string value and would benefit from taking ownership of buffered data owned by the `Deserializer`. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_string)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1110)

#### fn [deserialize\_bytes](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_bytes)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a byte array and does not benefit from taking ownership of buffered data owned by the `Deserializer`. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_bytes)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1110)

#### fn [deserialize\_byte\_buf](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_byte_buf)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a byte array and would benefit from taking ownership of buffered data owned by the `Deserializer`. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_byte_buf)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1110)

#### fn [deserialize\_option](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_option)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting an optional value. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_option)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1110)

#### fn [deserialize\_unit](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_unit)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a unit value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1110)

#### fn [deserialize\_unit\_struct](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_unit_struct)<V>( self, name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a unit struct with a particular name.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1110)

#### fn [deserialize\_newtype\_struct](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_newtype_struct)<V>( self, name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a newtype struct with a particular name.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1110)

#### fn [deserialize\_seq](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_seq)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a sequence of values.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1110)

#### fn [deserialize\_tuple](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_tuple)<V>( self, len: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a sequence of values and knows how many values there are without looking at the serialized data.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1110)

#### fn [deserialize\_tuple\_struct](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_tuple_struct)<V>( self, name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), len: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a tuple struct with a particular name and number of fields.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1110)

#### fn [deserialize\_map](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_map)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a map of key-value pairs.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1110)

#### fn [deserialize\_struct](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_struct)<V>( self, name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), fields: &'static \[&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\], visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a struct with a particular name and fields.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1110)

#### fn [deserialize\_identifier](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_identifier)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting the name of a struct field or the discriminant of an enum variant.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1110)

#### fn [deserialize\_enum](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_enum)<V>( self, name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), variants: &'static \[&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\], visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting an enum value with a particular name and possible variants.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1110)

#### fn [deserialize\_ignored\_any](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_ignored_any)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type needs to deserialize a value whose type doesn’t matter because it is ignored. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_ignored_any)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1110)

#### fn [is\_human\_readable](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#method.is_human_readable)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Determine whether `Deserialize` implementations should expect to deserialize their human-readable form. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#method.is_human_readable)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1111)

### impl<'de> [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de> for &mut (dyn [Deserializer](trait.Deserializer.html "trait bevy::reflect::erased_serde::Deserializer")<'de> + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"))

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1111)

#### type [Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#associatedtype.Error) = [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")

The error type that can be returned if some error occurs during deserialization.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1111)

#### fn [deserialize\_any](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_any)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Require the `Deserializer` to figure out how to drive the visitor based on what data type is in the input. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_any)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1111)

#### fn [deserialize\_bool](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_bool)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a `bool` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1111)

#### fn [deserialize\_i8](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_i8)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting an `i8` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1111)

#### fn [deserialize\_i16](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_i16)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting an `i16` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1111)

#### fn [deserialize\_i32](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_i32)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting an `i32` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1111)

#### fn [deserialize\_i64](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_i64)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting an `i64` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1111)

#### fn [deserialize\_i128](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#method.deserialize_i128)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting an `i128` value. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#method.deserialize_i128)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1111)

#### fn [deserialize\_u8](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_u8)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a `u8` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1111)

#### fn [deserialize\_u16](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_u16)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a `u16` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1111)

#### fn [deserialize\_u32](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_u32)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a `u32` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1111)

#### fn [deserialize\_u64](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_u64)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a `u64` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1111)

#### fn [deserialize\_u128](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#method.deserialize_u128)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting an `u128` value. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#method.deserialize_u128)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1111)

#### fn [deserialize\_f32](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_f32)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a `f32` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1111)

#### fn [deserialize\_f64](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_f64)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a `f64` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1111)

#### fn [deserialize\_char](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_char)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a `char` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1111)

#### fn [deserialize\_str](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_str)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a string value and does not benefit from taking ownership of buffered data owned by the `Deserializer`. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_str)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1111)

#### fn [deserialize\_string](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_string)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a string value and would benefit from taking ownership of buffered data owned by the `Deserializer`. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_string)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1111)

#### fn [deserialize\_bytes](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_bytes)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a byte array and does not benefit from taking ownership of buffered data owned by the `Deserializer`. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_bytes)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1111)

#### fn [deserialize\_byte\_buf](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_byte_buf)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a byte array and would benefit from taking ownership of buffered data owned by the `Deserializer`. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_byte_buf)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1111)

#### fn [deserialize\_option](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_option)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting an optional value. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_option)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1111)

#### fn [deserialize\_unit](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_unit)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a unit value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1111)

#### fn [deserialize\_unit\_struct](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_unit_struct)<V>( self, name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a unit struct with a particular name.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1111)

#### fn [deserialize\_newtype\_struct](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_newtype_struct)<V>( self, name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a newtype struct with a particular name.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1111)

#### fn [deserialize\_seq](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_seq)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a sequence of values.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1111)

#### fn [deserialize\_tuple](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_tuple)<V>( self, len: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a sequence of values and knows how many values there are without looking at the serialized data.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1111)

#### fn [deserialize\_tuple\_struct](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_tuple_struct)<V>( self, name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), len: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a tuple struct with a particular name and number of fields.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1111)

#### fn [deserialize\_map](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_map)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a map of key-value pairs.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1111)

#### fn [deserialize\_struct](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_struct)<V>( self, name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), fields: &'static \[&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\], visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a struct with a particular name and fields.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1111)

#### fn [deserialize\_identifier](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_identifier)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting the name of a struct field or the discriminant of an enum variant.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1111)

#### fn [deserialize\_enum](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_enum)<V>( self, name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), variants: &'static \[&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\], visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting an enum value with a particular name and possible variants.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1111)

#### fn [deserialize\_ignored\_any](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_ignored_any)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type needs to deserialize a value whose type doesn’t matter because it is ignored. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_ignored_any)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1111)

#### fn [is\_human\_readable](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#method.is_human_readable)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Determine whether `Deserialize` implementations should expect to deserialize their human-readable form. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#method.is_human_readable)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1112)

### impl<'de> [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de> for [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Deserializer](trait.Deserializer.html "trait bevy::reflect::erased_serde::Deserializer")<'de> + '\_>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1112)

#### type [Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#associatedtype.Error) = [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")

The error type that can be returned if some error occurs during deserialization.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1112)

#### fn [deserialize\_any](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_any)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Require the `Deserializer` to figure out how to drive the visitor based on what data type is in the input. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_any)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1112)

#### fn [deserialize\_bool](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_bool)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a `bool` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1112)

#### fn [deserialize\_i8](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_i8)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting an `i8` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1112)

#### fn [deserialize\_i16](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_i16)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting an `i16` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1112)

#### fn [deserialize\_i32](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_i32)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting an `i32` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1112)

#### fn [deserialize\_i64](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_i64)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting an `i64` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1112)

#### fn [deserialize\_i128](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#method.deserialize_i128)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting an `i128` value. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#method.deserialize_i128)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1112)

#### fn [deserialize\_u8](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_u8)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a `u8` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1112)

#### fn [deserialize\_u16](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_u16)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a `u16` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1112)

#### fn [deserialize\_u32](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_u32)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a `u32` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1112)

#### fn [deserialize\_u64](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_u64)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a `u64` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1112)

#### fn [deserialize\_u128](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#method.deserialize_u128)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting an `u128` value. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#method.deserialize_u128)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1112)

#### fn [deserialize\_f32](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_f32)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a `f32` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1112)

#### fn [deserialize\_f64](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_f64)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a `f64` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1112)

#### fn [deserialize\_char](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_char)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a `char` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1112)

#### fn [deserialize\_str](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_str)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a string value and does not benefit from taking ownership of buffered data owned by the `Deserializer`. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_str)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1112)

#### fn [deserialize\_string](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_string)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a string value and would benefit from taking ownership of buffered data owned by the `Deserializer`. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_string)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1112)

#### fn [deserialize\_bytes](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_bytes)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a byte array and does not benefit from taking ownership of buffered data owned by the `Deserializer`. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_bytes)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1112)

#### fn [deserialize\_byte\_buf](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_byte_buf)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a byte array and would benefit from taking ownership of buffered data owned by the `Deserializer`. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_byte_buf)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1112)

#### fn [deserialize\_option](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_option)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting an optional value. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_option)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1112)

#### fn [deserialize\_unit](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_unit)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a unit value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1112)

#### fn [deserialize\_unit\_struct](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_unit_struct)<V>( self, name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a unit struct with a particular name.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1112)

#### fn [deserialize\_newtype\_struct](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_newtype_struct)<V>( self, name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a newtype struct with a particular name.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1112)

#### fn [deserialize\_seq](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_seq)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a sequence of values.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1112)

#### fn [deserialize\_tuple](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_tuple)<V>( self, len: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a sequence of values and knows how many values there are without looking at the serialized data.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1112)

#### fn [deserialize\_tuple\_struct](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_tuple_struct)<V>( self, name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), len: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a tuple struct with a particular name and number of fields.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1112)

#### fn [deserialize\_map](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_map)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a map of key-value pairs.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1112)

#### fn [deserialize\_struct](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_struct)<V>( self, name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), fields: &'static \[&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\], visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a struct with a particular name and fields.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1112)

#### fn [deserialize\_identifier](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_identifier)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting the name of a struct field or the discriminant of an enum variant.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1112)

#### fn [deserialize\_enum](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_enum)<V>( self, name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), variants: &'static \[&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\], visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting an enum value with a particular name and possible variants.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1112)

#### fn [deserialize\_ignored\_any](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_ignored_any)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type needs to deserialize a value whose type doesn’t matter because it is ignored. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_ignored_any)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1112)

#### fn [is\_human\_readable](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#method.is_human_readable)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Determine whether `Deserialize` implementations should expect to deserialize their human-readable form. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#method.is_human_readable)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1113)

### impl<'de> [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de> for [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Deserializer](trait.Deserializer.html "trait bevy::reflect::erased_serde::Deserializer")<'de> + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + '\_>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1113)

#### type [Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#associatedtype.Error) = [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")

The error type that can be returned if some error occurs during deserialization.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1113)

#### fn [deserialize\_any](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_any)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Require the `Deserializer` to figure out how to drive the visitor based on what data type is in the input. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_any)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1113)

#### fn [deserialize\_bool](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_bool)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a `bool` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1113)

#### fn [deserialize\_i8](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_i8)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting an `i8` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1113)

#### fn [deserialize\_i16](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_i16)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting an `i16` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1113)

#### fn [deserialize\_i32](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_i32)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting an `i32` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1113)

#### fn [deserialize\_i64](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_i64)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting an `i64` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1113)

#### fn [deserialize\_i128](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#method.deserialize_i128)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting an `i128` value. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#method.deserialize_i128)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1113)

#### fn [deserialize\_u8](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_u8)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a `u8` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1113)

#### fn [deserialize\_u16](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_u16)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a `u16` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1113)

#### fn [deserialize\_u32](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_u32)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a `u32` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1113)

#### fn [deserialize\_u64](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_u64)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a `u64` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1113)

#### fn [deserialize\_u128](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#method.deserialize_u128)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting an `u128` value. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#method.deserialize_u128)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1113)

#### fn [deserialize\_f32](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_f32)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a `f32` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1113)

#### fn [deserialize\_f64](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_f64)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a `f64` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1113)

#### fn [deserialize\_char](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_char)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a `char` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1113)

#### fn [deserialize\_str](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_str)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a string value and does not benefit from taking ownership of buffered data owned by the `Deserializer`. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_str)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1113)

#### fn [deserialize\_string](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_string)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a string value and would benefit from taking ownership of buffered data owned by the `Deserializer`. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_string)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1113)

#### fn [deserialize\_bytes](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_bytes)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a byte array and does not benefit from taking ownership of buffered data owned by the `Deserializer`. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_bytes)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1113)

#### fn [deserialize\_byte\_buf](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_byte_buf)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a byte array and would benefit from taking ownership of buffered data owned by the `Deserializer`. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_byte_buf)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1113)

#### fn [deserialize\_option](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_option)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting an optional value. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_option)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1113)

#### fn [deserialize\_unit](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_unit)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a unit value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1113)

#### fn [deserialize\_unit\_struct](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_unit_struct)<V>( self, name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a unit struct with a particular name.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1113)

#### fn [deserialize\_newtype\_struct](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_newtype_struct)<V>( self, name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a newtype struct with a particular name.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1113)

#### fn [deserialize\_seq](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_seq)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a sequence of values.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1113)

#### fn [deserialize\_tuple](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_tuple)<V>( self, len: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a sequence of values and knows how many values there are without looking at the serialized data.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1113)

#### fn [deserialize\_tuple\_struct](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_tuple_struct)<V>( self, name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), len: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a tuple struct with a particular name and number of fields.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1113)

#### fn [deserialize\_map](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_map)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a map of key-value pairs.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1113)

#### fn [deserialize\_struct](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_struct)<V>( self, name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), fields: &'static \[&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\], visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a struct with a particular name and fields.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1113)

#### fn [deserialize\_identifier](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_identifier)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting the name of a struct field or the discriminant of an enum variant.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1113)

#### fn [deserialize\_enum](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_enum)<V>( self, name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), variants: &'static \[&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\], visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting an enum value with a particular name and possible variants.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1113)

#### fn [deserialize\_ignored\_any](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_ignored_any)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type needs to deserialize a value whose type doesn’t matter because it is ignored. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_ignored_any)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1113)

#### fn [is\_human\_readable](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#method.is_human_readable)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Determine whether `Deserialize` implementations should expect to deserialize their human-readable form. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#method.is_human_readable)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1114)

### impl<'de> [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de> for [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Deserializer](trait.Deserializer.html "trait bevy::reflect::erased_serde::Deserializer")<'de> + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + '\_>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1114)

#### type [Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#associatedtype.Error) = [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")

The error type that can be returned if some error occurs during deserialization.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1114)

#### fn [deserialize\_any](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_any)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Require the `Deserializer` to figure out how to drive the visitor based on what data type is in the input. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_any)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1114)

#### fn [deserialize\_bool](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_bool)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a `bool` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1114)

#### fn [deserialize\_i8](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_i8)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting an `i8` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1114)

#### fn [deserialize\_i16](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_i16)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting an `i16` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1114)

#### fn [deserialize\_i32](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_i32)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting an `i32` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1114)

#### fn [deserialize\_i64](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_i64)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting an `i64` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1114)

#### fn [deserialize\_i128](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#method.deserialize_i128)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting an `i128` value. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#method.deserialize_i128)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1114)

#### fn [deserialize\_u8](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_u8)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a `u8` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1114)

#### fn [deserialize\_u16](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_u16)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a `u16` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1114)

#### fn [deserialize\_u32](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_u32)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a `u32` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1114)

#### fn [deserialize\_u64](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_u64)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a `u64` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1114)

#### fn [deserialize\_u128](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#method.deserialize_u128)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting an `u128` value. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#method.deserialize_u128)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1114)

#### fn [deserialize\_f32](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_f32)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a `f32` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1114)

#### fn [deserialize\_f64](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_f64)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a `f64` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1114)

#### fn [deserialize\_char](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_char)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a `char` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1114)

#### fn [deserialize\_str](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_str)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a string value and does not benefit from taking ownership of buffered data owned by the `Deserializer`. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_str)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1114)

#### fn [deserialize\_string](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_string)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a string value and would benefit from taking ownership of buffered data owned by the `Deserializer`. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_string)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1114)

#### fn [deserialize\_bytes](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_bytes)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a byte array and does not benefit from taking ownership of buffered data owned by the `Deserializer`. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_bytes)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1114)

#### fn [deserialize\_byte\_buf](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_byte_buf)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a byte array and would benefit from taking ownership of buffered data owned by the `Deserializer`. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_byte_buf)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1114)

#### fn [deserialize\_option](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_option)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting an optional value. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_option)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1114)

#### fn [deserialize\_unit](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_unit)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a unit value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1114)

#### fn [deserialize\_unit\_struct](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_unit_struct)<V>( self, name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a unit struct with a particular name.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1114)

#### fn [deserialize\_newtype\_struct](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_newtype_struct)<V>( self, name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a newtype struct with a particular name.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1114)

#### fn [deserialize\_seq](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_seq)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a sequence of values.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1114)

#### fn [deserialize\_tuple](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_tuple)<V>( self, len: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a sequence of values and knows how many values there are without looking at the serialized data.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1114)

#### fn [deserialize\_tuple\_struct](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_tuple_struct)<V>( self, name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), len: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a tuple struct with a particular name and number of fields.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1114)

#### fn [deserialize\_map](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_map)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a map of key-value pairs.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1114)

#### fn [deserialize\_struct](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_struct)<V>( self, name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), fields: &'static \[&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\], visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a struct with a particular name and fields.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1114)

#### fn [deserialize\_identifier](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_identifier)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting the name of a struct field or the discriminant of an enum variant.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1114)

#### fn [deserialize\_enum](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_enum)<V>( self, name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), variants: &'static \[&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\], visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting an enum value with a particular name and possible variants.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1114)

#### fn [deserialize\_ignored\_any](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_ignored_any)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type needs to deserialize a value whose type doesn’t matter because it is ignored. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_ignored_any)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1114)

#### fn [is\_human\_readable](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#method.is_human_readable)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Determine whether `Deserialize` implementations should expect to deserialize their human-readable form. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#method.is_human_readable)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1115)

### impl<'de> [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de> for [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Deserializer](trait.Deserializer.html "trait bevy::reflect::erased_serde::Deserializer")<'de> + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + '\_>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1115)

#### type [Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#associatedtype.Error) = [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")

The error type that can be returned if some error occurs during deserialization.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1115)

#### fn [deserialize\_any](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_any)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Require the `Deserializer` to figure out how to drive the visitor based on what data type is in the input. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_any)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1115)

#### fn [deserialize\_bool](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_bool)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a `bool` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1115)

#### fn [deserialize\_i8](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_i8)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting an `i8` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1115)

#### fn [deserialize\_i16](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_i16)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting an `i16` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1115)

#### fn [deserialize\_i32](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_i32)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting an `i32` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1115)

#### fn [deserialize\_i64](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_i64)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting an `i64` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1115)

#### fn [deserialize\_i128](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#method.deserialize_i128)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting an `i128` value. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#method.deserialize_i128)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1115)

#### fn [deserialize\_u8](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_u8)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a `u8` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1115)

#### fn [deserialize\_u16](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_u16)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a `u16` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1115)

#### fn [deserialize\_u32](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_u32)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a `u32` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1115)

#### fn [deserialize\_u64](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_u64)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a `u64` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1115)

#### fn [deserialize\_u128](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#method.deserialize_u128)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting an `u128` value. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#method.deserialize_u128)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1115)

#### fn [deserialize\_f32](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_f32)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a `f32` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1115)

#### fn [deserialize\_f64](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_f64)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a `f64` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1115)

#### fn [deserialize\_char](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_char)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a `char` value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1115)

#### fn [deserialize\_str](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_str)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a string value and does not benefit from taking ownership of buffered data owned by the `Deserializer`. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_str)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1115)

#### fn [deserialize\_string](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_string)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a string value and would benefit from taking ownership of buffered data owned by the `Deserializer`. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_string)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1115)

#### fn [deserialize\_bytes](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_bytes)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a byte array and does not benefit from taking ownership of buffered data owned by the `Deserializer`. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_bytes)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1115)

#### fn [deserialize\_byte\_buf](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_byte_buf)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a byte array and would benefit from taking ownership of buffered data owned by the `Deserializer`. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_byte_buf)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1115)

#### fn [deserialize\_option](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_option)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting an optional value. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_option)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1115)

#### fn [deserialize\_unit](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_unit)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a unit value.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1115)

#### fn [deserialize\_unit\_struct](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_unit_struct)<V>( self, name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a unit struct with a particular name.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1115)

#### fn [deserialize\_newtype\_struct](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_newtype_struct)<V>( self, name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a newtype struct with a particular name.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1115)

#### fn [deserialize\_seq](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_seq)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a sequence of values.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1115)

#### fn [deserialize\_tuple](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_tuple)<V>( self, len: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a sequence of values and knows how many values there are without looking at the serialized data.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1115)

#### fn [deserialize\_tuple\_struct](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_tuple_struct)<V>( self, name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), len: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a tuple struct with a particular name and number of fields.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1115)

#### fn [deserialize\_map](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_map)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a map of key-value pairs.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1115)

#### fn [deserialize\_struct](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_struct)<V>( self, name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), fields: &'static \[&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\], visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting a struct with a particular name and fields.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1115)

#### fn [deserialize\_identifier](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_identifier)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting the name of a struct field or the discriminant of an enum variant.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1115)

#### fn [deserialize\_enum](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_enum)<V>( self, name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), variants: &'static \[&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\], visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type is expecting an enum value with a particular name and possible variants.

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1115)

#### fn [deserialize\_ignored\_any](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_ignored_any)<V>( self, visitor: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<V as [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>>::[Value](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html#associatedtype.Value "type serde_core::de::Visitor::Value"), [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

where V: [Visitor](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Visitor.html "trait serde_core::de::Visitor")<'de>,

Hint that the `Deserialize` type needs to deserialize a value whose type doesn’t matter because it is ignored. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#tymethod.deserialize_ignored_any)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1115)

#### fn [is\_human\_readable](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#method.is_human_readable)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Determine whether `Deserialize` implementations should expect to deserialize their human-readable form. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#method.is_human_readable)

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1576)

### impl<'de, T> [Deserializer](trait.Deserializer.html "trait bevy::reflect::erased_serde::Deserializer")<'de> for [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

where T: [Deserializer](trait.Deserializer.html "trait bevy::reflect::erased_serde::Deserializer")<'de> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1576)

#### fn [erased\_deserialize\_any](#tymethod.erased_deserialize_any)( &mut self, visitor: &mut dyn Visitor<'de>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Out, [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1576)

#### fn [erased\_deserialize\_bool](#tymethod.erased_deserialize_bool)( &mut self, visitor: &mut dyn Visitor<'de>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Out, [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1576)

#### fn [erased\_deserialize\_i8](#tymethod.erased_deserialize_i8)( &mut self, visitor: &mut dyn Visitor<'de>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Out, [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1576)

#### fn [erased\_deserialize\_i16](#tymethod.erased_deserialize_i16)( &mut self, visitor: &mut dyn Visitor<'de>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Out, [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1576)

#### fn [erased\_deserialize\_i32](#tymethod.erased_deserialize_i32)( &mut self, visitor: &mut dyn Visitor<'de>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Out, [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1576)

#### fn [erased\_deserialize\_i64](#tymethod.erased_deserialize_i64)( &mut self, visitor: &mut dyn Visitor<'de>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Out, [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1576)

#### fn [erased\_deserialize\_i128](#tymethod.erased_deserialize_i128)( &mut self, visitor: &mut dyn Visitor<'de>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Out, [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1576)

#### fn [erased\_deserialize\_u8](#tymethod.erased_deserialize_u8)( &mut self, visitor: &mut dyn Visitor<'de>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Out, [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1576)

#### fn [erased\_deserialize\_u16](#tymethod.erased_deserialize_u16)( &mut self, visitor: &mut dyn Visitor<'de>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Out, [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1576)

#### fn [erased\_deserialize\_u32](#tymethod.erased_deserialize_u32)( &mut self, visitor: &mut dyn Visitor<'de>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Out, [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1576)

#### fn [erased\_deserialize\_u64](#tymethod.erased_deserialize_u64)( &mut self, visitor: &mut dyn Visitor<'de>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Out, [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1576)

#### fn [erased\_deserialize\_u128](#tymethod.erased_deserialize_u128)( &mut self, visitor: &mut dyn Visitor<'de>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Out, [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1576)

#### fn [erased\_deserialize\_f32](#tymethod.erased_deserialize_f32)( &mut self, visitor: &mut dyn Visitor<'de>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Out, [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1576)

#### fn [erased\_deserialize\_f64](#tymethod.erased_deserialize_f64)( &mut self, visitor: &mut dyn Visitor<'de>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Out, [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1576)

#### fn [erased\_deserialize\_char](#tymethod.erased_deserialize_char)( &mut self, visitor: &mut dyn Visitor<'de>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Out, [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1576)

#### fn [erased\_deserialize\_str](#tymethod.erased_deserialize_str)( &mut self, visitor: &mut dyn Visitor<'de>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Out, [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1576)

#### fn [erased\_deserialize\_string](#tymethod.erased_deserialize_string)( &mut self, visitor: &mut dyn Visitor<'de>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Out, [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1576)

#### fn [erased\_deserialize\_bytes](#tymethod.erased_deserialize_bytes)( &mut self, visitor: &mut dyn Visitor<'de>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Out, [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1576)

#### fn [erased\_deserialize\_byte\_buf](#tymethod.erased_deserialize_byte_buf)( &mut self, visitor: &mut dyn Visitor<'de>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Out, [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1576)

#### fn [erased\_deserialize\_option](#tymethod.erased_deserialize_option)( &mut self, visitor: &mut dyn Visitor<'de>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Out, [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1576)

#### fn [erased\_deserialize\_unit](#tymethod.erased_deserialize_unit)( &mut self, visitor: &mut dyn Visitor<'de>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Out, [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1576)

#### fn [erased\_deserialize\_unit\_struct](#tymethod.erased_deserialize_unit_struct)( &mut self, name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), visitor: &mut dyn Visitor<'de>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Out, [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1576)

#### fn [erased\_deserialize\_newtype\_struct](#tymethod.erased_deserialize_newtype_struct)( &mut self, name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), visitor: &mut dyn Visitor<'de>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Out, [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1576)

#### fn [erased\_deserialize\_seq](#tymethod.erased_deserialize_seq)( &mut self, visitor: &mut dyn Visitor<'de>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Out, [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1576)

#### fn [erased\_deserialize\_tuple](#tymethod.erased_deserialize_tuple)( &mut self, len: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), visitor: &mut dyn Visitor<'de>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Out, [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1576)

#### fn [erased\_deserialize\_tuple\_struct](#tymethod.erased_deserialize_tuple_struct)( &mut self, name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), len: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), visitor: &mut dyn Visitor<'de>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Out, [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1576)

#### fn [erased\_deserialize\_map](#tymethod.erased_deserialize_map)( &mut self, visitor: &mut dyn Visitor<'de>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Out, [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1576)

#### fn [erased\_deserialize\_struct](#tymethod.erased_deserialize_struct)( &mut self, name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), fields: &'static \[&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\], visitor: &mut dyn Visitor<'de>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Out, [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1576)

#### fn [erased\_deserialize\_identifier](#tymethod.erased_deserialize_identifier)( &mut self, visitor: &mut dyn Visitor<'de>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Out, [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1576)

#### fn [erased\_deserialize\_enum](#tymethod.erased_deserialize_enum)( &mut self, name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), variants: &'static \[&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\], visitor: &mut dyn Visitor<'de>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Out, [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1576)

#### fn [erased\_deserialize\_ignored\_any](#tymethod.erased_deserialize_ignored_any)( &mut self, visitor: &mut dyn Visitor<'de>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Out, [Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1576)

#### fn [erased\_is\_human\_readable](#tymethod.erased_is_human_readable)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

## Implementors

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#1577)

### impl<'de, T> [Deserializer](trait.Deserializer.html "trait bevy::reflect::erased_serde::Deserializer")<'de> for [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<T>

where T: [Deserializer](trait.Deserializer.html "trait bevy::reflect::erased_serde::Deserializer")<'de> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),