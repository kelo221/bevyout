[bevy](../../index.html)::[reflect](../index.html)::[erased\_serde](index.html)

# Trait Serializer 

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#112)

```rust
pub trait Serializer: Sealed {
    // Required methods
    fn erased_serialize_bool(&mut self, v: bool);
    fn erased_serialize_i8(&mut self, v: i8);
    fn erased_serialize_i16(&mut self, v: i16);
    fn erased_serialize_i32(&mut self, v: i32);
    fn erased_serialize_i64(&mut self, v: i64);
    fn erased_serialize_i128(&mut self, v: i128);
    fn erased_serialize_u8(&mut self, v: u8);
    fn erased_serialize_u16(&mut self, v: u16);
    fn erased_serialize_u32(&mut self, v: u32);
    fn erased_serialize_u64(&mut self, v: u64);
    fn erased_serialize_u128(&mut self, v: u128);
    fn erased_serialize_f32(&mut self, v: f32);
    fn erased_serialize_f64(&mut self, v: f64);
    fn erased_serialize_char(&mut self, v: char);
    fn erased_serialize_str(&mut self, v: &str);
    fn erased_serialize_bytes(&mut self, v: &[u8]);
    fn erased_serialize_none(&mut self);
    fn erased_serialize_some(&mut self, value: &dyn Serialize);
    fn erased_serialize_unit(&mut self);
    fn erased_serialize_unit_struct(&mut self, name: &'static str);
    fn erased_serialize_unit_variant(
        &mut self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    );
    fn erased_serialize_newtype_struct(
        &mut self,
        name: &'static str,
        value: &dyn Serialize,
    );
    fn erased_serialize_newtype_variant(
        &mut self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &dyn Serialize,
    );
    fn erased_serialize_seq(
        &mut self,
        len: Option<usize>,
    ) -> Result<&mut dyn SerializeSeq, ErrorImpl>;
    fn erased_serialize_tuple(
        &mut self,
        len: usize,
    ) -> Result<&mut dyn SerializeTuple, ErrorImpl>;
    fn erased_serialize_tuple_struct(
        &mut self,
        name: &'static str,
        len: usize,
    ) -> Result<&mut dyn SerializeTupleStruct, ErrorImpl>;
    fn erased_serialize_tuple_variant(
        &mut self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<&mut dyn SerializeTupleVariant, ErrorImpl>;
    fn erased_serialize_map(
        &mut self,
        len: Option<usize>,
    ) -> Result<&mut dyn SerializeMap, ErrorImpl>;
    fn erased_serialize_struct(
        &mut self,
        name: &'static str,
        len: usize,
    ) -> Result<&mut dyn SerializeStruct, ErrorImpl>;
    fn erased_serialize_struct_variant(
        &mut self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<&mut dyn SerializeStructVariant, ErrorImpl>;
    fn erased_is_human_readable(&self) -> bool;
}
```

An object-safe equivalent of Serde’s `Serializer` trait.

Any implementation of Serde’s `Serializer` can be converted to a `&dyn erased_serde::Serializer` or `Box<dyn erased_serde::Serializer>` trait object using `erased_serde::Serializer::erase`.

```rust
use erased_serde::{Serialize, Serializer};
use std::collections::BTreeMap as Map;
use std::io;

fn main() {
    // Construct some serializers.
    let json = &mut serde_json::Serializer::new(io::stdout());
    let cbor = &mut serde_cbor::Serializer::new(serde_cbor::ser::IoWrite::new(io::stdout()));

    // The values in this map are boxed trait objects. Ordinarily this would not
    // be possible with serde::Serializer because of object safety, but type
    // erasure makes it possible with erased_serde::Serializer.
    let mut formats: Map<&str, Box<dyn Serializer>> = Map::new();
    formats.insert("json", Box::new(<dyn Serializer>::erase(json)));
    formats.insert("cbor", Box::new(<dyn Serializer>::erase(cbor)));

    // These are boxed trait objects as well. Same thing here - type erasure
    // makes this possible.
    let mut values: Map<&str, Box<dyn Serialize>> = Map::new();
    values.insert("vec", Box::new(vec!["a", "b"]));
    values.insert("int", Box::new(65536));

    // Pick a Serializer out of the formats map.
    let format = formats.get_mut("json").unwrap();

    // Pick a Serialize out of the values map.
    let value = values.get("vec").unwrap();

    // This line prints `["a","b"]` to stdout.
    value.erased_serialize(format).unwrap();
}
```

This trait is sealed and can only be implemented via a `serde::Serializer` impl.

## Required Methods

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#113)

#### fn [erased\_serialize\_bool](#tymethod.erased_serialize_bool)(&mut self, v: [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html))

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#114)

#### fn [erased\_serialize\_i8](#tymethod.erased_serialize_i8)(&mut self, v: [i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html))

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#115)

#### fn [erased\_serialize\_i16](#tymethod.erased_serialize_i16)(&mut self, v: [i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html))

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#116)

#### fn [erased\_serialize\_i32](#tymethod.erased_serialize_i32)(&mut self, v: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html))

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#117)

#### fn [erased\_serialize\_i64](#tymethod.erased_serialize_i64)(&mut self, v: [i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html))

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#118)

#### fn [erased\_serialize\_i128](#tymethod.erased_serialize_i128)(&mut self, v: [i128](https://doc.rust-lang.org/nightly/std/primitive.i128.html))

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#119)

#### fn [erased\_serialize\_u8](#tymethod.erased_serialize_u8)(&mut self, v: [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html))

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#120)

#### fn [erased\_serialize\_u16](#tymethod.erased_serialize_u16)(&mut self, v: [u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html))

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#121)

#### fn [erased\_serialize\_u32](#tymethod.erased_serialize_u32)(&mut self, v: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#122)

#### fn [erased\_serialize\_u64](#tymethod.erased_serialize_u64)(&mut self, v: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html))

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#123)

#### fn [erased\_serialize\_u128](#tymethod.erased_serialize_u128)(&mut self, v: [u128](https://doc.rust-lang.org/nightly/std/primitive.u128.html))

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#124)

#### fn [erased\_serialize\_f32](#tymethod.erased_serialize_f32)(&mut self, v: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#125)

#### fn [erased\_serialize\_f64](#tymethod.erased_serialize_f64)(&mut self, v: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html))

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#126)

#### fn [erased\_serialize\_char](#tymethod.erased_serialize_char)(&mut self, v: [char](https://doc.rust-lang.org/nightly/std/primitive.char.html))

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#127)

#### fn [erased\_serialize\_str](#tymethod.erased_serialize_str)(&mut self, v: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html))

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#128)

#### fn [erased\_serialize\_bytes](#tymethod.erased_serialize_bytes)(&mut self, v: &\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\])

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#129)

#### fn [erased\_serialize\_none](#tymethod.erased_serialize_none)(&mut self)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#130)

#### fn [erased\_serialize\_some](#tymethod.erased_serialize_some)(&mut self, value: &dyn [Serialize](trait.Serialize.html "trait bevy::reflect::erased_serde::Serialize"))

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#131)

#### fn [erased\_serialize\_unit](#tymethod.erased_serialize_unit)(&mut self)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#132)

#### fn [erased\_serialize\_unit\_struct](#tymethod.erased_serialize_unit_struct)(&mut self, name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html))

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#133-138)

#### fn [erased\_serialize\_unit\_variant](#tymethod.erased_serialize_unit_variant)( &mut self, name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), variant\_index: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), variant: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), )

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#139)

#### fn [erased\_serialize\_newtype\_struct](#tymethod.erased_serialize_newtype_struct)( &mut self, name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), value: &dyn [Serialize](trait.Serialize.html "trait bevy::reflect::erased_serde::Serialize"), )

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#140-146)

#### fn [erased\_serialize\_newtype\_variant](#tymethod.erased_serialize_newtype_variant)( &mut self, name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), variant\_index: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), variant: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), value: &dyn [Serialize](trait.Serialize.html "trait bevy::reflect::erased_serde::Serialize"), )

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#147-150)

#### fn [erased\_serialize\_seq](#tymethod.erased_serialize_seq)( &mut self, len: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&mut dyn SerializeSeq, ErrorImpl>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#151)

#### fn [erased\_serialize\_tuple](#tymethod.erased_serialize_tuple)( &mut self, len: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&mut dyn SerializeTuple, ErrorImpl>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#152-156)

#### fn [erased\_serialize\_tuple\_struct](#tymethod.erased_serialize_tuple_struct)( &mut self, name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), len: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&mut dyn SerializeTupleStruct, ErrorImpl>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#157-163)

#### fn [erased\_serialize\_tuple\_variant](#tymethod.erased_serialize_tuple_variant)( &mut self, name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), variant\_index: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), variant: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), len: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&mut dyn SerializeTupleVariant, ErrorImpl>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#164-167)

#### fn [erased\_serialize\_map](#tymethod.erased_serialize_map)( &mut self, len: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&mut dyn SerializeMap, ErrorImpl>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#168-172)

#### fn [erased\_serialize\_struct](#tymethod.erased_serialize_struct)( &mut self, name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), len: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&mut dyn SerializeStruct, ErrorImpl>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#173-179)

#### fn [erased\_serialize\_struct\_variant](#tymethod.erased_serialize_struct_variant)( &mut self, name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), variant\_index: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), variant: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), len: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&mut dyn SerializeStructVariant, ErrorImpl>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#180)

#### fn [erased\_is\_human\_readable](#tymethod.erased_is_human_readable)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

## Implementations

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#185)

### impl dyn [Serializer](trait.Serializer.html "trait bevy::reflect::erased_serde::Serializer")

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#186-228)

#### pub fn [erase](#method.erase)<S>(serializer: S) -> Serializer<S>

where S: [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer"),

Available on **non-`docsrs`** only.

Convert any Serde `Serializer` to a trait object.

```rust
use erased_serde::{Serialize, Serializer};
use std::collections::BTreeMap as Map;
use std::io;

fn main() {
    // Construct some serializers.
    let json = &mut serde_json::Serializer::new(io::stdout());
    let cbor = &mut serde_cbor::Serializer::new(serde_cbor::ser::IoWrite::new(io::stdout()));

    // The values in this map are boxed trait objects. Ordinarily this would not
    // be possible with serde::Serializer because of object safety, but type
    // erasure makes it possible with erased_serde::Serializer.
    let mut formats: Map<&str, Box<dyn Serializer>> = Map::new();
    formats.insert("json", Box::new(<dyn Serializer>::erase(json)));
    formats.insert("cbor", Box::new(<dyn Serializer>::erase(cbor)));

    // These are boxed trait objects as well. Same thing here - type erasure
    // makes this possible.
    let mut values: Map<&str, Box<dyn Serialize>> = Map::new();
    values.insert("vec", Box::new(vec!["a", "b"]));
    values.insert("int", Box::new(65536));

    // Pick a Serializer out of the formats map.
    let format = formats.get_mut("json").unwrap();

    // Pick a Serialize out of the values map.
    let value = values.get("vec").unwrap();

    // This line prints `["a","b"]` to stdout.
    value.erased_serialize(format).unwrap();
}
```

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#1474)

### impl<T> [Serializer](trait.Serializer.html "trait bevy::reflect::erased_serde::Serializer") for [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

where T: [Serializer](trait.Serializer.html "trait bevy::reflect::erased_serde::Serializer") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#1474)

#### fn [erased\_serialize\_bool](#tymethod.erased_serialize_bool)(&mut self, v: [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html))

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#1474)

#### fn [erased\_serialize\_i8](#tymethod.erased_serialize_i8)(&mut self, v: [i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html))

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#1474)

#### fn [erased\_serialize\_i16](#tymethod.erased_serialize_i16)(&mut self, v: [i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html))

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#1474)

#### fn [erased\_serialize\_i32](#tymethod.erased_serialize_i32)(&mut self, v: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html))

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#1474)

#### fn [erased\_serialize\_i64](#tymethod.erased_serialize_i64)(&mut self, v: [i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html))

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#1474)

#### fn [erased\_serialize\_i128](#tymethod.erased_serialize_i128)(&mut self, v: [i128](https://doc.rust-lang.org/nightly/std/primitive.i128.html))

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#1474)

#### fn [erased\_serialize\_u8](#tymethod.erased_serialize_u8)(&mut self, v: [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html))

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#1474)

#### fn [erased\_serialize\_u16](#tymethod.erased_serialize_u16)(&mut self, v: [u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html))

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#1474)

#### fn [erased\_serialize\_u32](#tymethod.erased_serialize_u32)(&mut self, v: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#1474)

#### fn [erased\_serialize\_u64](#tymethod.erased_serialize_u64)(&mut self, v: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html))

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#1474)

#### fn [erased\_serialize\_u128](#tymethod.erased_serialize_u128)(&mut self, v: [u128](https://doc.rust-lang.org/nightly/std/primitive.u128.html))

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#1474)

#### fn [erased\_serialize\_f32](#tymethod.erased_serialize_f32)(&mut self, v: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#1474)

#### fn [erased\_serialize\_f64](#tymethod.erased_serialize_f64)(&mut self, v: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html))

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#1474)

#### fn [erased\_serialize\_char](#tymethod.erased_serialize_char)(&mut self, v: [char](https://doc.rust-lang.org/nightly/std/primitive.char.html))

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#1474)

#### fn [erased\_serialize\_str](#tymethod.erased_serialize_str)(&mut self, v: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html))

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#1474)

#### fn [erased\_serialize\_bytes](#tymethod.erased_serialize_bytes)(&mut self, v: &\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\])

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#1474)

#### fn [erased\_serialize\_none](#tymethod.erased_serialize_none)(&mut self)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#1474)

#### fn [erased\_serialize\_some](#tymethod.erased_serialize_some)(&mut self, value: &dyn [Serialize](trait.Serialize.html "trait bevy::reflect::erased_serde::Serialize"))

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#1474)

#### fn [erased\_serialize\_unit](#tymethod.erased_serialize_unit)(&mut self)

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#1474)

#### fn [erased\_serialize\_unit\_struct](#tymethod.erased_serialize_unit_struct)(&mut self, name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html))

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#1474)

#### fn [erased\_serialize\_unit\_variant](#tymethod.erased_serialize_unit_variant)( &mut self, name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), variant\_index: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), variant: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), )

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#1474)

#### fn [erased\_serialize\_newtype\_struct](#tymethod.erased_serialize_newtype_struct)( &mut self, name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), value: &dyn [Serialize](trait.Serialize.html "trait bevy::reflect::erased_serde::Serialize"), )

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#1474)

#### fn [erased\_serialize\_newtype\_variant](#tymethod.erased_serialize_newtype_variant)( &mut self, name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), variant\_index: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), variant: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), value: &dyn [Serialize](trait.Serialize.html "trait bevy::reflect::erased_serde::Serialize"), )

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#1474)

#### fn [erased\_serialize\_seq](#tymethod.erased_serialize_seq)( &mut self, len: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&mut dyn SerializeSeq, ErrorImpl>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#1474)

#### fn [erased\_serialize\_tuple](#tymethod.erased_serialize_tuple)( &mut self, len: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&mut dyn SerializeTuple, ErrorImpl>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#1474)

#### fn [erased\_serialize\_tuple\_struct](#tymethod.erased_serialize_tuple_struct)( &mut self, name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), len: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&mut dyn SerializeTupleStruct, ErrorImpl>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#1474)

#### fn [erased\_serialize\_tuple\_variant](#tymethod.erased_serialize_tuple_variant)( &mut self, name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), variant\_index: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), variant: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), len: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&mut dyn SerializeTupleVariant, ErrorImpl>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#1474)

#### fn [erased\_serialize\_map](#tymethod.erased_serialize_map)( &mut self, len: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&mut dyn SerializeMap, ErrorImpl>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#1474)

#### fn [erased\_serialize\_struct](#tymethod.erased_serialize_struct)( &mut self, name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), len: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&mut dyn SerializeStruct, ErrorImpl>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#1474)

#### fn [erased\_serialize\_struct\_variant](#tymethod.erased_serialize_struct_variant)( &mut self, name: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), variant\_index: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), variant: &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), len: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&mut dyn SerializeStructVariant, ErrorImpl>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#1474)

#### fn [erased\_is\_human\_readable](#tymethod.erased_is_human_readable)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

## Implementors

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#1475)

### impl<T> [Serializer](trait.Serializer.html "trait bevy::reflect::erased_serde::Serializer") for [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<T>

where T: [Serializer](trait.Serializer.html "trait bevy::reflect::erased_serde::Serializer") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),