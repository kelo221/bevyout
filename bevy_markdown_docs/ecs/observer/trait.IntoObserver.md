[bevy](../../index.html)::[ecs](../index.html)::[observer](index.html)

# Trait IntoObserver 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/observer/distributed_storage.rs.html#561)

```rust
pub trait IntoObserver<Marker>: Send + 'static {
    // Required method
    fn into_observer(self) -> Observer;
}
```

Trait for types that can be converted into an [`Observer`](../../prelude/struct.Observer.html "struct bevy::prelude::Observer").

## Required Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/observer/distributed_storage.rs.html#563)

#### fn [into\_observer](#tymethod.into_observer)(self) -> [Observer](../../prelude/struct.Observer.html "struct bevy::prelude::Observer")

Converts this type into an [`Observer`](../../prelude/struct.Observer.html "struct bevy::prelude::Observer").

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/observer/distributed_storage.rs.html#566)

### impl [IntoObserver](trait.IntoObserver.html "trait bevy::ecs::observer::IntoObserver")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\> for [Observer](../../prelude/struct.Observer.html "struct bevy::prelude::Observer")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/observer/distributed_storage.rs.html#572)

### impl<E, B, M, T> [IntoObserver](trait.IntoObserver.html "trait bevy::ecs::observer::IntoObserver")<[(E, B, M)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\> for T

where E: [Event](../../prelude/trait.Event.html "trait bevy::prelude::Event"), B: [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"), T: [IntoObserverSystem](../system/trait.IntoObserverSystem.html "trait bevy::ecs::system::IntoObserverSystem")<E, B, M>,