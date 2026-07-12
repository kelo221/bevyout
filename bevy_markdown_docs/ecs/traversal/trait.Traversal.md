[bevy](../../index.html)::[ecs](../index.html)::[traversal](index.html)

# Trait Traversal 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/traversal.rs.html#28-29)

```rust
pub trait Traversal<D>:
    ReadOnlyQueryData
    + ReleaseStateQueryData
    + SingleEntityQueryDatawhere
    D: ?Sized,{
    // Required method
    fn traverse(item: Self::Item<'_, '_>, data: &D) -> Option<Entity>;
}
```

A component that can point to another entity, and which can be used to define a path through the ECS.

Traversals are used to [specify the direction](../event/struct.PropagateEntityTrigger.html "struct bevy::ecs::event::PropagateEntityTrigger") of [event propagation](../../prelude/struct.On.html#method.propagate "method bevy::prelude::On::propagate") in [`EntityEvent`](../../prelude/trait.EntityEvent.html "trait bevy::prelude::EntityEvent") [observers](../../prelude/struct.Observer.html "struct bevy::prelude::Observer"). The default query is `()`.

Infinite loops are possible, and are not checked for. While looping can be desirable in some contexts (for example, an observer that triggers itself multiple times before stopping), following an infinite traversal loop without an eventual exit will cause your application to hang. Each implementer of `Traversal` is responsible for documenting possible looping behavior, and consumers of those implementations are responsible for avoiding infinite loops in their code.

Traversals may be parameterized with additional data. For example, in observer event propagation, the parameter `D` is the event type given in `On<E>`. This allows traversal to differ depending on event data.

## Required Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/traversal.rs.html#32)

#### fn [traverse](#tymethod.traverse)(item: Self::[Item](../query/trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'\_, '\_>, data: [&D](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\>

Returns the next entity to visit.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/traversal.rs.html#35)

### impl<D> [Traversal](trait.Traversal.html "trait bevy::ecs::traversal::Traversal")<D> for [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/traversal.rs.html#36)

#### fn [traverse](#tymethod.traverse)(\_: <[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) as [QueryData](../query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](../query/trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'\_, '\_>, \_data: [&D](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/traversal.rs.html#48)

### impl<R, D> [Traversal](trait.Traversal.html "trait bevy::ecs::traversal::Traversal")<D> for [&R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

where R: [Relationship](../relationship/trait.Relationship.html "trait bevy::ecs::relationship::Relationship"),

This provides generalized hierarchy traversal for use in [event propagation](../../prelude/struct.On.html#method.propagate "method bevy::prelude::On::propagate").

#### Warning

Traversing in a loop could result in infinite loops for relationship graphs with loops.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/traversal.rs.html#49)

#### fn [traverse](#tymethod.traverse)(item: <[&R](https://doc.rust-lang.org/nightly/std/primitive.reference.html) as [QueryData](../query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](../query/trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'\_, '\_>, \_data: [&D](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\>

## Implementors

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/lib.rs.html#244)

### impl [Traversal](trait.Traversal.html "trait bevy::ecs::traversal::Traversal")<[AcquireFocus](../../input_focus/struct.AcquireFocus.html "struct bevy::input_focus::AcquireFocus")\> for [WindowTraversal](../../input_focus/struct.WindowTraversal.html "struct bevy::input_focus::WindowTraversal")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#100-102)

### impl<E> [Traversal](trait.Traversal.html "trait bevy::ecs::traversal::Traversal")<[Pointer](../../prelude/struct.Pointer.html "struct bevy::prelude::Pointer")<E>> for [PointerTraversal](../../prelude/struct.PointerTraversal.html "struct bevy::prelude::PointerTraversal")

where E: [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect"),

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/lib.rs.html#226)

### impl<M> [Traversal](trait.Traversal.html "trait bevy::ecs::traversal::Traversal")<[FocusedInput](../../input_focus/struct.FocusedInput.html "struct bevy::input_focus::FocusedInput")<M>> for [WindowTraversal](../../input_focus/struct.WindowTraversal.html "struct bevy::input_focus::WindowTraversal")

where M: [Message](../../prelude/trait.Message.html "trait bevy::prelude::Message") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),