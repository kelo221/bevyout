[bevy](../../index.html)::[ecs](../index.html)::[bundle](index.html)

# Trait DynamicBundle 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/mod.rs.html#249)

```rust
pub trait DynamicBundle: Sized {
    type Effect;

    // Required methods
    unsafe fn get_components(
        ptr: MovingPtr<'_, Self>,
        func: &mut impl FnMut(StorageType, OwningPtr<'_>),
    );
    unsafe fn apply_effect(
        ptr: MovingPtr<'_, MaybeUninit<Self>>,
        entity: &mut EntityWorldMut<'_>,
    );
}
```

The parts from [`Bundle`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") that don’t require statically knowing the components of the bundle.

## Required Associated Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/mod.rs.html#251)

#### type [Effect](#associatedtype.Effect)

An operation on the entity that happens _after_ inserting this bundle.

## Required Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/mod.rs.html#272-275)

#### unsafe fn [get\_components](#tymethod.get_components)( ptr: [MovingPtr](../ptr/struct.MovingPtr.html "struct bevy::ecs::ptr::MovingPtr")<'\_, Self>, func: &mut impl [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([StorageType](../component/enum.StorageType.html "enum bevy::ecs::component::StorageType"), [OwningPtr](../ptr/struct.OwningPtr.html "struct bevy::ecs::ptr::OwningPtr")<'\_>), )

Moves the components out of the bundle.

##### Safety

For callers:

*   Must be called exactly once before `apply_effect`
*   The `StorageType` argument passed into `func` must be correct for the component being fetched.
*   `apply_effect` must be called exactly once after this has been called if `Effect: !NoBundleEffect`

For implementors:

*   Implementors of this function must convert `ptr` into pointers to individual components stored within `Self` and call `func` on each of them in exactly the same order as [`Bundle::get_component_ids`](../../prelude/trait.Bundle.html#tymethod.get_component_ids "associated function bevy::prelude::Bundle::get_component_ids") and [`BundleFromComponents::from_components`](trait.BundleFromComponents.html#tymethod.from_components "associated function bevy::ecs::bundle::BundleFromComponents::from_components").
*   If any part of `ptr` is to be accessed in `apply_effect`, it must _not_ be dropped at any point in this function. Calling [`bevy_ptr::deconstruct_moving_ptr`](../ptr/macro.deconstruct_moving_ptr.html "macro bevy::ecs::ptr::deconstruct_moving_ptr") in this function automatically ensures this.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/mod.rs.html#299)

#### unsafe fn [apply\_effect](#tymethod.apply_effect)( ptr: [MovingPtr](../ptr/struct.MovingPtr.html "struct bevy::ecs::ptr::MovingPtr")<'\_, [MaybeUninit](https://doc.rust-lang.org/nightly/core/mem/maybe_uninit/union.MaybeUninit.html "union core::mem::maybe_uninit::MaybeUninit")<Self>>, entity: &mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'\_>, )

Applies the after-effects of spawning this bundle.

This is applied after all residual changes to the [`World`](../../prelude/struct.World.html "struct bevy::prelude::World"), including flushing the internal command queue.

##### Safety

For callers:

*   Must be called exactly once after `get_components` has been called.
*   `ptr` must point to the instance of `Self` that `get_components` was called on, all of fields that were moved out of in `get_components` will not be valid anymore.

For implementors:

*   If any part of `ptr` is to be accessed in this function, it must _not_ be dropped at any point in `get_components`. Calling [`bevy_ptr::deconstruct_moving_ptr`](../ptr/macro.deconstruct_moving_ptr.html "macro bevy::ecs::ptr::deconstruct_moving_ptr") in `get_components` automatically ensures this is the case.
*   Note that `entity` may already have been despawned by hooks or observers at this point, so check [`EntityWorldMut::is_spawned`](../../prelude/struct.EntityWorldMut.html#method.is_spawned "method bevy::prelude::EntityWorldMut::is_spawned") before trusting it.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#176-183)

### impl [DynamicBundle](trait.DynamicBundle.html "trait bevy::ecs::bundle::DynamicBundle") for [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#176-183)

#### type [Effect](#associatedtype.Effect) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#176-183)

#### unsafe fn [get\_components](#tymethod.get_components)( ptr: [MovingPtr](../ptr/struct.MovingPtr.html "struct bevy::ecs::ptr::MovingPtr")<'\_, [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\>, func: &mut impl [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([StorageType](../component/enum.StorageType.html "enum bevy::ecs::component::StorageType"), [OwningPtr](../ptr/struct.OwningPtr.html "struct bevy::ecs::ptr::OwningPtr")<'\_>), )

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#176-183)

#### unsafe fn [apply\_effect](#tymethod.apply_effect)( ptr: [MovingPtr](../ptr/struct.MovingPtr.html "struct bevy::ecs::ptr::MovingPtr")<'\_, [MaybeUninit](https://doc.rust-lang.org/nightly/core/mem/maybe_uninit/union.MaybeUninit.html "union core::mem::maybe_uninit::MaybeUninit")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\>>, entity: &mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'\_>, )

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#176-183)

### impl<B> [DynamicBundle](trait.DynamicBundle.html "trait bevy::ecs::bundle::DynamicBundle") for [(B₁, B₂, …, Bₙ)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html#trait-implementations-1)

where B: [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"),

This trait is implemented for tuples up to 16 items long.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#176-183)

#### type [Effect](#associatedtype.Effect) = (<B as [DynamicBundle](trait.DynamicBundle.html "trait bevy::ecs::bundle::DynamicBundle")\>::[Effect](trait.DynamicBundle.html#associatedtype.Effect "type bevy::ecs::bundle::DynamicBundle::Effect"),)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#176-183)

#### unsafe fn [get\_components](#tymethod.get_components)( ptr: [MovingPtr](../ptr/struct.MovingPtr.html "struct bevy::ecs::ptr::MovingPtr")<'\_, [(B,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>, func: &mut impl [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([StorageType](../component/enum.StorageType.html "enum bevy::ecs::component::StorageType"), [OwningPtr](../ptr/struct.OwningPtr.html "struct bevy::ecs::ptr::OwningPtr")<'\_>), )

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#176-183)

#### unsafe fn [apply\_effect](#tymethod.apply_effect)( ptr: [MovingPtr](../ptr/struct.MovingPtr.html "struct bevy::ecs::ptr::MovingPtr")<'\_, [MaybeUninit](https://doc.rust-lang.org/nightly/core/mem/maybe_uninit/union.MaybeUninit.html "union core::mem::maybe_uninit::MaybeUninit")<[(B,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>>, entity: &mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'\_>, )

## Implementors

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#43)

### impl<C> [DynamicBundle](trait.DynamicBundle.html "trait bevy::ecs::bundle::DynamicBundle") for C

where C: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#44)

#### type [Effect](#associatedtype.Effect) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/observe.rs.html#44-45)

### impl<E, B, M, I> [DynamicBundle](trait.DynamicBundle.html "trait bevy::ecs::bundle::DynamicBundle") for [AddObserver](../../ui_widgets/struct.AddObserver.html "struct bevy::ui_widgets::AddObserver")<E, B, M, I>

where E: [EntityEvent](../../prelude/trait.EntityEvent.html "trait bevy::prelude::EntityEvent"), B: [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"), I: [IntoObserverSystem](../system/trait.IntoObserverSystem.html "trait bevy::ecs::system::IntoObserverSystem")<E, B, M>,

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/observe.rs.html#47)

#### type [Effect](#associatedtype.Effect) = [AddObserver](../../ui_widgets/struct.AddObserver.html "struct bevy::ui_widgets::AddObserver")<E, B, M, I>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/spawn.rs.html#364)

### impl<R, B> [DynamicBundle](trait.DynamicBundle.html "trait bevy::ecs::bundle::DynamicBundle") for [SpawnOneRelated](../spawn/struct.SpawnOneRelated.html "struct bevy::ecs::spawn::SpawnOneRelated")<R, B>

where R: [Relationship](../relationship/trait.Relationship.html "trait bevy::ecs::relationship::Relationship"), B: [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/spawn.rs.html#365)

#### type [Effect](#associatedtype.Effect) = [SpawnOneRelated](../spawn/struct.SpawnOneRelated.html "struct bevy::ecs::spawn::SpawnOneRelated")<R, B>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/spawn.rs.html#314)

### impl<R, L> [DynamicBundle](trait.DynamicBundle.html "trait bevy::ecs::bundle::DynamicBundle") for [SpawnRelatedBundle](../spawn/struct.SpawnRelatedBundle.html "struct bevy::ecs::spawn::SpawnRelatedBundle")<R, L>

where R: [Relationship](../relationship/trait.Relationship.html "trait bevy::ecs::relationship::Relationship"), L: [SpawnableList](../spawn/trait.SpawnableList.html "trait bevy::ecs::spawn::SpawnableList")<R>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/spawn.rs.html#315)

#### type [Effect](#associatedtype.Effect) = [SpawnRelatedBundle](../spawn/struct.SpawnRelatedBundle.html "struct bevy::ecs::spawn::SpawnRelatedBundle")<R, L>