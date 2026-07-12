[bevy](../../index.html)::[material](../index.html)::[labels](index.html)

# Trait DrawFunctionLabel 

[Source](https://docs.rs/bevy_material/0.19.0/x86_64-unknown-linux-gnu/src/bevy_material/labels.rs.html#19-26)

```rust
pub trait DrawFunctionLabel:
    Send
    + Sync
    + Debug
    + DynEq
    + DynHash {
    // Required method
    fn dyn_clone(&self) -> Box<dyn DrawFunctionLabel>;

    // Provided method
    fn intern(&self) -> Interned<dyn DrawFunctionLabel>
       where Self: Sized { ... }
}
```

Labels used to uniquely identify types of material shaders

## Required Methods

[Source](https://docs.rs/bevy_material/0.19.0/x86_64-unknown-linux-gnu/src/bevy_material/labels.rs.html#19-26)

#### fn [dyn\_clone](#tymethod.dyn_clone)(&self) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [DrawFunctionLabel](trait.DrawFunctionLabel.html "trait bevy::material::labels::DrawFunctionLabel")\>

Clones this `DrawFunctionLabel`.

## Provided Methods

[Source](https://docs.rs/bevy_material/0.19.0/x86_64-unknown-linux-gnu/src/bevy_material/labels.rs.html#19-26)

#### fn [intern](#method.intern)(&self) -> [Interned](../../ecs/intern/struct.Interned.html "struct bevy::ecs::intern::Interned")<dyn [DrawFunctionLabel](trait.DrawFunctionLabel.html "trait bevy::material::labels::DrawFunctionLabel")\>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Returns an [`Interned`](../../ecs/intern/struct.Interned.html "struct bevy::ecs::intern::Interned") value corresponding to `self`.

## Trait Implementations

[Source](https://docs.rs/bevy_material/0.19.0/x86_64-unknown-linux-gnu/src/bevy_material/labels.rs.html#19-26)

### impl [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") for dyn [DrawFunctionLabel](trait.DrawFunctionLabel.html "trait bevy::material::labels::DrawFunctionLabel")

[Source](https://docs.rs/bevy_material/0.19.0/x86_64-unknown-linux-gnu/src/bevy_material/labels.rs.html#19-26)

### impl [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") for dyn [DrawFunctionLabel](trait.DrawFunctionLabel.html "trait bevy::material::labels::DrawFunctionLabel")

[Source](https://docs.rs/bevy_material/0.19.0/x86_64-unknown-linux-gnu/src/bevy_material/labels.rs.html#19-26)

#### fn [hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)<H>(&self, state: [&mut H](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

where H: [Hasher](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"),

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

[Source](https://docs.rs/bevy_material/0.19.0/x86_64-unknown-linux-gnu/src/bevy_material/labels.rs.html#19-26)

### impl [Internable](../../ecs/intern/trait.Internable.html "trait bevy::ecs::intern::Internable") for dyn [DrawFunctionLabel](trait.DrawFunctionLabel.html "trait bevy::material::labels::DrawFunctionLabel")

[Source](https://docs.rs/bevy_material/0.19.0/x86_64-unknown-linux-gnu/src/bevy_material/labels.rs.html#19-26)

#### fn [leak](../../ecs/intern/trait.Internable.html#tymethod.leak)(&self) -> &'static dyn [DrawFunctionLabel](trait.DrawFunctionLabel.html "trait bevy::material::labels::DrawFunctionLabel")

Creates a static reference to `self`, possibly leaking memory.

[Source](https://docs.rs/bevy_material/0.19.0/x86_64-unknown-linux-gnu/src/bevy_material/labels.rs.html#19-26)

#### fn [ref\_eq](../../ecs/intern/trait.Internable.html#tymethod.ref_eq)(&self, other: &(dyn [DrawFunctionLabel](trait.DrawFunctionLabel.html "trait bevy::material::labels::DrawFunctionLabel") + 'static)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if the two references point to the same value.

[Source](https://docs.rs/bevy_material/0.19.0/x86_64-unknown-linux-gnu/src/bevy_material/labels.rs.html#19-26)

#### fn [ref\_hash](../../ecs/intern/trait.Internable.html#tymethod.ref_hash)<H>(&self, state: [&mut H](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

where H: [Hasher](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"),

Feeds the reference to the hasher.

[Source](https://docs.rs/bevy_material/0.19.0/x86_64-unknown-linux-gnu/src/bevy_material/labels.rs.html#19-26)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") for dyn [DrawFunctionLabel](trait.DrawFunctionLabel.html "trait bevy::material::labels::DrawFunctionLabel")

[Source](https://docs.rs/bevy_material/0.19.0/x86_64-unknown-linux-gnu/src/bevy_material/labels.rs.html#19-26)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &(dyn [DrawFunctionLabel](trait.DrawFunctionLabel.html "trait bevy::material::labels::DrawFunctionLabel") + 'static)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#1425)

### impl [DrawFunctionLabel](trait.DrawFunctionLabel.html "trait bevy::material::labels::DrawFunctionLabel") for [DeferredAlphaMaskDrawFunction](../../pbr/struct.DeferredAlphaMaskDrawFunction.html "struct bevy::pbr::DeferredAlphaMaskDrawFunction")

where [DeferredAlphaMaskDrawFunction](../../pbr/struct.DeferredAlphaMaskDrawFunction.html "struct bevy::pbr::DeferredAlphaMaskDrawFunction"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#1423)

### impl [DrawFunctionLabel](trait.DrawFunctionLabel.html "trait bevy::material::labels::DrawFunctionLabel") for [DeferredOpaqueDrawFunction](../../pbr/struct.DeferredOpaqueDrawFunction.html "struct bevy::pbr::DeferredOpaqueDrawFunction")

where [DeferredOpaqueDrawFunction](../../pbr/struct.DeferredOpaqueDrawFunction.html "struct bevy::pbr::DeferredOpaqueDrawFunction"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_material/0.19.0/x86_64-unknown-linux-gnu/src/bevy_material/labels.rs.html#19-26)

### impl [DrawFunctionLabel](trait.DrawFunctionLabel.html "trait bevy::material::labels::DrawFunctionLabel") for [Interned](../../ecs/intern/struct.Interned.html "struct bevy::ecs::intern::Interned")<dyn [DrawFunctionLabel](trait.DrawFunctionLabel.html "trait bevy::material::labels::DrawFunctionLabel")\>

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#1409)

### impl [DrawFunctionLabel](trait.DrawFunctionLabel.html "trait bevy::material::labels::DrawFunctionLabel") for [MainPassAlphaMaskDrawFunction](../../pbr/struct.MainPassAlphaMaskDrawFunction.html "struct bevy::pbr::MainPassAlphaMaskDrawFunction")

where [MainPassAlphaMaskDrawFunction](../../pbr/struct.MainPassAlphaMaskDrawFunction.html "struct bevy::pbr::MainPassAlphaMaskDrawFunction"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#1407)

### impl [DrawFunctionLabel](trait.DrawFunctionLabel.html "trait bevy::material::labels::DrawFunctionLabel") for [MainPassOpaqueDrawFunction](../../pbr/struct.MainPassOpaqueDrawFunction.html "struct bevy::pbr::MainPassOpaqueDrawFunction")

where [MainPassOpaqueDrawFunction](../../pbr/struct.MainPassOpaqueDrawFunction.html "struct bevy::pbr::MainPassOpaqueDrawFunction"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#1411)

### impl [DrawFunctionLabel](trait.DrawFunctionLabel.html "trait bevy::material::labels::DrawFunctionLabel") for [MainPassTransmissiveDrawFunction](../../pbr/struct.MainPassTransmissiveDrawFunction.html "struct bevy::pbr::MainPassTransmissiveDrawFunction")

where [MainPassTransmissiveDrawFunction](../../pbr/struct.MainPassTransmissiveDrawFunction.html "struct bevy::pbr::MainPassTransmissiveDrawFunction"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#1413)

### impl [DrawFunctionLabel](trait.DrawFunctionLabel.html "trait bevy::material::labels::DrawFunctionLabel") for [MainPassTransparentDrawFunction](../../pbr/struct.MainPassTransparentDrawFunction.html "struct bevy::pbr::MainPassTransparentDrawFunction")

where [MainPassTransparentDrawFunction](../../pbr/struct.MainPassTransparentDrawFunction.html "struct bevy::pbr::MainPassTransparentDrawFunction"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#1418)

### impl [DrawFunctionLabel](trait.DrawFunctionLabel.html "trait bevy::material::labels::DrawFunctionLabel") for [PrepassAlphaMaskDrawFunction](../../pbr/struct.PrepassAlphaMaskDrawFunction.html "struct bevy::pbr::PrepassAlphaMaskDrawFunction")

where [PrepassAlphaMaskDrawFunction](../../pbr/struct.PrepassAlphaMaskDrawFunction.html "struct bevy::pbr::PrepassAlphaMaskDrawFunction"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#1420)

### impl [DrawFunctionLabel](trait.DrawFunctionLabel.html "trait bevy::material::labels::DrawFunctionLabel") for [PrepassOpaqueDepthOnlyDrawFunction](../../pbr/struct.PrepassOpaqueDepthOnlyDrawFunction.html "struct bevy::pbr::PrepassOpaqueDepthOnlyDrawFunction")

where [PrepassOpaqueDepthOnlyDrawFunction](../../pbr/struct.PrepassOpaqueDepthOnlyDrawFunction.html "struct bevy::pbr::PrepassOpaqueDepthOnlyDrawFunction"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#1416)

### impl [DrawFunctionLabel](trait.DrawFunctionLabel.html "trait bevy::material::labels::DrawFunctionLabel") for [PrepassOpaqueDrawFunction](../../pbr/struct.PrepassOpaqueDrawFunction.html "struct bevy::pbr::PrepassOpaqueDrawFunction")

where [PrepassOpaqueDrawFunction](../../pbr/struct.PrepassOpaqueDrawFunction.html "struct bevy::pbr::PrepassOpaqueDrawFunction"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#1430)

### impl [DrawFunctionLabel](trait.DrawFunctionLabel.html "trait bevy::material::labels::DrawFunctionLabel") for [ShadowsDepthOnlyDrawFunction](../../pbr/struct.ShadowsDepthOnlyDrawFunction.html "struct bevy::pbr::ShadowsDepthOnlyDrawFunction")

where [ShadowsDepthOnlyDrawFunction](../../pbr/struct.ShadowsDepthOnlyDrawFunction.html "struct bevy::pbr::ShadowsDepthOnlyDrawFunction"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#1428)

### impl [DrawFunctionLabel](trait.DrawFunctionLabel.html "trait bevy::material::labels::DrawFunctionLabel") for [ShadowsDrawFunction](../../pbr/struct.ShadowsDrawFunction.html "struct bevy::pbr::ShadowsDrawFunction")

where [ShadowsDrawFunction](../../pbr/struct.ShadowsDrawFunction.html "struct bevy::pbr::ShadowsDrawFunction"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),