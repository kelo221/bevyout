[bevy](../../index.html)::[render](../index.html)::[render\_resource](index.html)

# Trait SpecializerKey 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/specializer.rs.html#214)

```rust
pub trait SpecializerKey:
    Clone
    + Hash
    + Eq {
    type Canonical: Hash + Eq;

    const IS_CANONICAL: bool;
}
```

Defines a type that is able to be used as a key for [`Specializer`](trait.Specializer.html "trait bevy::render::render_resource::Specializer")s

**Most types should implement this trait with the included derive macro.**  
This generates a "canonical" key type, with `IS_CANONICAL = true`, and `Canonical = Self`

### What’s a “canonical” key?

The specialization API memoizes pipelines based on the hash of each key, but this can still produce duplicates. For example, if one used a list of vertex attributes as a key, even if all the same attributes were present they could be in any order. In each case, though the keys would be “different” they would produce the same pipeline.

To address this, during specialization keys are processed into a [canonical](https://en.wikipedia.org/wiki/Canonicalization) (or “standard”) form that represents the actual descriptor that was produced. In the previous example, that would be the final `VertexBufferLayout` contained by the pipeline descriptor. This new key is used by [`Variants`](struct.Variants.html "struct bevy::render::render_resource::Variants") to perform additional checks for duplicates, but only if required. If a key is canonical from the start, then there’s no need.

For implementors: the main property of a canonical key is that if two keys hash differently, they should nearly always produce different descriptors.

## Required Associated Constants

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/specializer.rs.html#217)

#### const [IS\_CANONICAL](#associatedconstant.IS_CANONICAL): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Denotes whether this key is canonical or not. This should only be `true` if and only if `Canonical = Self`.

## Required Associated Types

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/specializer.rs.html#220)

#### type [Canonical](#associatedtype.Canonical): [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq")

The canonical key type to convert this into during specialization.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/specializer.rs.html#259-265)

### impl [SpecializerKey](trait.SpecializerKey.html "trait bevy::render::render_resource::SpecializerKey") for [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/specializer.rs.html#259-265)

#### const [IS\_CANONICAL](#associatedconstant.IS_CANONICAL): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/specializer.rs.html#259-265)

#### type [Canonical](#associatedtype.Canonical) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/specializer.rs.html#259-265)

### impl<T> [SpecializerKey](trait.SpecializerKey.html "trait bevy::render::render_resource::SpecializerKey") for [(T₁, T₂, …, Tₙ)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html#trait-implementations-1)

where T: [SpecializerKey](trait.SpecializerKey.html "trait bevy::render::render_resource::SpecializerKey"),

This trait is implemented for tuples up to 13 items long.

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/specializer.rs.html#259-265)

#### const [IS\_CANONICAL](#associatedconstant.IS_CANONICAL): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/specializer.rs.html#259-265)

#### type [Canonical](#associatedtype.Canonical) = (<T as [SpecializerKey](trait.SpecializerKey.html "trait bevy::render::render_resource::SpecializerKey")\>::[Canonical](trait.SpecializerKey.html#associatedtype.Canonical "type bevy::render::render_resource::SpecializerKey::Canonical"),)

## Implementors

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/contrast_adaptive_sharpening/mod.rs.html#189)

### impl [SpecializerKey](trait.SpecializerKey.html "trait bevy::render::render_resource::SpecializerKey") for [CasPipelineKey](../../anti_alias/contrast_adaptive_sharpening/struct.CasPipelineKey.html "struct bevy::anti_alias::contrast_adaptive_sharpening::CasPipelineKey")

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/contrast_adaptive_sharpening/mod.rs.html#189)

#### const [IS\_CANONICAL](#associatedconstant.IS_CANONICAL): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/contrast_adaptive_sharpening/mod.rs.html#189)

#### type [Canonical](#associatedtype.Canonical) = [CasPipelineKey](../../anti_alias/contrast_adaptive_sharpening/struct.CasPipelineKey.html "struct bevy::anti_alias::contrast_adaptive_sharpening::CasPipelineKey")