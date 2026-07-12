[bevy](../../index.html)::[ecs](../index.html)

# Module change\_detection 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lib.rs.html#32)

Types that detect when their internal data mutate.

## Structs

[CheckChangeTicks](struct.CheckChangeTicks.html "struct bevy::ecs::change_detection::CheckChangeTicks")

An [`Event`](../../prelude/derive.Event.html "derive bevy::prelude::Event") that can be used to maintain [`Tick`](struct.Tick.html "struct bevy::ecs::change_detection::Tick")s in custom data structures, enabling to make use of bevy’s periodic checks that clamps ticks to a certain range, preventing overflows and thus keeping methods like [`Tick::is_newer_than`](struct.Tick.html#method.is_newer_than "method bevy::ecs::change_detection::Tick::is_newer_than") reliably return `false` for ticks that got too old.

[ComponentTickCells](struct.ComponentTickCells.html "struct bevy::ecs::change_detection::ComponentTickCells")

Interior-mutable access to the [`Tick`](struct.Tick.html "struct bevy::ecs::change_detection::Tick")s of a single component or resource.

[ComponentTicks](struct.ComponentTicks.html "struct bevy::ecs::change_detection::ComponentTicks")

Records when a component or resource was added and when it was last mutably dereferenced (or added).

[ContiguousComponentTicksMut](struct.ContiguousComponentTicksMut.html "struct bevy::ecs::change_detection::ContiguousComponentTicksMut")

Data type storing contiguously lying ticks, which may be accessed to mutate.

[ContiguousComponentTicksRef](struct.ContiguousComponentTicksRef.html "struct bevy::ecs::change_detection::ContiguousComponentTicksRef")

Data type storing contiguously lying ticks.

[ContiguousMut](struct.ContiguousMut.html "struct bevy::ecs::change_detection::ContiguousMut")

Data type returned by [`ContiguousQueryData::fetch_contiguous`](../query/trait.ContiguousQueryData.html#tymethod.fetch_contiguous "associated function bevy::ecs::query::ContiguousQueryData::fetch_contiguous") for [`Mut<T>`](../../prelude/struct.Mut.html "struct bevy::prelude::Mut") and `&mut T`

[ContiguousRef](struct.ContiguousRef.html "struct bevy::ecs::change_detection::ContiguousRef")

Contiguous equivalent of [`Ref<T>`](../../prelude/struct.Ref.html "struct bevy::prelude::Ref").

[MaybeLocation](struct.MaybeLocation.html "struct bevy::ecs::change_detection::MaybeLocation")

A value that contains a `T` if the `track_location` feature is enabled, and is a ZST if it is not.

[Mut](struct.Mut.html "struct bevy::ecs::change_detection::Mut")

Unique mutable borrow of an entity’s component or of a resource.

[MutUntyped](struct.MutUntyped.html "struct bevy::ecs::change_detection::MutUntyped")

Unique mutable borrow of resources or an entity’s component.

[NonSend](struct.NonSend.html "struct bevy::ecs::change_detection::NonSend")

Shared borrow of a non-[`Send`](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") resource.

[NonSendMut](struct.NonSendMut.html "struct bevy::ecs::change_detection::NonSendMut")

Unique borrow of a non-[`Send`](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") resource.

[Ref](struct.Ref.html "struct bevy::ecs::change_detection::Ref")

Shared borrow of an entity’s component with access to change detection. Similar to [`Mut`](../../prelude/struct.Mut.html "struct bevy::prelude::Mut") but is immutable and so doesn’t require unique access.

[Res](struct.Res.html "struct bevy::ecs::change_detection::Res")

Shared borrow of a [`Resource`](../../prelude/trait.Resource.html "trait bevy::prelude::Resource").

[ResMut](struct.ResMut.html "struct bevy::ecs::change_detection::ResMut")

Unique mutable borrow of a [`Resource`](../../prelude/trait.Resource.html "trait bevy::prelude::Resource").

[Tick](struct.Tick.html "struct bevy::ecs::change_detection::Tick")

A value that tracks when a system ran relative to other systems. This is used to power change detection.

## Constants

[CHECK\_TICK\_THRESHOLD](constant.CHECK_TICK_THRESHOLD.html "constant bevy::ecs::change_detection::CHECK_TICK_THRESHOLD")

The (arbitrarily chosen) minimum number of world tick increments between `check_tick` scans.

[MAX\_CHANGE\_AGE](constant.MAX_CHANGE_AGE.html "constant bevy::ecs::change_detection::MAX_CHANGE_AGE")

The maximum change tick difference that won’t overflow before the next `check_tick` scan.

## Traits

[DetectChanges](trait.DetectChanges.html "trait bevy::ecs::change_detection::DetectChanges")

Types that can read change detection information. This change detection is controlled by [`DetectChangesMut`](../../prelude/trait.DetectChangesMut.html "trait bevy::prelude::DetectChangesMut") types such as [`ResMut`](../../prelude/struct.ResMut.html "struct bevy::prelude::ResMut").

[DetectChangesMut](trait.DetectChangesMut.html "trait bevy::ecs::change_detection::DetectChangesMut")

Types that implement reliable change detection.