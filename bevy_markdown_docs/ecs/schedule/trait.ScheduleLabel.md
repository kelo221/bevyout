[bevy](../../index.html)::[ecs](../index.html)::[schedule](index.html)

# Trait ScheduleLabel 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/set.rs.html#22-61)

```rust
pub trait ScheduleLabel:
    Send
    + Sync
    + Debug
    + DynEq
    + DynHash {
    // Required method
    fn dyn_clone(&self) -> Box<dyn ScheduleLabel>;

    // Provided method
    fn intern(&self) -> Interned<dyn ScheduleLabel>
       where Self: Sized { ... }
}
```

A strongly-typed class of labels used to identify a [`Schedule`](../../prelude/struct.Schedule.html "struct bevy::prelude::Schedule").

Each schedule in a [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") has a unique schedule label value, and schedules can be automatically created from labels via [`Schedules::add_systems()`](../../prelude/struct.Schedules.html#method.add_systems "method bevy::prelude::Schedules::add_systems").

## Defining new schedule labels

By default, you should use Bevy’s premade schedule labels which implement this trait. If you are using [`bevy_ecs`](../index.html "mod bevy::ecs") directly or if you need to run a group of systems outside the existing schedules, you may define your own schedule labels by using `#[derive(ScheduleLabel)]`.

```rust
use bevy_ecs::prelude::*;
use bevy_ecs::schedule::ScheduleLabel;

// Declare a new schedule label.
#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash, Default)]
struct Update;

let mut world = World::new();

// Add a system to the schedule with that label (creating it automatically).
fn a_system_function() {}
world.get_resource_or_init::<Schedules>().add_systems(Update, a_system_function);

// Run the schedule, and therefore run the system.
world.run_schedule(Update);
```

## Required Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/set.rs.html#22-61)

#### fn [dyn\_clone](#tymethod.dyn_clone)(&self) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [ScheduleLabel](trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel")\>

Clones this `ScheduleLabel`.

## Provided Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/set.rs.html#22-61)

#### fn [intern](#method.intern)(&self) -> [Interned](../intern/struct.Interned.html "struct bevy::ecs::intern::Interned")<dyn [ScheduleLabel](trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel")\>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Returns an [`Interned`](../intern/struct.Interned.html "struct bevy::ecs::intern::Interned") value corresponding to `self`.

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

examples/showcase/stepping.rs ([line 23](../../../src/breakout/stepping.rs.html#23))

```rust
22    pub fn add_schedule(mut self, label: impl ScheduleLabel) -> SteppingPlugin {
23        self.schedule_labels.push(label.intern());
24        self
25    }
```

## Trait Implementations

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/set.rs.html#22-61)

### impl [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") for dyn [ScheduleLabel](trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/set.rs.html#22-61)

### impl [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") for dyn [ScheduleLabel](trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/set.rs.html#22-61)

#### fn [hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)<H>(&self, state: [&mut H](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

where H: [Hasher](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"),

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/set.rs.html#22-61)

### impl [Internable](../intern/trait.Internable.html "trait bevy::ecs::intern::Internable") for dyn [ScheduleLabel](trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/set.rs.html#22-61)

#### fn [leak](../intern/trait.Internable.html#tymethod.leak)(&self) -> &'static dyn [ScheduleLabel](trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel")

Creates a static reference to `self`, possibly leaking memory.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/set.rs.html#22-61)

#### fn [ref\_eq](../intern/trait.Internable.html#tymethod.ref_eq)(&self, other: &(dyn [ScheduleLabel](trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel") + 'static)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if the two references point to the same value.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/set.rs.html#22-61)

#### fn [ref\_hash](../intern/trait.Internable.html#tymethod.ref_hash)<H>(&self, state: [&mut H](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

where H: [Hasher](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"),

Feeds the reference to the hasher.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/set.rs.html#22-61)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") for dyn [ScheduleLabel](trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/set.rs.html#22-61)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &(dyn [ScheduleLabel](trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel") + 'static)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/schedule.rs.html#73)

### impl [ScheduleLabel](trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel") for [Core2d](../../core_pipeline/struct.Core2d.html "struct bevy::core_pipeline::Core2d")

where [Core2d](../../core_pipeline/struct.Core2d.html "struct bevy::core_pipeline::Core2d"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/schedule.rs.html#34)

### impl [ScheduleLabel](trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel") for [Core3d](../../core_pipeline/struct.Core3d.html "struct bevy::core_pipeline::Core3d")

where [Core3d](../../core_pipeline/struct.Core3d.html "struct bevy::core_pipeline::Core3d"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/extract_plugin.rs.html#86)

### impl [ScheduleLabel](trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel") for [ExtractSchedule](../../prelude/struct.ExtractSchedule.html "struct bevy::prelude::ExtractSchedule")

where [ExtractSchedule](../../prelude/struct.ExtractSchedule.html "struct bevy::prelude::ExtractSchedule"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/main_schedule.rs.html#80)

### impl [ScheduleLabel](trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel") for [First](../../prelude/struct.First.html "struct bevy::prelude::First")

where [First](../../prelude/struct.First.html "struct bevy::prelude::First"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/main_schedule.rs.html#110)

### impl [ScheduleLabel](trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel") for [FixedFirst](../../prelude/struct.FixedFirst.html "struct bevy::prelude::FixedFirst")

where [FixedFirst](../../prelude/struct.FixedFirst.html "struct bevy::prelude::FixedFirst"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/main_schedule.rs.html#147)

### impl [ScheduleLabel](trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel") for [FixedLast](../../prelude/struct.FixedLast.html "struct bevy::prelude::FixedLast")

where [FixedLast](../../prelude/struct.FixedLast.html "struct bevy::prelude::FixedLast"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/main_schedule.rs.html#159)

### impl [ScheduleLabel](trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel") for [FixedMain](../../app/struct.FixedMain.html "struct bevy::app::FixedMain")

where [FixedMain](../../app/struct.FixedMain.html "struct bevy::app::FixedMain"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/main_schedule.rs.html#140)

### impl [ScheduleLabel](trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel") for [FixedPostUpdate](../../prelude/struct.FixedPostUpdate.html "struct bevy::prelude::FixedPostUpdate")

where [FixedPostUpdate](../../prelude/struct.FixedPostUpdate.html "struct bevy::prelude::FixedPostUpdate"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/main_schedule.rs.html#117)

### impl [ScheduleLabel](trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel") for [FixedPreUpdate](../../prelude/struct.FixedPreUpdate.html "struct bevy::prelude::FixedPreUpdate")

where [FixedPreUpdate](../../prelude/struct.FixedPreUpdate.html "struct bevy::prelude::FixedPreUpdate"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/main_schedule.rs.html#132)

### impl [ScheduleLabel](trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel") for [FixedUpdate](../../prelude/struct.FixedUpdate.html "struct bevy::prelude::FixedUpdate")

where [FixedUpdate](../../prelude/struct.FixedUpdate.html "struct bevy::prelude::FixedUpdate"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/set.rs.html#22-61)

### impl [ScheduleLabel](trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel") for [Interned](../intern/struct.Interned.html "struct bevy::ecs::intern::Interned")<dyn [ScheduleLabel](trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel")\>

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/main_schedule.rs.html#195)

### impl [ScheduleLabel](trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel") for [Last](../../prelude/struct.Last.html "struct bevy::prelude::Last")

where [Last](../../prelude/struct.Last.html "struct bevy::prelude::Last"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/main_schedule.rs.html#56)

### impl [ScheduleLabel](trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel") for [Main](../../prelude/struct.Main.html "struct bevy::prelude::Main")

where [Main](../../prelude/struct.Main.html "struct bevy::prelude::Main"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/main_schedule.rs.html#74)

### impl [ScheduleLabel](trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel") for [PostStartup](../../prelude/struct.PostStartup.html "struct bevy::prelude::PostStartup")

where [PostStartup](../../prelude/struct.PostStartup.html "struct bevy::prelude::PostStartup"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/main_schedule.rs.html#189)

### impl [ScheduleLabel](trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel") for [PostUpdate](../../prelude/struct.PostUpdate.html "struct bevy::prelude::PostUpdate")

where [PostUpdate](../../prelude/struct.PostUpdate.html "struct bevy::prelude::PostUpdate"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/main_schedule.rs.html#62)

### impl [ScheduleLabel](trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel") for [PreStartup](../../prelude/struct.PreStartup.html "struct bevy::prelude::PreStartup")

where [PreStartup](../../prelude/struct.PreStartup.html "struct bevy::prelude::PreStartup"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/main_schedule.rs.html#91)

### impl [ScheduleLabel](trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel") for [PreUpdate](../../prelude/struct.PreUpdate.html "struct bevy::prelude::PreUpdate")

where [PreUpdate](../../prelude/struct.PreUpdate.html "struct bevy::prelude::PreUpdate"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_remote/0.19.0/x86_64-unknown-linux-gnu/src/bevy_remote/lib.rs.html#910)

### impl [ScheduleLabel](trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel") for [RemoteLast](../../remote/struct.RemoteLast.html "struct bevy::remote::RemoteLast")

where [RemoteLast](../../remote/struct.RemoteLast.html "struct bevy::remote::RemoteLast"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/lib.rs.html#287)

### impl [ScheduleLabel](trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel") for [Render](../../render/struct.Render.html "struct bevy::render::Render")

where [Render](../../render/struct.Render.html "struct bevy::render::Render"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/renderer/mod.rs.html#34)

### impl [ScheduleLabel](trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel") for [RenderGraph](../../prelude/struct.RenderGraph.html "struct bevy::prelude::RenderGraph")

where [RenderGraph](../../prelude/struct.RenderGraph.html "struct bevy::prelude::RenderGraph"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/lib.rs.html#214)

### impl [ScheduleLabel](trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel") for [RenderStartup](../../render/struct.RenderStartup.html "struct bevy::render::RenderStartup")

where [RenderStartup](../../render/struct.RenderStartup.html "struct bevy::render::RenderStartup"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/main_schedule.rs.html#103)

### impl [ScheduleLabel](trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel") for [RunFixedMainLoop](../../prelude/struct.RunFixedMainLoop.html "struct bevy::prelude::RunFixedMainLoop")

where [RunFixedMainLoop](../../prelude/struct.RunFixedMainLoop.html "struct bevy::prelude::RunFixedMainLoop"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/main_schedule.rs.html#178)

### impl [ScheduleLabel](trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel") for [SpawnScene](../../prelude/struct.SpawnScene.html "struct bevy::prelude::SpawnScene")

where [SpawnScene](../../prelude/struct.SpawnScene.html "struct bevy::prelude::SpawnScene"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/main_schedule.rs.html#68)

### impl [ScheduleLabel](trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel") for [Startup](../../prelude/struct.Startup.html "struct bevy::prelude::Startup")

where [Startup](../../prelude/struct.Startup.html "struct bevy::prelude::Startup"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state/transitions.rs.html#60)

### impl [ScheduleLabel](trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel") for [StateTransition](../../prelude/struct.StateTransition.html "struct bevy::prelude::StateTransition")

where [StateTransition](../../prelude/struct.StateTransition.html "struct bevy::prelude::StateTransition"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/main_schedule.rs.html#172)

### impl [ScheduleLabel](trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel") for [Update](../../prelude/struct.Update.html "struct bevy::prelude::Update")

where [Update](../../prelude/struct.Update.html "struct bevy::prelude::Update"): 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state/transitions.rs.html#18)

### impl<S> [ScheduleLabel](trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel") for [OnEnter](../../prelude/struct.OnEnter.html "struct bevy::prelude::OnEnter")<S>

where S: [States](../../prelude/trait.States.html "trait bevy::prelude::States"), [OnEnter](../../prelude/struct.OnEnter.html "struct bevy::prelude::OnEnter")<S>: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state/transitions.rs.html#24)

### impl<S> [ScheduleLabel](trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel") for [OnExit](../../prelude/struct.OnExit.html "struct bevy::prelude::OnExit")<S>

where S: [States](../../prelude/trait.States.html "trait bevy::prelude::States"), [OnExit](../../prelude/struct.OnExit.html "struct bevy::prelude::OnExit")<S>: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state/transitions.rs.html#33)

### impl<S> [ScheduleLabel](trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel") for [OnTransition](../../prelude/struct.OnTransition.html "struct bevy::prelude::OnTransition")<S>

where S: [States](../../prelude/trait.States.html "trait bevy::prelude::States"), [OnTransition](../../prelude/struct.OnTransition.html "struct bevy::prelude::OnTransition")<S>: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),