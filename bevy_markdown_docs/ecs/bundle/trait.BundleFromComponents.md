[bevy](../../index.html)::[ecs](../index.html)::[bundle](index.html)

# Trait BundleFromComponents 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/mod.rs.html#233)

```rust
pub unsafe trait BundleFromComponents { }
```

Creates a [`Bundle`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") by taking it from internal storage.

## Safety

Manual implementations of this trait are unsupported. That is, there is no safe way to implement this trait, and you must not do so. If you want a type to implement [`Bundle`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"), you must use [`derive@Bundle`](../../prelude/derive.Bundle.html "derive bevy::prelude::Bundle").

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#176-183)

### impl [BundleFromComponents](trait.BundleFromComponents.html "trait bevy::ecs::bundle::BundleFromComponents") for [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#176-183)

### impl<B> [BundleFromComponents](trait.BundleFromComponents.html "trait bevy::ecs::bundle::BundleFromComponents") for [(B₁, B₂, …, Bₙ)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html#trait-implementations-1)

where B: [BundleFromComponents](trait.BundleFromComponents.html "trait bevy::ecs::bundle::BundleFromComponents"),

This trait is implemented for tuples up to 16 items long.

## Implementors

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#30)

### impl<C> [BundleFromComponents](trait.BundleFromComponents.html "trait bevy::ecs::bundle::BundleFromComponents") for C

where C: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),