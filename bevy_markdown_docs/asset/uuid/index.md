[bevy](../../index.html)::[asset](../index.html)

# Crate uuid 

[Source](https://docs.rs/uuid/1.23.2/x86_64-unknown-linux-gnu/src/uuid/lib.rs.html#12-1713)

Generate and parse universally unique identifiers (UUIDs).

Here’s an example of a UUID:

```
67e55044-10b1-426f-9247-bb680e5fe0c8
```

A UUID is a unique 128-bit value, stored as 16 octets, and regularly formatted as a hex string in five groups. UUIDs are used to assign unique identifiers to entities without requiring a central allocating authority.

They are particularly useful in distributed systems, though can be used in disparate areas, such as databases and network protocols. Typically a UUID is displayed in a readable string form as a sequence of hexadecimal digits, separated into groups by hyphens.

The uniqueness property is not strictly guaranteed, however for all practical purposes, it can be assumed that an unintentional collision would be extremely unlikely.

UUIDs have a number of standardized encodings that are specified in [RFC 9562](https://www.ietf.org/rfc/rfc9562.html).

## Getting started

Add the following to your `Cargo.toml`:

```toml
[dependencies.uuid]
version = "1.23.2"
# Lets you generate random UUIDs
features = [
    "v4",
]
```

When you want a UUID, you can generate one:

```rust
use uuid::Uuid;

let id = Uuid::new_v4();
```

If you have a UUID value, you can use its string literal form inline:

```rust
use uuid::{uuid, Uuid};

const ID: Uuid = uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8");
```

## Working with different UUID versions

This library supports all standardized methods for generating UUIDs through individual Cargo features.

By default, this crate depends on nothing but the Rust standard library and can parse and format UUIDs, but cannot generate them. Depending on the kind of UUID you’d like to work with, there are Cargo features that enable generating them:

*   `v1` - Version 1 UUIDs using a timestamp and monotonic counter.
*   `v3` - Version 3 UUIDs based on the MD5 hash of some data.
*   `v4` - Version 4 UUIDs with random data.
*   `v5` - Version 5 UUIDs based on the SHA1 hash of some data.
*   `v6` - Version 6 UUIDs using a timestamp and monotonic counter.
*   `v7` - Version 7 UUIDs using a Unix timestamp.
*   `v8` - Version 8 UUIDs using user-defined data.

This library also includes a [`Builder`](struct.Builder.html "struct bevy::asset::uuid::Builder") type that can be used to help construct UUIDs of any version without any additional dependencies or features. It’s a lower-level API than [`Uuid`](struct.Uuid.html "struct bevy::asset::uuid::Uuid") that can be used when you need control over implicit requirements on things like a source of randomness.

### Which UUID version should I use?

If you just want to generate unique identifiers then consider version 4 (`v4`) UUIDs. If you want to use UUIDs as database keys or need to sort them then consider version 7 (`v7`) UUIDs. Other versions should generally be avoided unless there’s an existing need for them.

Some UUID versions supersede others. Prefer version 6 over version 1 and version 5 over version 3.

## Other features

Other crate features can also be useful beyond the version support:

*   `serde` - adds the ability to serialize and deserialize a UUID using `serde`.
*   `borsh` - adds the ability to serialize and deserialize a UUID using `borsh`.
*   `arbitrary` - adds an `Arbitrary` trait implementation to `Uuid` for fuzzing.
*   `fast-rng` - uses a faster algorithm for generating random UUIDs when available. This feature requires more dependencies to compile, but is just as suitable for UUIDs as the default algorithm.
*   `rng-rand` - forces `rand` as the backend for randomness.
*   `rng-getrandom` - forces `getrandom` as the backend for randomness.
*   `bytemuck` - adds a `Pod` trait implementation to `Uuid` for byte manipulation

## Unstable features

Some features are unstable. They may be incomplete or depend on other unstable libraries. These include:

*   `zerocopy` - adds support for zero-copy deserialization using the `zerocopy` library.

Unstable features may break between minor releases.

To allow unstable features, you’ll need to enable the Cargo feature as normal, but also pass an additional flag through your environment to opt-in to unstable `uuid` features:

```
RUSTFLAGS="--cfg uuid_unstable"
```

## Building for other targets

### WebAssembly

For WebAssembly, enable the `js` feature:

```toml
[dependencies.uuid]
version = "1.23.2"
features = [
    "v4",
    "v7",
    "js",
]
```

### Embedded

For embedded targets without the standard library, you’ll need to disable default features when building `uuid`:

```toml
[dependencies.uuid]
version = "1.23.2"
default-features = false
```

Some additional features are supported in no-std environments:

*   `v1`, `v3`, `v5`, `v6`, and `v8`.
*   `serde`.

If you need to use `v4` or `v7` in a no-std environment, you’ll need to produce random bytes yourself and then pass them to [`Builder::from_random_bytes`](struct.Builder.html#method.from_random_bytes "associated function bevy::asset::uuid::Builder::from_random_bytes") without enabling the `v4` or `v7` features.

If you’re using `getrandom`, you can specify the `rng-getrandom` or `rng-rand` features of `uuid` and configure `getrandom`’s provider per its docs. `uuid` may upgrade its version of `getrandom` in minor releases.

## Examples

Parse a UUID given in the simple format and print it as a URN:

```rust
let my_uuid = Uuid::parse_str("a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8")?;

println!("{}", my_uuid.urn());
```

Generate a random UUID and print it out in hexadecimal form:

```rust
// Note that this requires the `v4` feature to be enabled.
let my_uuid = Uuid::new_v4();

println!("{}", my_uuid);
```

## References

*   [Wikipedia: Universally Unique Identifier](http://en.wikipedia.org/wiki/Universally_unique_identifier)
*   [RFC 9562: Universally Unique IDentifiers (UUID)](https://www.ietf.org/rfc/rfc9562.html).

## Modules

[fmt](fmt/index.html "mod bevy::asset::uuid::fmt")

Adapters for alternative string formats.

[serde](serde/index.html "mod bevy::asset::uuid::serde")`serde`

Adapters for alternative `serde` formats.

[timestamp](timestamp/index.html "mod bevy::asset::uuid::timestamp")

Generating UUIDs from timestamps.

## Macros

[uuid](macro.uuid.html "macro bevy::asset::uuid::uuid")

Parse [`Uuid`](https://docs.rs/uuid/*/uuid/struct.Uuid.html)s from string literals at compile time.

## Structs

[Builder](struct.Builder.html "struct bevy::asset::uuid::Builder")

A builder for creating a UUID.

[Error](struct.Error.html "struct bevy::asset::uuid::Error")

A general error that can occur when working with UUIDs.

[NoContext](struct.NoContext.html "struct bevy::asset::uuid::NoContext")

An empty counter that will always return the value `0`.

[NonNilUuid](struct.NonNilUuid.html "struct bevy::asset::uuid::NonNilUuid")

A UUID that is guaranteed not to be the [nil UUID](https://www.ietf.org/rfc/rfc9562.html#name-nil-uuid).

[Timestamp](struct.Timestamp.html "struct bevy::asset::uuid::Timestamp")

A timestamp that can be encoded into a UUID.

[Uuid](struct.Uuid.html "struct bevy::asset::uuid::Uuid")

A Universally Unique Identifier (UUID).

## Enums

[Variant](enum.Variant.html "enum bevy::asset::uuid::Variant")

The reserved variants of UUIDs.

[Version](enum.Version.html "enum bevy::asset::uuid::Version")

The version of the UUID, denoting the generating algorithm.

## Traits

[ClockSequence](trait.ClockSequence.html "trait bevy::asset::uuid::ClockSequence")

A counter that can be used by versions 1 and 6 UUIDs to support the uniqueness of timestamps.

## Type Aliases

[Bytes](type.Bytes.html "type bevy::asset::uuid::Bytes")

A 128-bit (16 byte) buffer containing the UUID.