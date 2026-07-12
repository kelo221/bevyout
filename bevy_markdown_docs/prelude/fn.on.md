[bevy](../index.html)::[prelude](index.html)

# Function on 

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#554-556)

```rust
pub fn on<I, E, B, M>(observer: I) -> OnTemplate<I, E, B, M>where
    I: IntoObserverSystem<E, B, M>,
    E: EntityEvent,
    B: Bundle,
    M: 'static,
```

Returns an [`OnTemplate`](../scene/struct.OnTemplate.html "struct bevy::scene::OnTemplate") that will create an [`Observer`](struct.Observer.html "struct bevy::prelude::Observer") of a given [`EntityEvent`](trait.EntityEvent.html "trait bevy::prelude::EntityEvent") on the current [`Scene`](trait.Scene.html "trait bevy::prelude::Scene") entity.