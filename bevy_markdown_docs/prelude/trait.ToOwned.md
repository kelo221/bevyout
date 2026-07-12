[bevy](../index.html)::[prelude](index.html)

# Trait ToOwned 

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#27)

```rust
pub trait ToOwned {
    type Owned: Borrow<Self>;

    // Required method
    fn to_owned(&self) -> Self::Owned;

    // Provided method
    fn clone_into(&self, target: &mut Self::Owned) { ... }
}
```

A generalization of `Clone` to borrowed data.

Some types make it possible to go from borrowed to owned, usually by implementing the `Clone` trait. But `Clone` works only for going from `&T` to `T`. The `ToOwned` trait generalizes `Clone` to construct owned data from any borrow of a given type.

## Required Associated Types

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#30)

#### type [Owned](#associatedtype.Owned): [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<Self>

The resulting type after obtaining ownership.

## Required Methods

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#48)

#### fn [to\_owned](#tymethod.to_owned)(&self) -> Self::[Owned](trait.ToOwned.html#associatedtype.Owned "type bevy::prelude::ToOwned::Owned")

Creates owned data from borrowed data, usually by cloning.

##### Examples

Basic usage:

```rust
let s: &str = "a";
let ss: String = s.to_owned();

let v: &[i32] = &[1, 2];
let vv: Vec<i32> = v.to_owned();
```

## Provided Methods

1.63.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#66)

#### fn [clone\_into](#method.clone_into)(&self, target: &mut Self::[Owned](trait.ToOwned.html#associatedtype.Owned "type bevy::prelude::ToOwned::Owned"))

Uses borrowed data to replace owned data, usually by cloning.

This is borrow-generalized version of [`Clone::clone_from`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from "method core::clone::Clone::clone_from").

##### Examples

Basic usage:

```rust
let mut s: String = String::new();
"hello".clone_into(&mut s);

let mut v: Vec<i32> = Vec::new();
[1, 2][..].clone_into(&mut v);
```

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/winnow/0.7.15/x86_64-unknown-linux-gnu/src/winnow/stream/bstr.rs.html#358)

### impl [ToOwned](trait.ToOwned.html "trait bevy::prelude::ToOwned") for [BStr](https://docs.rs/winnow/0.7.15/x86_64-unknown-linux-gnu/winnow/stream/bstr/struct.BStr.html "struct winnow::stream::bstr::BStr")

Available on **crate feature `alloc`** only.

[Source](https://docs.rs/winnow/0.7.15/x86_64-unknown-linux-gnu/src/winnow/stream/bstr.rs.html#359)

#### type [Owned](#associatedtype.Owned) = [Vec](struct.Vec.html "struct bevy::prelude::Vec")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\>

[Source](https://doc.rust-lang.org/nightly/src/alloc/bstr.rs.html#557)

### impl [ToOwned](trait.ToOwned.html "trait bevy::prelude::ToOwned") for [ByteStr](https://doc.rust-lang.org/nightly/core/bstr/struct.ByteStr.html "struct core::bstr::ByteStr")

[Source](https://doc.rust-lang.org/nightly/src/alloc/bstr.rs.html#558)

#### type [Owned](#associatedtype.Owned) = [ByteString](https://doc.rust-lang.org/nightly/alloc/bstr/struct.ByteString.html "struct alloc::bstr::ByteString")

[Source](https://docs.rs/winnow/0.7.15/x86_64-unknown-linux-gnu/src/winnow/stream/bytes.rs.html#373)

### impl [ToOwned](trait.ToOwned.html "trait bevy::prelude::ToOwned") for [Bytes](https://docs.rs/winnow/0.7.15/x86_64-unknown-linux-gnu/winnow/stream/bytes/struct.Bytes.html "struct winnow::stream::bytes::Bytes")

Available on **crate feature `alloc`** only.

[Source](https://docs.rs/winnow/0.7.15/x86_64-unknown-linux-gnu/src/winnow/stream/bytes.rs.html#374)

#### type [Owned](#associatedtype.Owned) = [Vec](struct.Vec.html "struct bevy::prelude::Vec")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\>

1.3.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/ffi/c_str.rs.html#1072)

### impl [ToOwned](trait.ToOwned.html "trait bevy::prelude::ToOwned") for [CStr](https://doc.rust-lang.org/nightly/core/ffi/c_str/struct.CStr.html "struct core::ffi::c_str::CStr")

[Source](https://doc.rust-lang.org/nightly/src/alloc/ffi/c_str.rs.html#1073)

#### type [Owned](#associatedtype.Owned) = [CString](https://doc.rust-lang.org/nightly/alloc/ffi/c_str/struct.CString.html "struct alloc::ffi::c_str::CString")

[Source](https://docs.rs/icu_provider/2.1.1/x86_64-unknown-linux-gnu/src/icu_provider/request.rs.html#354)

### impl [ToOwned](trait.ToOwned.html "trait bevy::prelude::ToOwned") for [DataMarkerAttributes](https://docs.rs/icu_provider/2.1.1/x86_64-unknown-linux-gnu/icu_provider/request/struct.DataMarkerAttributes.html "struct icu_provider::request::DataMarkerAttributes")

Available on **crate feature `alloc`** only.

✨ _Enabled with the `alloc` Cargo feature._

[Source](https://docs.rs/icu_provider/2.1.1/x86_64-unknown-linux-gnu/src/icu_provider/request.rs.html#355)

#### type [Owned](#associatedtype.Owned) = [Box](struct.Box.html "struct bevy::prelude::Box")<[DataMarkerAttributes](https://docs.rs/icu_provider/2.1.1/x86_64-unknown-linux-gnu/icu_provider/request/struct.DataMarkerAttributes.html "struct icu_provider::request::DataMarkerAttributes")\>

[Source](https://docs.rs/icu_locale/2.1.1/x86_64-unknown-linux-gnu/src/icu_locale/provider.rs.html#194)

### impl [ToOwned](trait.ToOwned.html "trait bevy::prelude::ToOwned") for [LanguageStrStrPairVarULE](https://docs.rs/icu_locale/2.1.1/x86_64-unknown-linux-gnu/icu_locale/provider/struct.LanguageStrStrPairVarULE.html "struct icu_locale::provider::LanguageStrStrPairVarULE")

[Source](https://docs.rs/icu_locale/2.1.1/x86_64-unknown-linux-gnu/src/icu_locale/provider.rs.html#194)

#### type [Owned](#associatedtype.Owned) = [Box](struct.Box.html "struct bevy::prelude::Box")<[LanguageStrStrPairVarULE](https://docs.rs/icu_locale/2.1.1/x86_64-unknown-linux-gnu/icu_locale/provider/struct.LanguageStrStrPairVarULE.html "struct icu_locale::provider::LanguageStrStrPairVarULE")\>

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/std/ffi/os_str.rs.html#1739)

### impl [ToOwned](trait.ToOwned.html "trait bevy::prelude::ToOwned") for [OsStr](https://doc.rust-lang.org/nightly/std/ffi/os_str/struct.OsStr.html "struct std::ffi::os_str::OsStr")

[Source](https://doc.rust-lang.org/nightly/src/std/ffi/os_str.rs.html#1740)

#### type [Owned](#associatedtype.Owned) = [OsString](https://doc.rust-lang.org/nightly/std/ffi/os_str/struct.OsString.html "struct std::ffi::os_str::OsString")

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/std/path.rs.html#2232)

### impl [ToOwned](trait.ToOwned.html "trait bevy::prelude::ToOwned") for [Path](https://doc.rust-lang.org/nightly/std/path/struct.Path.html "struct std::path::Path")

[Source](https://doc.rust-lang.org/nightly/src/std/path.rs.html#2233)

#### type [Owned](#associatedtype.Owned) = [PathBuf](https://doc.rust-lang.org/nightly/std/path/struct.PathBuf.html "struct std::path::PathBuf")

[Source](https://docs.rs/serde_json/1.0.150/x86_64-unknown-linux-gnu/src/serde_json/raw.rs.html#141)

### impl [ToOwned](trait.ToOwned.html "trait bevy::prelude::ToOwned") for serde\_json::raw::[RawValue](https://docs.rs/serde_json/1.0.150/x86_64-unknown-linux-gnu/serde_json/raw/struct.RawValue.html "struct serde_json::raw::RawValue")

[Source](https://docs.rs/serde_json/1.0.150/x86_64-unknown-linux-gnu/src/serde_json/raw.rs.html#142)

#### type [Owned](#associatedtype.Owned) = [Box](struct.Box.html "struct bevy::prelude::Box")<[RawValue](https://docs.rs/serde_json/1.0.150/x86_64-unknown-linux-gnu/serde_json/raw/struct.RawValue.html "struct serde_json::raw::RawValue")\>

[Source](https://docs.rs/ron/0.12.1/x86_64-unknown-linux-gnu/src/ron/value/raw.rs.html#68)

### impl [ToOwned](trait.ToOwned.html "trait bevy::prelude::ToOwned") for ron::value::raw::[RawValue](https://docs.rs/ron/0.12.1/x86_64-unknown-linux-gnu/ron/value/raw/struct.RawValue.html "struct ron::value::raw::RawValue")

[Source](https://docs.rs/ron/0.12.1/x86_64-unknown-linux-gnu/src/ron/value/raw.rs.html#69)

#### type [Owned](#associatedtype.Owned) = [Box](struct.Box.html "struct bevy::prelude::Box")<[RawValue](https://docs.rs/ron/0.12.1/x86_64-unknown-linux-gnu/ron/value/raw/struct.RawValue.html "struct ron::value::raw::RawValue")\>

[Source](https://docs.rs/icu_locale/2.1.1/x86_64-unknown-linux-gnu/src/icu_locale/provider.rs.html#168)

### impl [ToOwned](trait.ToOwned.html "trait bevy::prelude::ToOwned") for [StrStrPairVarULE](https://docs.rs/icu_locale/2.1.1/x86_64-unknown-linux-gnu/icu_locale/provider/struct.StrStrPairVarULE.html "struct icu_locale::provider::StrStrPairVarULE")

[Source](https://docs.rs/icu_locale/2.1.1/x86_64-unknown-linux-gnu/src/icu_locale/provider.rs.html#168)

#### type [Owned](#associatedtype.Owned) = [Box](struct.Box.html "struct bevy::prelude::Box")<[StrStrPairVarULE](https://docs.rs/icu_locale/2.1.1/x86_64-unknown-linux-gnu/icu_locale/provider/struct.StrStrPairVarULE.html "struct icu_locale::provider::StrStrPairVarULE")\>

[Source](https://docs.rs/zerotrie/0.2.4/x86_64-unknown-linux-gnu/src/zerotrie/zerotrie.rs.html#716-722)

### impl [ToOwned](trait.ToOwned.html "trait bevy::prelude::ToOwned") for [ZeroAsciiIgnoreCaseTrie](https://docs.rs/zerotrie/0.2.4/x86_64-unknown-linux-gnu/zerotrie/zerotrie/struct.ZeroAsciiIgnoreCaseTrie.html "struct zerotrie::zerotrie::ZeroAsciiIgnoreCaseTrie")<\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\]>

Available on **crate feature `alloc`** only.

[Source](https://docs.rs/zerotrie/0.2.4/x86_64-unknown-linux-gnu/src/zerotrie/zerotrie.rs.html#716-722)

#### type [Owned](#associatedtype.Owned) = [ZeroAsciiIgnoreCaseTrie](https://docs.rs/zerotrie/0.2.4/x86_64-unknown-linux-gnu/zerotrie/zerotrie/struct.ZeroAsciiIgnoreCaseTrie.html "struct zerotrie::zerotrie::ZeroAsciiIgnoreCaseTrie")<[Box](struct.Box.html "struct bevy::prelude::Box")<\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\]>>

[Source](https://docs.rs/zerotrie/0.2.4/x86_64-unknown-linux-gnu/src/zerotrie/zerotrie.rs.html#730-736)

### impl [ToOwned](trait.ToOwned.html "trait bevy::prelude::ToOwned") for [ZeroTrieExtendedCapacity](https://docs.rs/zerotrie/0.2.4/x86_64-unknown-linux-gnu/zerotrie/zerotrie/struct.ZeroTrieExtendedCapacity.html "struct zerotrie::zerotrie::ZeroTrieExtendedCapacity")<\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\]>

Available on **crate feature `alloc`** only.

[Source](https://docs.rs/zerotrie/0.2.4/x86_64-unknown-linux-gnu/src/zerotrie/zerotrie.rs.html#730-736)

#### type [Owned](#associatedtype.Owned) = [ZeroTrieExtendedCapacity](https://docs.rs/zerotrie/0.2.4/x86_64-unknown-linux-gnu/zerotrie/zerotrie/struct.ZeroTrieExtendedCapacity.html "struct zerotrie::zerotrie::ZeroTrieExtendedCapacity")<[Box](struct.Box.html "struct bevy::prelude::Box")<\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\]>>

[Source](https://docs.rs/zerotrie/0.2.4/x86_64-unknown-linux-gnu/src/zerotrie/zerotrie.rs.html#723-729)

### impl [ToOwned](trait.ToOwned.html "trait bevy::prelude::ToOwned") for [ZeroTriePerfectHash](https://docs.rs/zerotrie/0.2.4/x86_64-unknown-linux-gnu/zerotrie/zerotrie/struct.ZeroTriePerfectHash.html "struct zerotrie::zerotrie::ZeroTriePerfectHash")<\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\]>

Available on **crate feature `alloc`** only.

[Source](https://docs.rs/zerotrie/0.2.4/x86_64-unknown-linux-gnu/src/zerotrie/zerotrie.rs.html#723-729)

#### type [Owned](#associatedtype.Owned) = [ZeroTriePerfectHash](https://docs.rs/zerotrie/0.2.4/x86_64-unknown-linux-gnu/zerotrie/zerotrie/struct.ZeroTriePerfectHash.html "struct zerotrie::zerotrie::ZeroTriePerfectHash")<[Box](struct.Box.html "struct bevy::prelude::Box")<\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\]>>

[Source](https://docs.rs/zerotrie/0.2.4/x86_64-unknown-linux-gnu/src/zerotrie/zerotrie.rs.html#709-715)

### impl [ToOwned](trait.ToOwned.html "trait bevy::prelude::ToOwned") for [ZeroTrieSimpleAscii](https://docs.rs/zerotrie/0.2.4/x86_64-unknown-linux-gnu/zerotrie/zerotrie/struct.ZeroTrieSimpleAscii.html "struct zerotrie::zerotrie::ZeroTrieSimpleAscii")<\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\]>

Available on **crate feature `alloc`** only.

[Source](https://docs.rs/zerotrie/0.2.4/x86_64-unknown-linux-gnu/src/zerotrie/zerotrie.rs.html#709-715)

#### type [Owned](#associatedtype.Owned) = [ZeroTrieSimpleAscii](https://docs.rs/zerotrie/0.2.4/x86_64-unknown-linux-gnu/zerotrie/zerotrie/struct.ZeroTrieSimpleAscii.html "struct zerotrie::zerotrie::ZeroTrieSimpleAscii")<[Box](struct.Box.html "struct bevy::prelude::Box")<\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\]>>

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/str.rs.html#246)

### impl [ToOwned](trait.ToOwned.html "trait bevy::prelude::ToOwned") for [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Available on **non-`no_global_oom_handling`** only.

[Source](https://doc.rust-lang.org/nightly/src/alloc/str.rs.html#247)

#### type [Owned](#associatedtype.Owned) = [String](struct.String.html "struct bevy::prelude::String")

[Source](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/src/zerovec/ule/tuplevar.rs.html#243)

### impl<A, B, C, D, E, F, Format> [ToOwned](trait.ToOwned.html "trait bevy::prelude::ToOwned") for [Tuple6VarULE](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/zerovec/ule/tuplevar/struct.Tuple6VarULE.html "struct zerovec::ule::tuplevar::Tuple6VarULE")<A, B, C, D, E, F, Format>

where A: [VarULE](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/zerovec/ule/trait.VarULE.html "trait zerovec::ule::VarULE") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), B: [VarULE](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/zerovec/ule/trait.VarULE.html "trait zerovec::ule::VarULE") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), C: [VarULE](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/zerovec/ule/trait.VarULE.html "trait zerovec::ule::VarULE") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), D: [VarULE](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/zerovec/ule/trait.VarULE.html "trait zerovec::ule::VarULE") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), E: [VarULE](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/zerovec/ule/trait.VarULE.html "trait zerovec::ule::VarULE") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [VarULE](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/zerovec/ule/trait.VarULE.html "trait zerovec::ule::VarULE") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Format: [VarZeroVecFormat](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/zerovec/varzerovec/components/trait.VarZeroVecFormat.html "trait zerovec::varzerovec::components::VarZeroVecFormat"),

Available on **crate feature `alloc`** only.

[Source](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/src/zerovec/ule/tuplevar.rs.html#243)

#### type [Owned](#associatedtype.Owned) = [Box](struct.Box.html "struct bevy::prelude::Box")<[Tuple6VarULE](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/zerovec/ule/tuplevar/struct.Tuple6VarULE.html "struct zerovec::ule::tuplevar::Tuple6VarULE")<A, B, C, D, E, F, Format>>

[Source](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/src/zerovec/ule/tuplevar.rs.html#242)

### impl<A, B, C, D, E, Format> [ToOwned](trait.ToOwned.html "trait bevy::prelude::ToOwned") for [Tuple5VarULE](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/zerovec/ule/tuplevar/struct.Tuple5VarULE.html "struct zerovec::ule::tuplevar::Tuple5VarULE")<A, B, C, D, E, Format>

where A: [VarULE](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/zerovec/ule/trait.VarULE.html "trait zerovec::ule::VarULE") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), B: [VarULE](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/zerovec/ule/trait.VarULE.html "trait zerovec::ule::VarULE") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), C: [VarULE](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/zerovec/ule/trait.VarULE.html "trait zerovec::ule::VarULE") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), D: [VarULE](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/zerovec/ule/trait.VarULE.html "trait zerovec::ule::VarULE") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), E: [VarULE](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/zerovec/ule/trait.VarULE.html "trait zerovec::ule::VarULE") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Format: [VarZeroVecFormat](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/zerovec/varzerovec/components/trait.VarZeroVecFormat.html "trait zerovec::varzerovec::components::VarZeroVecFormat"),

Available on **crate feature `alloc`** only.

[Source](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/src/zerovec/ule/tuplevar.rs.html#242)

#### type [Owned](#associatedtype.Owned) = [Box](struct.Box.html "struct bevy::prelude::Box")<[Tuple5VarULE](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/zerovec/ule/tuplevar/struct.Tuple5VarULE.html "struct zerovec::ule::tuplevar::Tuple5VarULE")<A, B, C, D, E, Format>>

[Source](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/src/zerovec/ule/tuplevar.rs.html#241)

### impl<A, B, C, D, Format> [ToOwned](trait.ToOwned.html "trait bevy::prelude::ToOwned") for [Tuple4VarULE](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/zerovec/ule/tuplevar/struct.Tuple4VarULE.html "struct zerovec::ule::tuplevar::Tuple4VarULE")<A, B, C, D, Format>

where A: [VarULE](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/zerovec/ule/trait.VarULE.html "trait zerovec::ule::VarULE") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), B: [VarULE](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/zerovec/ule/trait.VarULE.html "trait zerovec::ule::VarULE") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), C: [VarULE](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/zerovec/ule/trait.VarULE.html "trait zerovec::ule::VarULE") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), D: [VarULE](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/zerovec/ule/trait.VarULE.html "trait zerovec::ule::VarULE") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Format: [VarZeroVecFormat](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/zerovec/varzerovec/components/trait.VarZeroVecFormat.html "trait zerovec::varzerovec::components::VarZeroVecFormat"),

Available on **crate feature `alloc`** only.

[Source](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/src/zerovec/ule/tuplevar.rs.html#241)

#### type [Owned](#associatedtype.Owned) = [Box](struct.Box.html "struct bevy::prelude::Box")<[Tuple4VarULE](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/zerovec/ule/tuplevar/struct.Tuple4VarULE.html "struct zerovec::ule::tuplevar::Tuple4VarULE")<A, B, C, D, Format>>

[Source](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/src/zerovec/ule/tuplevar.rs.html#240)

### impl<A, B, C, Format> [ToOwned](trait.ToOwned.html "trait bevy::prelude::ToOwned") for [Tuple3VarULE](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/zerovec/ule/tuplevar/struct.Tuple3VarULE.html "struct zerovec::ule::tuplevar::Tuple3VarULE")<A, B, C, Format>

where A: [VarULE](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/zerovec/ule/trait.VarULE.html "trait zerovec::ule::VarULE") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), B: [VarULE](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/zerovec/ule/trait.VarULE.html "trait zerovec::ule::VarULE") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), C: [VarULE](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/zerovec/ule/trait.VarULE.html "trait zerovec::ule::VarULE") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Format: [VarZeroVecFormat](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/zerovec/varzerovec/components/trait.VarZeroVecFormat.html "trait zerovec::varzerovec::components::VarZeroVecFormat"),

Available on **crate feature `alloc`** only.

[Source](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/src/zerovec/ule/tuplevar.rs.html#240)

#### type [Owned](#associatedtype.Owned) = [Box](struct.Box.html "struct bevy::prelude::Box")<[Tuple3VarULE](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/zerovec/ule/tuplevar/struct.Tuple3VarULE.html "struct zerovec::ule::tuplevar::Tuple3VarULE")<A, B, C, Format>>

[Source](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/src/zerovec/ule/tuplevar.rs.html#239)

### impl<A, B, Format> [ToOwned](trait.ToOwned.html "trait bevy::prelude::ToOwned") for [Tuple2VarULE](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/zerovec/ule/tuplevar/struct.Tuple2VarULE.html "struct zerovec::ule::tuplevar::Tuple2VarULE")<A, B, Format>

where A: [VarULE](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/zerovec/ule/trait.VarULE.html "trait zerovec::ule::VarULE") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), B: [VarULE](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/zerovec/ule/trait.VarULE.html "trait zerovec::ule::VarULE") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Format: [VarZeroVecFormat](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/zerovec/varzerovec/components/trait.VarZeroVecFormat.html "trait zerovec::varzerovec::components::VarZeroVecFormat"),

Available on **crate feature `alloc`** only.

[Source](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/src/zerovec/ule/tuplevar.rs.html#239)

#### type [Owned](#associatedtype.Owned) = [Box](struct.Box.html "struct bevy::prelude::Box")<[Tuple2VarULE](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/zerovec/ule/tuplevar/struct.Tuple2VarULE.html "struct zerovec::ule::tuplevar::Tuple2VarULE")<A, B, Format>>

[Source](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/src/zerovec/ule/vartuple.rs.html#183-186)

### impl<A, V> [ToOwned](trait.ToOwned.html "trait bevy::prelude::ToOwned") for [VarTupleULE](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/zerovec/ule/vartuple/struct.VarTupleULE.html "struct zerovec::ule::vartuple::VarTupleULE")<A, V>

where A: [AsULE](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/zerovec/ule/trait.AsULE.html "trait zerovec::ule::AsULE") + 'static, V: [VarULE](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/zerovec/ule/trait.VarULE.html "trait zerovec::ule::VarULE") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Available on **crate feature `alloc`** only.

[Source](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/src/zerovec/ule/vartuple.rs.html#188)

#### type [Owned](#associatedtype.Owned) = [Box](struct.Box.html "struct bevy::prelude::Box")<[VarTupleULE](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/zerovec/ule/vartuple/struct.VarTupleULE.html "struct zerovec::ule::vartuple::VarTupleULE")<A, V>>

[Source](https://docs.rs/bitvec/1.0.1/x86_64-unknown-linux-gnu/src/bitvec/slice/traits.rs.html#571-574)

### impl<T, O> [ToOwned](trait.ToOwned.html "trait bevy::prelude::ToOwned") for [BitSlice](https://docs.rs/bitvec/1.0.1/x86_64-unknown-linux-gnu/bitvec/slice/struct.BitSlice.html "struct bitvec::slice::BitSlice")<T, O>

where T: [BitStore](https://docs.rs/bitvec/1.0.1/x86_64-unknown-linux-gnu/bitvec/store/trait.BitStore.html "trait bitvec::store::BitStore"), O: [BitOrder](https://docs.rs/bitvec/1.0.1/x86_64-unknown-linux-gnu/bitvec/order/trait.BitOrder.html "trait bitvec::order::BitOrder"),

Available on **crate feature `alloc` and non-`tarpaulin_include`** only.

[Original](https://doc.rust-lang.org/std/primitive.slice.html#impl-ToOwned)

[Source](https://docs.rs/bitvec/1.0.1/x86_64-unknown-linux-gnu/src/bitvec/slice/traits.rs.html#576)

#### type [Owned](#associatedtype.Owned) = [BitVec](https://docs.rs/bitvec/1.0.1/x86_64-unknown-linux-gnu/bitvec/vec/struct.BitVec.html "struct bitvec::vec::BitVec")<T, O>

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74)

### impl<T> [ToOwned](trait.ToOwned.html "trait bevy::prelude::ToOwned") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76)

#### type [Owned](#associatedtype.Owned) = T

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1307)

### impl<T> [ToOwned](trait.ToOwned.html "trait bevy::prelude::ToOwned") for [UniqueEntityEquivalentSlice](../ecs/entity/struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>

where T: [EntityEquivalent](../ecs/entity/trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1308)

#### type [Owned](#associatedtype.Owned) = [UniqueEntityEquivalentVec](../ecs/entity/struct.UniqueEntityEquivalentVec.html "struct bevy::ecs::entity::UniqueEntityEquivalentVec")<T>

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/slice.rs.html#838)

### impl<T> [ToOwned](trait.ToOwned.html "trait bevy::prelude::ToOwned") for [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

Available on **non-`no_global_oom_handling`** only.

[Source](https://doc.rust-lang.org/nightly/src/alloc/slice.rs.html#839)

#### type [Owned](#associatedtype.Owned) = [Vec](struct.Vec.html "struct bevy::prelude::Vec")<T>