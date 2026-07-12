[bevy](../../index.html)::[ecs](../index.html)::[observer](index.html)

# Trait IntoEntityObserver 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/observer/distributed_storage.rs.html#596)

```rust
pub trait IntoEntityObserver<Marker>: Send + 'static {
    // Required method
    fn into_observer_for_entity(self, entity: Entity) -> Observer;
}
```

Trait for types that can be converted into an entity-targeting [`Observer`](../../prelude/struct.Observer.html "struct bevy::prelude::Observer").

This trait enforces that the event type implements [`EntityEvent`](../../prelude/trait.EntityEvent.html "trait bevy::prelude::EntityEvent").

## Required Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/observer/distributed_storage.rs.html#598)

#### fn [into\_observer\_for\_entity](#tymethod.into_observer_for_entity)(self, entity: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")) -> [Observer](../../prelude/struct.Observer.html "struct bevy::prelude::Observer")

Converts this type into an [`Observer`](../../prelude/struct.Observer.html "struct bevy::prelude::Observer") that watches the given entity.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/observer/distributed_storage.rs.html#601-602)

### impl<E, B, M, T> [IntoEntityObserver](trait.IntoEntityObserver.html "trait bevy::ecs::observer::IntoEntityObserver")<[(E, B, M)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\> for T

where E: [EntityEvent](../../prelude/trait.EntityEvent.html "trait bevy::prelude::EntityEvent"), B: [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"), T: [IntoObserverSystem](../system/trait.IntoObserverSystem.html "trait bevy::ecs::system::IntoObserverSystem")<E, B, M>,