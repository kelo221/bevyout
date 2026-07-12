[bevy](../../index.html)::[material](../index.html)::[labels](index.html)

# Trait ShaderLabel 

[Source](https://docs.rs/bevy_material/0.19.0/x86_64-unknown-linux-gnu/src/bevy_material/labels.rs.html#5-12)

```rust
pub trait ShaderLabel:
    Send
    + Sync
    + Debug
    + DynEq
    + DynHash {
    // Required method
    fn dyn_clone(&self) -> Box<dyn ShaderLabel>;

    // Provided method
    fn intern(&self) -> Interned<dyn ShaderLabel>
       where Self: Sized { ... }
}
```

Labels used to uniquely identify types of material shaders

## Required Methods

[Source](https://docs.rs/bevy_material/0.19.0/x86_64-unknown-linux-gnu/src/bevy_material/labels.rs.html#5-12)

#### fn [dyn\_clone](#tymethod.dyn_clone)(&self) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [ShaderLabel](trait.ShaderLabel.html "trait bevy::material::labels::ShaderLabel")\>

Clones this `ShaderLabel`.

## Provided Methods

[Source](https://docs.rs/bevy_material/0.19.0/x86_64-unknown-linux-gnu/src/bevy_material/labels.rs.html#5-12)

#### fn [intern](#method.intern)(&self) -> [Interned](../../ecs/intern/struct.Interned.html "struct bevy::ecs::intern::Interned")<dyn [ShaderLabel](trait.ShaderLabel.html "trait bevy::material::labels::ShaderLabel")\>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Returns an [`Interned`](../../ecs/intern/struct.Interned.html "struct bevy::ecs::intern::Interned") value corresponding to `self`.

## Trait Implementations

[Source](https://docs.rs/bevy_material/0.19.0/x86_64-unknown-linux-gnu/src/bevy_material/labels.rs.html#5-12)

### impl [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") for dyn [ShaderLabel](trait.ShaderLabel.html "trait bevy::material::labels::ShaderLabel")

[Source](https://docs.rs/bevy_material/0.19.0/x86_64-unknown-linux-gnu/src/bevy_material/labels.rs.html#5-12)

### impl [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") for dyn [ShaderLabel](trait.ShaderLabel.html "trait bevy::material::labels::ShaderLabel")

[Source](https://docs.rs/bevy_material/0.19.0/x86_64-unknown-linux-gnu/src/bevy_material/labels.rs.html#5-12)

#### fn [hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)<H>(&self, state: [&mut H](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

where H: [Hasher](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"),

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

[Source](https://docs.rs/bevy_material/0.19.0/x86_64-unknown-linux-gnu/src/bevy_material/labels.rs.html#5-12)

### impl [Internable](../../ecs/intern/trait.Internable.html "trait bevy::ecs::intern::Internable") for dyn [ShaderLabel](trait.ShaderLabel.html "trait bevy::material::labels::ShaderLabel")

[Source](https://docs.rs/bevy_material/0.19.0/x86_64-unknown-linux-gnu/src/bevy_material/labels.rs.html#5-12)

#### fn [leak](../../ecs/intern/trait.Internable.html#tymethod.leak)(&self) -> &'static dyn [ShaderLabel](trait.ShaderLabel.html "trait bevy::material::labels::ShaderLabel")

Creates a static reference to `self`, possibly leaking memory.

[Source](https://docs.rs/bevy_material/0.19.0/x86_64-unknown-linux-gnu/src/bevy_material/labels.rs.html#5-12)

#### fn [ref\_eq](../../ecs/intern/trait.Internable.html#tymethod.ref_eq)(&self, other: &(dyn [ShaderLabel](trait.ShaderLabel.html "trait bevy::material::labels::ShaderLabel") + 'static)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if the two references point to the same value.

[Source](https://docs.rs/bevy_material/0.19.0/x86_64-unknown-linux-gnu/src/bevy_material/labels.rs.html#5-12)

#### fn [ref\_hash](../../ecs/intern/trait.Internable.html#tymethod.ref_hash)<H>(&self, state: [&mut H](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

where H: [Hasher](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"),

Feeds the reference to the hasher.

[Source](https://docs.rs/bevy_material/0.19.0/x86_64-unknown-linux-gnu/src/bevy_material/labels.rs.html#5-12)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") for dyn [ShaderLabel](trait.ShaderLabel.html "trait bevy::material::labels::ShaderLabel")

[Source](https://docs.rs/bevy_material/0.19.0/x86_64-unknown-linux-gnu/src/bevy_material/labels.rs.html#5-12)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &(dyn [ShaderLabel](trait.ShaderLabel.html "trait bevy::material::labels::ShaderLabel") + 'static)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#1395)

### impl [ShaderLabel](trait.ShaderLabel.html "trait bevy::material::labels::ShaderLabel") for [DeferredFragmentShader](../../pbr/struct.DeferredFragmentShader.html "struct bevy::pbr::DeferredFragmentShader")

where [DeferredFragmentShader](../../pbr/struct.DeferredFragmentShader.html "struct bevy::pbr::DeferredFragmentShader"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#1392)

### impl [ShaderLabel](trait.ShaderLabel.html "trait bevy::material::labels::ShaderLabel") for [DeferredVertexShader](../../pbr/struct.DeferredVertexShader.html "struct bevy::pbr::DeferredVertexShader")

where [DeferredVertexShader](../../pbr/struct.DeferredVertexShader.html "struct bevy::pbr::DeferredVertexShader"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_material/0.19.0/x86_64-unknown-linux-gnu/src/bevy_material/labels.rs.html#5-12)

### impl [ShaderLabel](trait.ShaderLabel.html "trait bevy::material::labels::ShaderLabel") for [Interned](../../ecs/intern/struct.Interned.html "struct bevy::ecs::intern::Interned")<dyn [ShaderLabel](trait.ShaderLabel.html "trait bevy::material::labels::ShaderLabel")\>

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#1383)

### impl [ShaderLabel](trait.ShaderLabel.html "trait bevy::material::labels::ShaderLabel") for [MaterialFragmentShader](../../pbr/struct.MaterialFragmentShader.html "struct bevy::pbr::MaterialFragmentShader")

where [MaterialFragmentShader](../../pbr/struct.MaterialFragmentShader.html "struct bevy::pbr::MaterialFragmentShader"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#1380)

### impl [ShaderLabel](trait.ShaderLabel.html "trait bevy::material::labels::ShaderLabel") for [MaterialVertexShader](../../pbr/struct.MaterialVertexShader.html "struct bevy::pbr::MaterialVertexShader")

where [MaterialVertexShader](../../pbr/struct.MaterialVertexShader.html "struct bevy::pbr::MaterialVertexShader"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#1404)

### impl [ShaderLabel](trait.ShaderLabel.html "trait bevy::material::labels::ShaderLabel") for [MeshletDeferredFragmentShader](../../pbr/struct.MeshletDeferredFragmentShader.html "struct bevy::pbr::MeshletDeferredFragmentShader")

where [MeshletDeferredFragmentShader](../../pbr/struct.MeshletDeferredFragmentShader.html "struct bevy::pbr::MeshletDeferredFragmentShader"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#1398)

### impl [ShaderLabel](trait.ShaderLabel.html "trait bevy::material::labels::ShaderLabel") for [MeshletFragmentShader](../../pbr/struct.MeshletFragmentShader.html "struct bevy::pbr::MeshletFragmentShader")

where [MeshletFragmentShader](../../pbr/struct.MeshletFragmentShader.html "struct bevy::pbr::MeshletFragmentShader"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#1401)

### impl [ShaderLabel](trait.ShaderLabel.html "trait bevy::material::labels::ShaderLabel") for [MeshletPrepassFragmentShader](../../pbr/struct.MeshletPrepassFragmentShader.html "struct bevy::pbr::MeshletPrepassFragmentShader")

where [MeshletPrepassFragmentShader](../../pbr/struct.MeshletPrepassFragmentShader.html "struct bevy::pbr::MeshletPrepassFragmentShader"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#1389)

### impl [ShaderLabel](trait.ShaderLabel.html "trait bevy::material::labels::ShaderLabel") for [PrepassFragmentShader](../../pbr/struct.PrepassFragmentShader.html "struct bevy::pbr::PrepassFragmentShader")

where [PrepassFragmentShader](../../pbr/struct.PrepassFragmentShader.html "struct bevy::pbr::PrepassFragmentShader"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#1386)

### impl [ShaderLabel](trait.ShaderLabel.html "trait bevy::material::labels::ShaderLabel") for [PrepassVertexShader](../../pbr/struct.PrepassVertexShader.html "struct bevy::pbr::PrepassVertexShader")

where [PrepassVertexShader](../../pbr/struct.PrepassVertexShader.html "struct bevy::pbr::PrepassVertexShader"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),