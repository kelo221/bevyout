[bevy](../../index.html)::[ecs](../index.html)::[change\_detection](index.html)

# Trait DetectChanges 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/traits.rs.html#27)

```rust
pub trait DetectChanges {
    // Required methods
    fn is_added(&self) -> bool;
    fn is_changed(&self) -> bool;
    fn is_added_after(&self, other: Tick) -> bool;
    fn is_changed_after(&self, other: Tick) -> bool;
    fn last_changed(&self) -> Tick;
    fn added(&self) -> Tick;
    fn changed_by(&self) -> MaybeLocation;
}
```

Types that can read change detection information. This change detection is controlled by [`DetectChangesMut`](../../prelude/trait.DetectChangesMut.html "trait bevy::prelude::DetectChangesMut") types such as [`ResMut`](../../prelude/struct.ResMut.html "struct bevy::prelude::ResMut").

### Example

Using types that implement [`DetectChanges`](../../prelude/trait.DetectChanges.html "trait bevy::prelude::DetectChanges"), such as [`Res`](../../prelude/struct.Res.html "struct bevy::prelude::Res"), provide a way to query if a value has been mutated in another system.

```rust
use bevy_ecs::prelude::*;

#[derive(Resource)]
struct MyResource(u32);

fn my_system(mut resource: Res<MyResource>) {
    if resource.is_changed() {
        println!("My component was mutated!");
    }
}
```

## Required Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/traits.rs.html#29)

#### fn [is\_added](#tymethod.is_added)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if this value was added after the system last ran.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/traits.rs.html#37)

#### fn [is\_changed](#tymethod.is_changed)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if this value was added or mutably dereferenced either since the last time the system ran or, if the system never ran, since the beginning of the program.

To check if the value was mutably dereferenced only, use `this.is_changed() && !this.is_added()`.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/traits.rs.html#40)

#### fn [is\_added\_after](#tymethod.is_added_after)(&self, other: [Tick](struct.Tick.html "struct bevy::ecs::change_detection::Tick")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if this value was added after the `other` tick.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/traits.rs.html#71)

#### fn [is\_changed\_after](#tymethod.is_changed_after)(&self, other: [Tick](struct.Tick.html "struct bevy::ecs::change_detection::Tick")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if this value was added or mutably dereferenced after the `other` tick.

##### Example

```rust
fn system(query: Query<(Ref<Source>, &mut Target)>) {
    for (source, mut target) in query {
        // Only convert the source to the target if the source is newer
        if source.is_changed_after(target.last_changed()) {
            *target = Target::from_source(&source);
        }
    }
}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/traits.rs.html#80)

#### fn [last\_changed](#tymethod.last_changed)(&self) -> [Tick](struct.Tick.html "struct bevy::ecs::change_detection::Tick")

Returns the change tick recording the time this data was most recently changed.

Note that components and resources are also marked as changed upon insertion.

For comparison, the previous change tick of a system can be read using the [`SystemChangeTick`](../system/struct.SystemChangeTick.html "struct bevy::ecs::system::SystemChangeTick") [`SystemParam`](../system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/traits.rs.html#83)

#### fn [added](#tymethod.added)(&self) -> [Tick](struct.Tick.html "struct bevy::ecs::change_detection::Tick")

Returns the change tick recording the time this data was added.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/traits.rs.html#86)

#### fn [changed\_by](#tymethod.changed_by)(&self) -> [MaybeLocation](struct.MaybeLocation.html "struct bevy::ecs::change_detection::MaybeLocation")

The location that last caused this to change.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/params.rs.html#1216)

### impl<'w, T> [DetectChanges](../../prelude/trait.DetectChanges.html "trait bevy::prelude::DetectChanges") for [Mut](../../prelude/struct.Mut.html "struct bevy::prelude::Mut")<'w, T>

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/params.rs.html#597)

### impl<'w, T> [DetectChanges](../../prelude/trait.DetectChanges.html "trait bevy::prelude::DetectChanges") for [NonSend](../../prelude/struct.NonSend.html "struct bevy::prelude::NonSend")<'w, T>

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/params.rs.html#625)

### impl<'w, T> [DetectChanges](../../prelude/trait.DetectChanges.html "trait bevy::prelude::DetectChanges") for [NonSendMut](../../prelude/struct.NonSendMut.html "struct bevy::prelude::NonSendMut")<'w, T>

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/params.rs.html#869)

### impl<'w, T> [DetectChanges](../../prelude/trait.DetectChanges.html "trait bevy::prelude::DetectChanges") for [Ref](../../prelude/struct.Ref.html "struct bevy::prelude::Ref")<'w, T>

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/params.rs.html#522)

### impl<'w, T> [DetectChanges](../../prelude/trait.DetectChanges.html "trait bevy::prelude::DetectChanges") for [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'w, T>

where T: [Resource](../../prelude/trait.Resource.html "trait bevy::prelude::Resource") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/params.rs.html#565)

### impl<'w, T> [DetectChanges](../../prelude/trait.DetectChanges.html "trait bevy::prelude::DetectChanges") for [ResMut](../../prelude/struct.ResMut.html "struct bevy::prelude::ResMut")<'w, T>

where T: [Resource](../../prelude/trait.Resource.html "trait bevy::prelude::Resource")<Mutability = [Mutable](../component/struct.Mutable.html "struct bevy::ecs::component::Mutable")\> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/params.rs.html#1323)

### impl<'w> [DetectChanges](../../prelude/trait.DetectChanges.html "trait bevy::prelude::DetectChanges") for [MutUntyped](struct.MutUntyped.html "struct bevy::ecs::change_detection::MutUntyped")<'w>