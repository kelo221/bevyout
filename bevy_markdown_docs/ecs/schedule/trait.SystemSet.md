[bevy](../../index.html)::[ecs](../index.html)::[schedule](index.html)

# Trait SystemSet 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/set.rs.html#63-176)

```rust
pub trait SystemSet:
    Send
    + Sync
    + Debug
    + DynEq
    + DynHash {
    // Required method
    fn dyn_clone(&self) -> Box<dyn SystemSet>;

    // Provided methods
    fn system_type(&self) -> Option<TypeId> { ... }
    fn is_anonymous(&self) -> bool { ... }
    fn intern(&self) -> Interned<dyn SystemSet>
       where Self: Sized { ... }
}
```

System sets are tag-like labels that can be used to group systems together.

This allows you to share configuration (like run conditions) across multiple systems, and order systems or system sets relative to conceptual groups of systems. To control the behavior of a system set as a whole, use [`Schedule::configure_sets`](../../prelude/struct.Schedule.html#method.configure_sets "method bevy::prelude::Schedule::configure_sets"), or the method of the same name on `App`.

Systems can belong to any number of system sets, reflecting multiple roles or facets that they might have. For example, you may want to annotate a system as “consumes input” and “applies forces”, and ensure that your systems are ordered correctly for both of those sets.

System sets can belong to any number of other system sets, allowing you to create nested hierarchies of system sets to group systems together. Configuration applied to system sets will flow down to their members (including other system sets), allowing you to set and modify the configuration in a single place.

Systems sets are also useful for exposing a consistent public API for dependencies to hook into across versions of your crate, allowing them to add systems to a specific set, or order relative to that set, without leaking implementation details of the exact systems involved.

### Defining new system sets

To create a new system set, use the `#[derive(SystemSet)]` macro. Unit structs are a good choice for one-off sets.

```rust
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct PhysicsSystems;
```

When you want to define several related system sets, consider creating an enum system set. Each variant will be treated as a separate system set.

```rust
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum CombatSystems {
   TargetSelection,
   DamageCalculation,
   Cleanup,
}
```

By convention, the listed order of the system set in the enum corresponds to the order in which the systems are run. Ordering must be explicitly added to ensure that this is the case, but following this convention will help avoid confusion.

#### Adding systems to system sets

To add systems to a system set, call [`in_set`](../../prelude/trait.IntoScheduleConfigs.html#method.in_set "method bevy::prelude::IntoScheduleConfigs::in_set") on the system function while adding it to your app or schedule.

Like usual, these methods can be chained with other configuration methods like [`before`](../../prelude/trait.IntoScheduleConfigs.html#method.before "method bevy::prelude::IntoScheduleConfigs::before"), or repeated to add systems to multiple sets.

```rust
use bevy_ecs::prelude::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum CombatSystems {
   TargetSelection,
   DamageCalculation,
   Cleanup,
}

fn target_selection() {}

fn enemy_damage_calculation() {}

fn player_damage_calculation() {}

let mut schedule = Schedule::default();
// Configuring the sets to run in order.
schedule.configure_sets((CombatSystems::TargetSelection, CombatSystems::DamageCalculation, CombatSystems::Cleanup).chain());

// Adding a single system to a set.
schedule.add_systems(target_selection.in_set(CombatSystems::TargetSelection));

// Adding multiple systems to a set.
schedule.add_systems((player_damage_calculation, enemy_damage_calculation).in_set(CombatSystems::DamageCalculation));
```

## Required Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/set.rs.html#63-176)

#### fn [dyn\_clone](#tymethod.dyn_clone)(&self) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet")\>

Clones this `SystemSet`.

## Provided Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/set.rs.html#158)

#### fn [system\_type](#method.system_type)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId")\>

Returns `Some` if this system set is a [`SystemTypeSet`](struct.SystemTypeSet.html "struct bevy::ecs::schedule::SystemTypeSet").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/set.rs.html#163)

#### fn [is\_anonymous](#method.is_anonymous)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if this system set is an [`AnonymousSet`](struct.AnonymousSet.html "struct bevy::ecs::schedule::AnonymousSet").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/set.rs.html#63-176)

#### fn [intern](#method.intern)(&self) -> [Interned](../intern/struct.Interned.html "struct bevy::ecs::intern::Interned")<dyn [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet")\>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Returns an [`Interned`](../intern/struct.Interned.html "struct bevy::ecs::intern::Interned") value corresponding to `self`.

## Trait Implementations

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/set.rs.html#63-176)

### impl [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") for dyn [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/set.rs.html#63-176)

### impl [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") for dyn [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/set.rs.html#63-176)

#### fn [hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)<H>(&self, state: [&mut H](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

where H: [Hasher](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"),

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/set.rs.html#63-176)

### impl [Internable](../intern/trait.Internable.html "trait bevy::ecs::intern::Internable") for dyn [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/set.rs.html#63-176)

#### fn [leak](../intern/trait.Internable.html#tymethod.leak)(&self) -> &'static dyn [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet")

Creates a static reference to `self`, possibly leaking memory.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/set.rs.html#63-176)

#### fn [ref\_eq](../intern/trait.Internable.html#tymethod.ref_eq)(&self, other: &(dyn [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet") + 'static)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if the two references point to the same value.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/set.rs.html#63-176)

#### fn [ref\_hash](../intern/trait.Internable.html#tymethod.ref_hash)<H>(&self, state: [&mut H](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

where H: [Hasher](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"),

Feeds the reference to the hasher.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/set.rs.html#63-176)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") for dyn [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/set.rs.html#63-176)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &(dyn [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet") + 'static)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_a11y/0.19.0/x86_64-unknown-linux-gnu/src/bevy_a11y/lib.rs.html#249)

### impl [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet") for [AccessibilitySystems](../../a11y/enum.AccessibilitySystems.html "enum bevy::a11y::AccessibilitySystems")

where [AccessibilitySystems](../../a11y/enum.AccessibilitySystems.html "enum bevy::a11y::AccessibilitySystems"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/main_schedule.rs.html#199)

### impl [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet") for [AnimationSystems](../../app/struct.AnimationSystems.html "struct bevy::app::AnimationSystems")

where [AnimationSystems](../../app/struct.AnimationSystems.html "struct bevy::app::AnimationSystems"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/set.rs.html#249)

### impl [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet") for [AnonymousSet](struct.AnonymousSet.html "struct bevy::ecs::schedule::AnonymousSet")

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/lib.rs.html#708)

### impl [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet") for [AssetEventSystems](../../asset/struct.AssetEventSystems.html "struct bevy::asset::AssetEventSystems")

where [AssetEventSystems](../../asset/struct.AssetEventSystems.html "struct bevy::asset::AssetEventSystems"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/erased_render_asset.rs.html#29)

### impl [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet") for bevy::render::erased\_render\_asset::[AssetExtractionSystems](../../render/erased_render_asset/struct.AssetExtractionSystems.html "struct bevy::render::erased_render_asset::AssetExtractionSystems")

where [AssetExtractionSystems](../../render/erased_render_asset/struct.AssetExtractionSystems.html "struct bevy::render::erased_render_asset::AssetExtractionSystems"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_asset.rs.html#28)

### impl [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet") for bevy::render::render\_asset::[AssetExtractionSystems](../../render/render_asset/struct.AssetExtractionSystems.html "struct bevy::render::render_asset::AssetExtractionSystems")

where [AssetExtractionSystems](../../render/render_asset/struct.AssetExtractionSystems.html "struct bevy::render::render_asset::AssetExtractionSystems"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/lib.rs.html#702)

### impl [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet") for [AssetTrackingSystems](../../asset/struct.AssetTrackingSystems.html "struct bevy::asset::AssetTrackingSystems")

where [AssetTrackingSystems](../../asset/struct.AssetTrackingSystems.html "struct bevy::asset::AssetTrackingSystems"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/lib.rs.html#43)

### impl [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet") for [CameraUpdateSystems](../../camera/struct.CameraUpdateSystems.html "struct bevy::camera::CameraUpdateSystems")

where [CameraUpdateSystems](../../camera/struct.CameraUpdateSystems.html "struct bevy::camera::CameraUpdateSystems"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/schedule.rs.html#85)

### impl [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet") for [Core2dSystems](../../core_pipeline/enum.Core2dSystems.html "enum bevy::core_pipeline::Core2dSystems")

where [Core2dSystems](../../core_pipeline/enum.Core2dSystems.html "enum bevy::core_pipeline::Core2dSystems"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/schedule.rs.html#46)

### impl [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet") for [Core3dSystems](../../core_pipeline/enum.Core3dSystems.html "enum bevy::core_pipeline::Core3dSystems")

where [Core3dSystems](../../core_pipeline/enum.Core3dSystems.html "enum bevy::core_pipeline::Core3dSystems"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/diagnostics_overlay.rs.html#239)

### impl [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet") for [DiagnosticsOverlaySystems](../../dev_tools/diagnostics_overlay/enum.DiagnosticsOverlaySystems.html "enum bevy::dev_tools::diagnostics_overlay::DiagnosticsOverlaySystems")

where [DiagnosticsOverlaySystems](../../dev_tools/diagnostics_overlay/enum.DiagnosticsOverlaySystems.html "enum bevy::dev_tools::diagnostics_overlay::DiagnosticsOverlaySystems"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/camera.rs.html#1086)

### impl [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet") for [DirtySpecializationSystems](../../render/camera/enum.DirtySpecializationSystems.html "enum bevy::render::camera::DirtySpecializationSystems")

where [DirtySpecializationSystems](../../render/camera/enum.DirtySpecializationSystems.html "enum bevy::render::camera::DirtySpecializationSystems"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/lib.rs.html#108)

### impl [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet") for [EditableTextSystems](../../text/struct.EditableTextSystems.html "struct bevy::text::EditableTextSystems")

where [EditableTextSystems](../../text/struct.EditableTextSystems.html "struct bevy::text::EditableTextSystems"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/system.rs.html#8)

### impl [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet") for [ExitSystems](../../window/struct.ExitSystems.html "struct bevy::window::ExitSystems")

where [ExitSystems](../../window/struct.ExitSystems.html "struct bevy::window::ExitSystems"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/fps_overlay.rs.html#60)

### impl [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet") for [FpsOverlaySystems](../../dev_tools/fps_overlay/enum.FpsOverlaySystems.html "enum bevy::dev_tools::fps_overlay::FpsOverlaySystems")

where [FpsOverlaySystems](../../dev_tools/fps_overlay/enum.FpsOverlaySystems.html "enum bevy::dev_tools::fps_overlay::FpsOverlaySystems"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/frustum.rs.html#57)

### impl [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet") for [FrustumGizmoSystems](../../gizmos/frustum/struct.FrustumGizmoSystems.html "struct bevy::gizmos::frustum::FrustumGizmoSystems")

where [FrustumGizmoSystems](../../gizmos/frustum/struct.FrustumGizmoSystems.html "struct bevy::gizmos::frustum::FrustumGizmoSystems"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/lib.rs.html#283)

### impl [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet") for [GizmoMeshSystems](../../gizmos/struct.GizmoMeshSystems.html "struct bevy::gizmos::GizmoMeshSystems")

where [GizmoMeshSystems](../../gizmos/struct.GizmoMeshSystems.html "struct bevy::gizmos::GizmoMeshSystems"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_gizmos_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos_render/lib.rs.html#10)

### impl [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet") for [GizmoRenderSystems](../../gizmos_render/enum.GizmoRenderSystems.html "enum bevy::gizmos_render::GizmoRenderSystems")

where [GizmoRenderSystems](../../gizmos_render/enum.GizmoRenderSystems.html "enum bevy::gizmos_render::GizmoRenderSystems"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/text_input.rs.html#461)

### impl [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet") for [ImeSystems](../../ui_widgets/enum.ImeSystems.html "enum bevy::ui_widgets::ImeSystems")

where [ImeSystems](../../ui_widgets/enum.ImeSystems.html "enum bevy::ui_widgets::ImeSystems"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/lib.rs.html#314)

### impl [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet") for [InputFocusSystems](../../input_focus/enum.InputFocusSystems.html "enum bevy::input_focus::InputFocusSystems")

where [InputFocusSystems](../../input_focus/enum.InputFocusSystems.html "enum bevy::input_focus::InputFocusSystems"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/lib.rs.html#105)

### impl [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet") for [InputSystems](../../input/struct.InputSystems.html "struct bevy::input::InputSystems")

where [InputSystems](../../input/struct.InputSystems.html "struct bevy::input::InputSystems"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/set.rs.html#63-176)

### impl [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet") for [Interned](../intern/struct.Interned.html "struct bevy::ecs::intern::Interned")<dyn [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet")\>

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#609)

### impl [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet") for [MaterialExtractionSystems](../../pbr/struct.MaterialExtractionSystems.html "struct bevy::pbr::MaterialExtractionSystems")

where [MaterialExtractionSystems](../../pbr/struct.MaterialExtractionSystems.html "struct bevy::pbr::MaterialExtractionSystems"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#1666)

### impl [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet") for [MeshExtractionSystems](../../pbr/struct.MeshExtractionSystems.html "struct bevy::pbr::MeshExtractionSystems")

where [MeshExtractionSystems](../../pbr/struct.MeshExtractionSystems.html "struct bevy::pbr::MeshExtractionSystems"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#144)

### impl [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet") for [MeshPipelineSystems](../../pbr/struct.MeshPipelineSystems.html "struct bevy::pbr::MeshPipelineSystems")

where [MeshPipelineSystems](../../pbr/struct.MeshPipelineSystems.html "struct bevy::pbr::MeshPipelineSystems"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/lib.rs.html#257)

### impl [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet") for [PickingSystems](../../picking/enum.PickingSystems.html "enum bevy::picking::PickingSystems")

where [PickingSystems](../../picking/enum.PickingSystems.html "enum bevy::picking::PickingSystems"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_remote/0.19.0/x86_64-unknown-linux-gnu/src/bevy_remote/lib.rs.html#916)

### impl [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet") for [RemoteSystems](../../remote/enum.RemoteSystems.html "enum bevy::remote::RemoteSystems")

where [RemoteSystems](../../remote/enum.RemoteSystems.html "enum bevy::remote::RemoteSystems"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/renderer/mod.rs.html#54)

### impl [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet") for [RenderGraphSystems](../../render/renderer/enum.RenderGraphSystems.html "enum bevy::render::renderer::RenderGraphSystems")

where [RenderGraphSystems](../../render/renderer/enum.RenderGraphSystems.html "enum bevy::render::renderer::RenderGraphSystems"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/lib.rs.html#154)

### impl [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet") for [RenderSystems](../../render/enum.RenderSystems.html "enum bevy::render::RenderSystems")

where [RenderSystems](../../render/enum.RenderSystems.html "enum bevy::render::RenderSystems"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/lib.rs.html#122)

### impl [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet") for [RenderUiSystems](../../ui_render/enum.RenderUiSystems.html "enum bevy::ui_render::RenderUiSystems")

where [RenderUiSystems](../../ui_render/enum.RenderUiSystems.html "enum bevy::ui_render::RenderUiSystems"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_gilrs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gilrs/lib.rs.html#87)

### impl [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet") for [RumbleSystems](../../prelude/struct.RumbleSystems.html "struct bevy::prelude::RumbleSystems")

where [RumbleSystems](../../prelude/struct.RumbleSystems.html "struct bevy::prelude::RumbleSystems"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/main_schedule.rs.html#412)

### impl [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet") for [RunFixedMainLoopSystems](../../prelude/enum.RunFixedMainLoopSystems.html "enum bevy::prelude::RunFixedMainLoopSystems")

where [RunFixedMainLoopSystems](../../prelude/enum.RunFixedMainLoopSystems.html "enum bevy::prelude::RunFixedMainLoopSystems"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/main_schedule.rs.html#203)

### impl [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet") for [SceneSpawnerSystems](../../app/enum.SceneSpawnerSystems.html "enum bevy::app::SceneSpawnerSystems")

where [SceneSpawnerSystems](../../app/enum.SceneSpawnerSystems.html "enum bevy::app::SceneSpawnerSystems"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/schedule_data/plugin.rs.html#75)

### impl [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet") for [SerializeSchedulesSystems](../../dev_tools/schedule_data/plugin/struct.SerializeSchedulesSystems.html "struct bevy::dev_tools::schedule_data::plugin::SerializeSchedulesSystems")

where [SerializeSchedulesSystems](../../dev_tools/schedule_data/plugin/struct.SerializeSchedulesSystems.html "struct bevy::dev_tools::schedule_data::plugin::SerializeSchedulesSystems"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/lib.rs.html#315)

### impl [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet") for [SimulationLightSystems](../../light/enum.SimulationLightSystems.html "enum bevy::light::SimulationLightSystems")

where [SimulationLightSystems](../../light/enum.SimulationLightSystems.html "enum bevy::light::SimulationLightSystems"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/lib.rs.html#71)

### impl [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet") for bevy::sprite::[SpriteSystems](../../sprite/enum.SpriteSystems.html "enum bevy::sprite::SpriteSystems")

where [SpriteSystems](../../sprite/enum.SpriteSystems.html "enum bevy::sprite::SpriteSystems"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/lib.rs.html#57)

### impl [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet") for bevy::sprite\_render::[SpriteSystems](../../sprite_render/enum.SpriteSystems.html "enum bevy::sprite_render::SpriteSystems")

where [SpriteSystems](../../sprite_render/enum.SpriteSystems.html "enum bevy::sprite_render::SpriteSystems"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state/transitions.rs.html#80)

### impl [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet") for [StateTransitionSystems](../../state/state/enum.StateTransitionSystems.html "enum bevy::state::state::StateTransitionSystems")

where [StateTransitionSystems](../../state/state/enum.StateTransitionSystems.html "enum bevy::state::state::StateTransitionSystems"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/lib.rs.html#104)

### impl [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet") for [Text2dUpdateSystems](../../text/struct.Text2dUpdateSystems.html "struct bevy::text::Text2dUpdateSystems")

where [Text2dUpdateSystems](../../text/struct.Text2dUpdateSystems.html "struct bevy::text::Text2dUpdateSystems"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/lib.rs.html#63)

### impl [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet") for [TimeSystems](../../time/struct.TimeSystems.html "struct bevy::time::TimeSystems")

where [TimeSystems](../../time/struct.TimeSystems.html "struct bevy::time::TimeSystems"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/transform_gizmo.rs.html#205)

### impl [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet") for [TransformGizmoSystems](../../prelude/struct.TransformGizmoSystems.html "struct bevy::prelude::TransformGizmoSystems")

where [TransformGizmoSystems](../../prelude/struct.TransformGizmoSystems.html "struct bevy::prelude::TransformGizmoSystems"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/plugins.rs.html#12)

### impl [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet") for [TransformSystems](../../prelude/enum.TransformSystems.html "enum bevy::prelude::TransformSystems")

where [TransformSystems](../../prelude/enum.TransformSystems.html "enum bevy::prelude::TransformSystems"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/lib.rs.html#94)

### impl [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet") for [UiSystems](../../ui/enum.UiSystems.html "enum bevy::ui::UiSystems")

where [UiSystems](../../ui/enum.UiSystems.html "enum bevy::ui::UiSystems"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/hierarchy.rs.html#59)

### impl [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet") for [ValidateParentHasComponentSystems](../../app/struct.ValidateParentHasComponentSystems.html "struct bevy::app::ValidateParentHasComponentSystems")

where [ValidateParentHasComponentSystems](../../app/struct.ValidateParentHasComponentSystems.html "struct bevy::app::ValidateParentHasComponentSystems"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/mod.rs.html#468)

### impl [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet") for [VisibilitySystems](../../camera/visibility/enum.VisibilitySystems.html "enum bevy::camera::visibility::VisibilitySystems")

where [VisibilitySystems](../../camera/visibility/enum.VisibilitySystems.html "enum bevy::camera::visibility::VisibilitySystems"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/propagate.rs.html#87)

### impl<C> [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet") for [PropagateSet](../../app/struct.PropagateSet.html "struct bevy::app::PropagateSet")<C>

where C: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq"), [PropagateSet](../../app/struct.PropagateSet.html "struct bevy::app::PropagateSet")<C>: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state/transitions.rs.html#113)

### impl<S> [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet") for [EnterSchedules](../../prelude/struct.EnterSchedules.html "struct bevy::prelude::EnterSchedules")<S>

where S: [States](../../prelude/trait.States.html "trait bevy::prelude::States"), [EnterSchedules](../../prelude/struct.EnterSchedules.html "struct bevy::prelude::EnterSchedules")<S>: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state/transitions.rs.html#93)

### impl<S> [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet") for [ExitSchedules](../../prelude/struct.ExitSchedules.html "struct bevy::prelude::ExitSchedules")<S>

where S: [States](../../prelude/trait.States.html "trait bevy::prelude::States"), [ExitSchedules](../../prelude/struct.ExitSchedules.html "struct bevy::prelude::ExitSchedules")<S>: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state/transitions.rs.html#103)

### impl<S> [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet") for [TransitionSchedules](../../prelude/struct.TransitionSchedules.html "struct bevy::prelude::TransitionSchedules")<S>

where S: [States](../../prelude/trait.States.html "trait bevy::prelude::States"), [TransitionSchedules](../../prelude/struct.TransitionSchedules.html "struct bevy::prelude::TransitionSchedules")<S>: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/set.rs.html#227)

### impl<T> [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet") for [SystemTypeSet](struct.SystemTypeSet.html "struct bevy::ecs::schedule::SystemTypeSet")<T>