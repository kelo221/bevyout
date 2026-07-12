[bevy](../../../index.html)::[log](../../index.html)::[tracing](../index.html)::[callsite](index.html)

# Trait Callsite 

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/callsite.rs.html#125)

```rust
pub trait Callsite: Sync {
    // Required methods
    fn set_interest(&self, interest: Interest);
    fn metadata(&self) -> &Metadata<'_>;
}
```

Trait implemented by callsites.

These functions are only intended to be called by the callsite registry, which correctly handles determining the common interest between all subscribers.

See the [module-level documentation](index.html "mod bevy::log::tracing::callsite") for details on callsites.

## Required Methods

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/callsite.rs.html#133)

#### fn [set\_interest](#tymethod.set_interest)(&self, interest: [Interest](../subscriber/struct.Interest.html "struct bevy::log::tracing::subscriber::Interest"))

Sets the [`Interest`](../subscriber/struct.Interest.html "struct bevy::log::tracing::subscriber::Interest") for this callsite.

See the [documentation on callsite interest caching](index.html#performing-static-filtering "mod bevy::log::tracing::callsite") for details.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/callsite.rs.html#147)

#### fn [metadata](#tymethod.metadata)(&self) -> &[Metadata](../struct.Metadata.html "struct bevy::log::tracing::Metadata")<'\_>

Returns the [metadata](../struct.Metadata.html "struct bevy::log::tracing::Metadata") associated with the callsite.

**Note:** Implementations of this method should not produce [`Metadata`](../struct.Metadata.html "struct bevy::log::tracing::Metadata")
that share the same callsite [`Identifier`](struct.Identifier.html "struct bevy::log::tracing::callsite::Identifier") but otherwise differ in any
way (e.g., have different `name`s).

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/callsite.rs.html#357)

### impl [Callsite](../trait.Callsite.html "trait bevy::log::tracing::Callsite") for [DefaultCallsite](struct.DefaultCallsite.html "struct bevy::log::tracing::callsite::DefaultCallsite")