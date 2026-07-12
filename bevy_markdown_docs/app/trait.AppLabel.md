[bevy](../index.html)::[app](index.html)

# Trait AppLabel 

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#39-46)

```rust
pub trait AppLabel:
    Send
    + Sync
    + Debug
    + DynEq
    + DynHash {
    // Required method
    fn dyn_clone(&self) -> Box<dyn AppLabel>;

    // Provided method
    fn intern(&self) -> Interned<dyn AppLabel>
       where Self: Sized { ... }
}
```

A strongly-typed class of labels used to identify an [`App`](../prelude/struct.App.html "struct bevy::prelude::App").

## Required Methods

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#39-46)

#### fn [dyn\_clone](#tymethod.dyn_clone)(&self) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [AppLabel](trait.AppLabel.html "trait bevy::app::AppLabel")\>

Clones this `AppLabel`.

## Provided Methods

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#39-46)

#### fn [intern](#method.intern)(&self) -> [Interned](../ecs/intern/struct.Interned.html "struct bevy::ecs::intern::Interned")<dyn [AppLabel](trait.AppLabel.html "trait bevy::app::AppLabel")\>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Returns an [`Interned`](../ecs/intern/struct.Interned.html "struct bevy::ecs::intern::Interned") value corresponding to `self`.

## Trait Implementations

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#39-46)

### impl [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") for dyn [AppLabel](trait.AppLabel.html "trait bevy::app::AppLabel")

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#39-46)

### impl [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") for dyn [AppLabel](trait.AppLabel.html "trait bevy::app::AppLabel")

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#39-46)

#### fn [hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)<H>(&self, state: [&mut H](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

where H: [Hasher](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"),

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#39-46)

### impl [Internable](../ecs/intern/trait.Internable.html "trait bevy::ecs::intern::Internable") for dyn [AppLabel](trait.AppLabel.html "trait bevy::app::AppLabel")

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#39-46)

#### fn [leak](../ecs/intern/trait.Internable.html#tymethod.leak)(&self) -> &'static dyn [AppLabel](trait.AppLabel.html "trait bevy::app::AppLabel")

Creates a static reference to `self`, possibly leaking memory.

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#39-46)

#### fn [ref\_eq](../ecs/intern/trait.Internable.html#tymethod.ref_eq)(&self, other: &(dyn [AppLabel](trait.AppLabel.html "trait bevy::app::AppLabel") + 'static)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if the two references point to the same value.

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#39-46)

#### fn [ref\_hash](../ecs/intern/trait.Internable.html#tymethod.ref_hash)<H>(&self, state: [&mut H](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

where H: [Hasher](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"),

Feeds the reference to the hasher.

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#39-46)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") for dyn [AppLabel](trait.AppLabel.html "trait bevy::app::AppLabel")

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#39-46)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &(dyn [AppLabel](trait.AppLabel.html "trait bevy::app::AppLabel") + 'static)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#39-46)

### impl [AppLabel](trait.AppLabel.html "trait bevy::app::AppLabel") for [Interned](../ecs/intern/struct.Interned.html "struct bevy::ecs::intern::Interned")<dyn [AppLabel](trait.AppLabel.html "trait bevy::app::AppLabel")\>

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/lib.rs.html#345)

### impl [AppLabel](trait.AppLabel.html "trait bevy::app::AppLabel") for [RenderApp](../render/struct.RenderApp.html "struct bevy::render::RenderApp")

where [RenderApp](../render/struct.RenderApp.html "struct bevy::render::RenderApp"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/pipelined_rendering.rs.html#17)

### impl [AppLabel](trait.AppLabel.html "trait bevy::app::AppLabel") for [RenderExtractApp](../render/pipelined_rendering/struct.RenderExtractApp.html "struct bevy::render::pipelined_rendering::RenderExtractApp")

where [RenderExtractApp](../render/pipelined_rendering/struct.RenderExtractApp.html "struct bevy::render::pipelined_rendering::RenderExtractApp"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),