[bevy](../../index.html)::[ecs](../index.html)::[bundle](index.html)

# Trait NoBundleEffect 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/mod.rs.html#304)

```rust
pub trait NoBundleEffect { }
```

A trait implemented for [`DynamicBundle::Effect`](trait.DynamicBundle.html#associatedtype.Effect "associated type bevy::ecs::bundle::DynamicBundle::Effect") implementations that do nothing. This is used as a type constraint for [`Bundle`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") APIs that do not / cannot run [`DynamicBundle::Effect`](trait.DynamicBundle.html#associatedtype.Effect "associated type bevy::ecs::bundle::DynamicBundle::Effect"), such as “batch spawn” APIs.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#176-183)

### impl [NoBundleEffect](trait.NoBundleEffect.html "trait bevy::ecs::bundle::NoBundleEffect") for [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#176-183)

### impl<B> [NoBundleEffect](trait.NoBundleEffect.html "trait bevy::ecs::bundle::NoBundleEffect") for [(B₁, B₂, …, Bₙ)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html#trait-implementations-1)

where B: [NoBundleEffect](trait.NoBundleEffect.html "trait bevy::ecs::bundle::NoBundleEffect"),

This trait is implemented for tuples up to 16 items long.

## Implementors